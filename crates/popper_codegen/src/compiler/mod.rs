use std::collections::HashMap;
use std::env::var;
use popper_llvm::module::Module;
use popper_llvm::context::Context;
use popper_llvm::builder::Builder;
use popper_llvm::types::int_types::IntType;
use popper_llvm::types::TypeEnum;
use popper_llvm::value::{ToValue, Value, ValueEnum};

use popper_mir::mir_ast::{BodyFn, Const, Declare, Module as MirModule, Type, Value as MirValue};
use popper_mir::mir_ast::Ir;

use popper_common::hash::hash_file;
use crate::object::PopObject;

macro_rules! cmd {
    ($cmd:ident $($e:expr)* ) => {
        std::process::Command::new(stringify!($cmd))
            $(.arg($e))*
            .output()
            .expect(concat!("Failed to execute ", stringify!($cmd)))
    };
}

#[derive(Debug, Clone)]
pub struct Compiler {
    pub context: Context,
    pub module: Module,
    pub builder: Builder,
    pub mir_module: MirModule,
    pub cdylib_used: Vec<String>,
    pub env: HashMap<String, PopObject>
}

impl Compiler {
    pub fn new(mir_module: MirModule) -> Self {
        let mut context = Context::new();
        let module = context.new_module(mir_module.name.as_str());
        let builder = context.new_builder();
        Self {
            context,
            module,
            builder,
            mir_module,
            cdylib_used: Vec::new(),
            env: HashMap::new()
        }
    }

    pub fn mir_ty_to_llvm_ty(&mut self, mir_ty: Type) -> TypeEnum {
        match mir_ty {
            Type::Int => {
                let ty = self.context.i64_type();
                ty.to_type_enum()
            },
            Type::String(len) => {
                let ty = self.context.i8_type().array(len as u64);
                ty.to_type_enum()
            },
            Type::Float => {
                let ty = self.context.float_type();
                ty.to_type_enum()
            },
            Type::Bool => {
                let ty = self.context.i1_type();
                ty.to_type_enum()
            },
            Type::Void => {
                let ty = self.context.void_type();
                ty.to_type_enum()
            },
            _ => unimplemented!()
        }
    }

    pub fn compile_cdylib(&mut self, path: String) {
        let path = std::path::Path::new(path.as_str());

        if ! path.exists() {
            panic!("File not found: {}", path.display());
        }
        let filename = path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let hash = hash_file(path.display().to_string().as_str());
        let new_name = format!("librs{}_{}.dylib", hash, filename);
        let target_path = var("POPPER_PATH").unwrap();
        let target_path = std::path::Path::new(target_path.as_str());
        let lib_path = target_path.join("libs");

        if ! lib_path.exists() {
            std::fs::create_dir(lib_path.clone()).unwrap();
        }

        let file_path = lib_path.join(new_name);

        let output = cmd!(rustc "--crate-type=cdylib" path.to_str().unwrap().to_string() "-o" file_path.to_str().unwrap());

        if ! output.status.success() {
            panic!("Failed to compile cdylib: {}", output.status);
        }

        self.cdylib_used.push(file_path.display().to_string());
    }

    pub fn compile(&mut self) {
        self.clone().mir_module.ir.iter().for_each(|ir| {
            self.compile_ir(ir.clone());
        });
    }

    pub fn compile_ir(&mut self, ir: Ir) {
        match ir {
            Ir::LoadExternal(name) => {
                self.load_external(name.string);
            },
            Ir::LoadModule(module) => {
                self.load_module(module);
            },
            Ir::Declare(declare) => {
                self.declare(declare);
            },
            Ir::Function(func) => {
                self.compile_function(func);
            },
            _ => unimplemented!()
        }
    }

    pub fn load_external(&mut self, path: String) {
        let path = std::path::Path::new(path.as_str());
        if ! path.exists() {
            panic!("File not found: {}", path.display());
        }

        self.compile_cdylib(path.to_str().unwrap().to_string());
    }

    pub  fn load_module(&mut self, mir_module: MirModule) {
        let mut compiler = Compiler::new(mir_module);
        compiler.compile();
        self.module.link(&compiler.module);
    }

    pub fn declare(&mut self, declare: Declare) {
        let mut args = Vec::new();
        for arg in declare.args.list {
            args.push(
                self.mir_ty_to_llvm_ty(arg)
            );
        }

        let func = self.mir_ty_to_llvm_ty(declare.ret).func(args, false);
        self.module.add_function(declare.name.as_str(), func);
    }

