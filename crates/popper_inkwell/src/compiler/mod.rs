use inkwell::{builder::Builder, context::Context, module::Module, types::{AnyType, BasicType, BasicTypeEnum}, values::{BasicValue, BasicValueEnum}};
use popper_mir::mir_ast::{
    BodyFn, Const, Function as MirFunction, Ir, Module as MirModule, Type as MirType, Value
};
use popper_common::hash::hash_file;
use std::{collections::HashMap, env::var};

use crate::llvm_functions::{popper_va_arg_null_check, popper_panic, LLVMBuiltinFunction};

macro_rules! cmd {
    ($cmd:ident $($e:expr)* ) => {
        std::process::Command::new(stringify!($cmd))
            $(.arg($e))*
            .output()
            .expect(concat!("Failed to execute ", stringify!($cmd)))
    };
}

#[derive(Debug)]
pub struct Compiler<'ctx> {
    builder: Builder<'ctx>,
    context: &'ctx Context,
    module: Module<'ctx>,
    mir_module: MirModule,
    used_cdylibs: Vec<String>,
    env: HashMap<String, BasicValueEnum<'ctx>>,
    can_load: bool,
    old_basic_block: Option<inkwell::basic_block::BasicBlock<'ctx>>,
    is_llvm_va_arg_fn_decl: bool,
    is_current_fn_var_args: bool,
    llvm_functions: HashMap<String, inkwell::values::FunctionValue<'ctx>>,
    llvm_types: HashMap<String, inkwell::types::BasicTypeEnum<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(mir_module: MirModule, context: &'ctx Context) -> Self {
        let module = context.create_module(mir_module.name.as_str());
        let builder = context.create_builder();
        Self {
            builder,
            context,
            module,
            mir_module,
            used_cdylibs: vec![],
            env: HashMap::new(),
            can_load: true,
            is_current_fn_var_args: false,
            is_llvm_va_arg_fn_decl: false,
            llvm_functions: HashMap::new(),
            llvm_types: HashMap::new(),
            old_basic_block: None,
        }
    }

    pub fn define_popper_va_arg_null_check(&mut self) {
        if self.llvm_functions.contains_key("popper.va_arg_null_check") {
            return;
        }
        let ptr_type = self.context.i8_type().ptr_type(Default::default());
        let fn_type = self.context.void_type().fn_type(&[ptr_type.into()], false);

        let function = self.module.add_function(
            "popper.va_arg_null_check",
            fn_type,
            None,
        );

        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let va_arg = function.get_first_param().unwrap().into_pointer_value();
        let is_null = self.builder.build_is_null(va_arg, "is_null").unwrap();
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");
        self.builder.build_conditional_branch(is_null, then_block, else_block).unwrap();

        self.builder.position_at_end(then_block);
        self.builder.build_call(self.module.get_function("popper.panic").expect("panic not found"), &[], "").unwrap();
        self.builder.build_return(None).unwrap();
        self.builder.position_at_end(else_block);
        self.builder.build_return(None).unwrap();

        self.llvm_functions.insert("popper.va_arg_null_check".to_string(), function);

    }

    pub fn define_popper_panic(&mut self) {
        if self.llvm_functions.contains_key("popper.panic") {
            return;
        }
        let fn_type = self.context.void_type().fn_type(&[], false);
        let function = self.module.add_function(
            "popper.panic",
            fn_type,
            None,
        );

        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let s = self.context.const_string("Panic".as_bytes(), true);
        let ptr = self.builder.build_alloca(s.get_type(), "panic_msg").unwrap();
        self.builder.build_store(
            ptr,
            s.as_basic_value_enum()
        ).unwrap();
        self.builder.build_call(
            self.module.get_function("printf").unwrap(),
            &[ptr.into()],
            "",
        ).unwrap();

        self.builder.build_call(
            self.module.get_function("exit").unwrap(),
            &[self.context.i32_type().const_int(1, false).into()],
            "",
        ).unwrap();

        self.builder.build_unreachable().unwrap();
    }

    pub fn declare_exit_function(&mut self) {
        if self.llvm_functions.contains_key("exit") {
            return;
        }
        let fn_type = self.context.void_type().fn_type(&[self.context.i32_type().into()], false);
        let _ = self.module.add_function(
            "exit",
            fn_type,
            None,
        );

    }
    pub fn declare_llvm_va_arg_fn(&mut self) {
        if self.is_llvm_va_arg_fn_decl {
            return;
        }
        self.declare_exit_function();
        self.define_popper_panic();
        self.define_popper_va_arg_null_check();

        let i8_ptr_ty = self.context.i8_type().ptr_type(Default::default());

        let void = self.context.void_type();

        let llvm_va_start = self.module.add_function(
            "llvm.va_start",
            void.fn_type(&[i8_ptr_ty.into()], false),
            None
        );

        let llvm_va_end = self.module.add_function(
            "llvm.va_end",
            void.fn_type(&[i8_ptr_ty.into()], false),
            None
        );

        let llvm_va_copy = self.module.add_function(
            "llvm.va_copy",
            void.fn_type(&[i8_ptr_ty.into(), i8_ptr_ty.into()], false),
            None
        );

        let va_list = self.context.struct_type(&[i8_ptr_ty.into()], false).as_basic_type_enum();

        self.is_llvm_va_arg_fn_decl = true;

        self.llvm_functions.insert("llvm.va_start".to_string(), llvm_va_start);

        self.llvm_functions.insert("llvm.va_end".to_string(), llvm_va_end);

        self.llvm_functions.insert("llvm.va_copy".to_string(), llvm_va_copy);

        self.llvm_types.insert("va_list".to_string(), va_list);

        if let Some(basic_block) = self.old_basic_block {
            self.builder.position_at_end(basic_block);
        }


    }

    pub fn get_used_cdylibs(&self) -> Vec<String> {
        self.used_cdylibs.clone()
    }

    pub fn mir_type_to_llvm_type(&self, ty: MirType) -> inkwell::types::BasicTypeEnum<'ctx> {
        match ty {
            MirType::Int => self.context.i32_type().as_basic_type_enum(),
            MirType::Float => self.context.f32_type().as_basic_type_enum(),
            MirType::String(l) => self.context.i8_type().ptr_type(Default::default()).as_basic_type_enum(),
            MirType::Void => panic!("Void type not supported yet"),
            MirType::Bool => self.context.bool_type().as_basic_type_enum(),
            MirType::List(ty, l) => {
                let llvm_ty = self.mir_type_to_llvm_type(*ty);
                llvm_ty.array_type(l as u32).as_basic_type_enum()
            },
            MirType::Struct(fields) => {
                let mut llvm_fields = vec![];
                for field in fields {
                    let llvm_ty = self.mir_type_to_llvm_type(field);
                    llvm_fields.push(llvm_ty);
                }
                self.context.struct_type(&llvm_fields, false).as_basic_type_enum()
            },
            MirType::Function(..) => {
                panic!("Function type not supported yet")
            }
        }
    }

    pub fn compile_cdylib(&mut self, path: &String) {
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

        self.used_cdylibs.push(file_path.display().to_string());
    }


    pub fn compile(&mut self) {
        self.mir_module
            .ir
            .clone()
            .into_iter()
            .for_each(|ir| {
                self.compile_ir(&ir);
            });
    }
    pub fn compile_ir(&mut self, ir: &Ir) {
        match ir {
            Ir::LoadExternal(ext) => {
                self.compile_cdylib(&ext.string);
            },
            Ir::LoadModule(m) => {
                let mut compiler = Compiler::new(m.clone(), self.context);
                compiler.compile();
                let func = compiler.module.get_functions();
                for f in func {
                    self.module.add_function(f.get_name().to_str().unwrap(), f.get_type(), None);
                }

                self.used_cdylibs.extend(compiler.used_cdylibs);

            },
            Ir::Declare(d) => {
                let name = &d.name;
                let args = d.args.list.iter().map(|arg| {
                    self.mir_type_to_llvm_type(arg.clone()).into()
                }).collect::<Vec<_>>();

                let ret_ty = self.mir_type_to_llvm_type(d.ret.clone());
                let fn_ty = ret_ty.fn_type(args.as_slice(), d.is_var_args);
                self.module.add_function(name.as_str(), fn_ty, None);
            },
            Ir::Function(func) => {
                self.compile_function(func);
            }
            _ => todo!()
        }
    }

    pub fn compile_function(&mut self, func: &MirFunction) {
        let name = &func.name;
        let args = func.args.args.iter().map(|arg| {
            self.mir_type_to_llvm_type(arg.ty.clone()).into()
        }).collect::<Vec<_>>();

        let ret_ty = self.mir_type_to_llvm_type(func.ret.clone());

        let fn_ty = ret_ty.fn_type(args.as_slice(), func.is_var_args);

        let function = self.module.add_function(name.as_str(), fn_ty, None);

        let basic_block = self.context.append_basic_block(function, "entry");
        self.old_basic_block = Some(basic_block);

        self.builder.position_at_end(basic_block);

        for (i, arg) in function.get_param_iter().enumerate() {
            arg.set_name(&func.args.args[i].name);
            self.env.insert(func.args.args[i].name.clone(), arg);

        }


        if func.is_var_args {
            self.is_current_fn_var_args = true;
            if !self.is_llvm_va_arg_fn_decl {
                self.declare_llvm_va_arg_fn();
            }

            let popper_vl =
                self.builder
                    .build_alloca(self.llvm_types.get("va_list").unwrap().clone(), "__popper_vl")
                    .unwrap()
                    .as_basic_value_enum();
            self.env.insert("__popper_vl".to_string(), popper_vl);
            let va_start = self.llvm_functions.get("llvm.va_start").unwrap();
            self.builder.build_call(va_start.clone(), &[popper_vl.into()], "").unwrap();
        }
        for body in func.body.body.iter() {
            self.compile_body_fn(body);
        }

        self.old_basic_block = None;

        if func.is_var_args {
            self.is_current_fn_var_args = false;
        }

        self.env.clear();
    }

    pub fn compile_body_fn(&mut self, body_fn: &BodyFn) {
        match body_fn {
            BodyFn::Return(ret) => {
                let ret = self.compile_value(&ret.value);
                self.ret(Some(ret))
            },
            BodyFn::Call(c) => {
                let name = &c.name;

                let function = self.module.get_function(name.as_str()).unwrap();
                let args = c
                    .args
                    .list
                    .iter()
                    .cloned()
                    .map(|arg| {
                        self.compile_value(&arg).into()
                    })
                    .collect::<Vec<_>>();

                let ret = self.builder.build_call(function, args.as_slice(), "call").unwrap().try_as_basic_value().left().unwrap();
                let val = self.get_unloaded_var(c.ret.to_string()).into_pointer_value();
                self.builder.build_store(val, ret).unwrap();
            },
            BodyFn::VaArg(v) => {
                let val = self.builder.build_va_arg(self.env.get("__popper_vl").unwrap().into_pointer_value(), self.mir_type_to_llvm_type(v.ty.clone()), &v.res).unwrap();
                let ptr = match v.ty {
                    MirType::Int => {
                        let int = val.into_int_value();

                        self.builder.build_int_to_ptr(int, int.get_type().ptr_type(Default::default()), "inttoptr").unwrap()
                    },
                    MirType::Bool => {
                        let int = val.into_int_value();

                        self.builder.build_int_to_ptr(int, int.get_type().ptr_type(Default::default()), "inttoptr").unwrap()
                    },
                    MirType::String(_) => {
                        let array = val.into_array_value();
                        let i8ptr = self.context.i8_type().ptr_type(Default::default());
                        self.builder.build_bitcast(array, i8ptr, "bitcast").unwrap().into_pointer_value()
                    },
                    _ => panic!("unsupported type")
                };

                self.builder.build_call(self.llvm_functions.get("popper.va_arg_null_check").unwrap().clone(), &[ptr.into()], "").unwrap();

                self.env.insert(v.res.clone(), val.as_basic_value_enum());
            }
            BodyFn::Alloc(a) => {
                let ty = self.mir_type_to_llvm_type(a.ty.clone());
                let val = self.builder.build_alloca(ty, &a.name).unwrap();
                self.env.insert(a.name.clone(), val.as_basic_value_enum());
            },
            BodyFn::Store(s) => {
                let val = self.compile_value(&s.value);
                let var = self.env.get(&s.name).unwrap().into_pointer_value();
                self.builder.build_store(var, val).unwrap();
            },
            BodyFn::Add(a) => {
                let lhs = self.compile_value(&a.lhs);
                let rhs = self.compile_value(&a.rhs);
                let val = self.builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "add").unwrap();
                let var = self.get_unloaded_var(a.name.clone()).into_pointer_value();
                self.builder.build_store(var, val.as_basic_value_enum()).unwrap();
            },
            BodyFn::Index(i) => {
                self.can_load = false;
                let minor_type = self.mir_type_to_llvm_type(i.list.get_minor_type().unwrap());
                let val = self.compile_value(&i.list).into_pointer_value();
                let idx = self.compile_value(&i.index).into_int_value();
                let _ = unsafe {
                    self.builder.build_gep(minor_type, val, &[idx], &i.res).unwrap()
                };


            }
        }
    }

    pub fn ret(&self, val: Option<BasicValueEnum>) {
        if self.is_current_fn_var_args {
            let va_end = self.llvm_functions.get("llvm.va_end").expect("llvm.va_end not found.");
            self.builder.build_call(va_end.clone(), &[self.env.get("__popper_vl").expect("__popper_vl llvm var not found").clone().into()], "").unwrap();
        }
        if let Some(ref a) = val {
            self.builder.build_return(Some(a)).unwrap();
        } else {
            self.builder.build_return(None).unwrap();
        }
    }

    pub fn get_unloaded_var(&self, name: String) -> BasicValueEnum {
        self.env.get(&name).unwrap().clone()
    }

    pub fn compile_value(&self, val: &Value) -> BasicValueEnum {
        match val {
            Value::Const(c) => self.compile_const(c),
            Value::Variable(v) => {
                let var = self.env.get(&v.name).unwrap();
                let ty = self.mir_type_to_llvm_type(v.ty.clone());
                if var.is_pointer_value() {
                    if self.can_load {
                        self.builder.build_load(ty, var.into_pointer_value(), "load").unwrap()
                    } else {
                        var.clone()
                    }
                } else {
                    var.clone()
                }
            },
        }
    }

    pub fn compile_const(&self, c: &Const) -> BasicValueEnum {
        match c {
            Const::Int(i) => self.context.i32_type().const_int(i.value as u64, false).into(),
            Const::Float(f) => self.context.f32_type().const_float(f.value as f64).into(),
            Const::String(s) => {
                let s = &s.string;
                let alloc = self.builder.build_alloca(self.context.i8_type().array_type(s.len() as u32 + 1), "alloc").unwrap();
                let val = self.context.const_string(s.as_bytes(), true);
                self.builder.build_store(alloc, val).unwrap();
                alloc.as_basic_value_enum()
            },
            Const::Bool(b) => self.context.bool_type().const_int(b.value as u64, false).into(),
            Const::Void => self.context.i64_type().const_zero().as_basic_value_enum(),
            Const::List(l) => {
                let list = l.values.iter().map(|v| {
                    self.compile_value(v)
                }).collect::<Vec<_>>();

                self.build_array(l.get_minor_type(), list)
            },
        }
    }

    pub fn build_array<'a>(&'a self, ty: MirType, values: Vec<BasicValueEnum<'a>>) -> BasicValueEnum {
        let val = self.mir_type_to_llvm_type(ty);
        match val {
            BasicTypeEnum::IntType(i) => {
                let values = values.iter().map(|v| {
                    v.into_int_value()
                }).collect::<Vec<_>>();

                i.const_array(&values).as_basic_value_enum()
            },
            BasicTypeEnum::FloatType(f) => {
                let values = values.iter().map(|v| {
                    v.into_float_value()
                }).collect::<Vec<_>>();

                f.const_array(&values).as_basic_value_enum()
            },
            BasicTypeEnum::PointerType(p) => {
                let values = values.iter().map(|v| {
                    v.into_pointer_value()
                }).collect::<Vec<_>>();

                p.const_array(&values).as_basic_value_enum()
            },
            _ => todo!()

        }
    }

    pub fn build(&self) -> String {
        self.module.print_to_string().to_string()
    }
}