    pub fn compile_function(&mut self, func: popper_mir::mir_ast::Function) {
        let mut args = Vec::new();
        for arg in func.args.args.clone() {
            args.push(
                self.mir_ty_to_llvm_ty(arg.ty)
            );
        }

        let function = self.mir_ty_to_llvm_ty(func.ret).func(args, false);
        let fn_val = self.module.add_function(func.name.as_str(), function);
        for (index, arg) in func.args.args.iter().enumerate() {
            self.env.insert(
                arg.name.clone(),
                PopObject::from(fn_val.get_nth_param(index as u32).unwrap())
            );
        }
        self.env.insert(func.name.clone(), PopObject::new(fn_val.to_value(), function.clone().to_type_enum()));
        let basic_block = self.context.append_basic_block("entry", fn_val);
        self.builder.position_at_end(basic_block);

        for stmt in func.body.body {
            self.compile_stmt(stmt);
        }
    }

    pub fn compile_stmt(&mut self, stmt: BodyFn) -> PopObject {
        match stmt {
            BodyFn::Alloc(alloc) => {
                let ty = self.mir_ty_to_llvm_ty(alloc.ty);
                let value = self.builder.build_alloca(ty, alloc.name.as_str());

                self.env.insert(alloc.name.clone(), PopObject::new(value.to_value_enum(), ty));
                PopObject::new(value.to_value_enum(), ty)
            },
            BodyFn::Call(call) => {
                let mut args = Vec::new();
                for arg in &call.args.list {
                    args.push(self.compile_value(arg.clone()));
                }

                let fn_val = self.module.get_function(call.name.as_str()).unwrap();
                let args = vec![
                    IntType::new_sized(64).int(0, false).to_value_enum(),
                    IntType::new_sized(64).int(0, false).to_value_enum()
                ];
                self.module.verify();
                let value = self.builder.build_call(fn_val, args, call.name.as_str());
                PopObject::new(value, fn_val.get_type())
            },
            BodyFn::Add(add) => {
                let lhs = self.compile_value(add.lhs);
                let rhs = self.compile_value(add.rhs);
                let li = lhs.into_int_value();
                let ri = rhs.into_int_value();
                let out = self.env.get(add.name.as_str()).expect("Out Var not found");
                let res = self.builder.build_int_add(&li, &ri, "add").to_value();
                let ptr = out.value.into_ptr_value();
                self.builder.build_store(res, ptr);

                PopObject::new(res, res.get_type())
            },
            BodyFn::Return(ret) => {
                let val = self.compile_value(ret.value.unwrap());
                self.builder.build_ret(val);

                PopObject::new(val, val.get_type())
            },
            BodyFn::Store(store) => {
                let val = self.compile_value(store.value);
                let out = self.env.get(store.name.as_str()).unwrap();

                self.builder.build_store(val, out.value.into_ptr_value());

                out.clone()
            },
            _ => todo!()
        }
    }

    pub fn compile_value(&mut self, value: MirValue) -> ValueEnum {
        return match value {
            MirValue::Const(cons) => {
                match cons {
                    Const::Int(int) => {
                        let ty = self.context.i64_type();
                        let value = ty.int(int.value as u32, false);
                        value.to_value_enum()
                    },
                    Const::Float(float) => {
                        let ty = self.context.float_type();
                        let value = ty.float(float.value);
                        value.to_value_enum()
                    },
                    Const::String(string) => {
                        let value = self.context
                            .i8_type()
                            .array(string.string.len() as u64)
                            .const_string(string.string.as_str());
                        value.to_value_enum()
                    },
                    Const::Bool(bool) => {
                        let ty = self.context.i1_type();
                        let value = ty.int(bool.value as u32, false);
                        value.to_value_enum()
                    },
                    Const::Void => {
                        let ty = self.context.i8_type();
                        let value = ty.int(0, false);
                        value.to_value_enum()
                    },
                    _ => {
                        unimplemented!()
                    }
                }
            },
            MirValue::Variable(var) => {
                let value = self.env.get(var.name.as_str()).unwrap();
                self.load_value(value.type_enum, value.value)
            }
        };

    }

    pub fn load_value(&mut self, ty: TypeEnum, val: ValueEnum) -> ValueEnum {
        if let ValueEnum::PointerValue(pointer_value) = val {
            let value = self.builder.build_load(ty, pointer_value, "load");
            value
        } else {
            val
        }
    }

    pub fn build(&mut self) -> String {
        self.module.print_to_string().to_string()
    }


}
