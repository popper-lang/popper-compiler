use popper_llvm::builder::{Builder, MathOpType};
use popper_llvm::context::Context;
use popper_llvm::module::Module;
use popper_llvm::types::{RawType, Type as TypeTrait, TypeBuilder, TypeEnum};
use popper_llvm::value::function_value::FunctionValue;
use popper_llvm::value::{Value, ValueEnum};
use popper_mir::command::CommandEnum;
use popper_mir::consts::{ConstKind, Ident};
use popper_mir::expr::Expr;
use popper_mir::function::Function;
use popper_mir::marks::MarkKind;
use popper_mir::program::{ExternalFunction, Program};
use popper_mir::program::ProgramSection;
use popper_mir::stmt::{Statement, StmtKind};
use std::collections::HashMap;
use popper_llvm::types::struct_type::StructType;
use popper_mir::types::Types;
use crate::type_::Type;


#[derive(Debug, Copy, Clone)]
enum BitsMode {
    B8,
    B16,
    B32,
    B64
}

#[derive(Debug, Clone)]
pub struct Compiler {
    builder: Builder,
    context: Context,
    module: Module,
    program: Program,
    env: Vec<ValueEnum>,
    current_function: Option<Function>,
    functions_map: HashMap<String, FunctionValue>,
    struct_map: HashMap<String, StructType>,
    allocas_ptr: Vec<ValueEnum>,
    current_bits_mode: BitsMode
}

impl Compiler {
    pub fn new(program: Program, file: &str) -> Self {
        let context = Context::create();
        let builder = context.new_builder();
        let module = context.new_module(file);
        Compiler {
            builder,
            context,
            module,
            program,
            env: vec![],
            current_function: None,
            functions_map: HashMap::new(),
            struct_map: HashMap::new(),
            allocas_ptr: vec![],
            current_bits_mode: BitsMode::B32
        }
    }

    fn switch_bits_mode(&mut self, mode: BitsMode) {
        self.current_bits_mode = mode;
    }

    fn int_type(&self) -> TypeEnum {
        match self.current_bits_mode {
            BitsMode::B8 => self.context.i8_type().to_type_enum(),
            BitsMode::B16 => self.context.i16_type().to_type_enum(),
            BitsMode::B32 => self.context.i32_type().to_type_enum(),
            BitsMode::B64 => self.context.i64_type().to_type_enum()
        }
    }

    fn int_value(&self, val: i64) -> ValueEnum {
        self.int_type().into_int_type().int(val as u32, false).to_value_enum()
    }


    pub fn cast_type(&self, context: Context, types: Types, is_pure_struct: bool) -> Type {
        Type::from_type(match types {
            Types::Int => self.int_type(),
            Types::Float => context.float_type().to_type_enum(),
            Types::Bool => context.bool_type().to_type_enum(),
            Types::String(_) => context.i8_type().ptr().to_type_enum(),
            Types::LLVMPtr => context.i8_type().ptr().to_type_enum(),
            Types::List(sub_ty, l) => {
                let sub_ty = self.cast_type(context, *sub_ty, is_pure_struct).get_type();
                let sub_ty = sub_ty.array(l as u64);
                sub_ty.to_type_enum()
            },
            Types::Ptr(sub_ty) => {
                let sub_ty = self.cast_type(context, *sub_ty, is_pure_struct).get_type();
                sub_ty.ptr().to_type_enum()
            },
            Types::Struct(name, ty) => {
                let tys = ty.iter().map(|t| self.cast_type(context, t.clone(), false).get_type()).collect::<Vec<_>>();
                let ty = context.named_struct_type(&name);
                ty.set_body(&tys, false);
                let s = ty.to_type_enum();
                if is_pure_struct {
                    s
                } else {
                    let ty = self.context.i64_type().to_type_enum();
                    return Type::new(ty, s);
                }
            },
            Types::Label => panic!("Cannot cast to label type"),
            Types::TypeId(e) => {
                let s = self.struct_map.get(&e).unwrap().to_type_enum();
                if is_pure_struct {
                    s
                } else {
                    let ty = self.context.i64_type().to_type_enum();
                    return Type::new(ty, s);
                }
            },
            _ => panic!("Cannot cast to void type")
        })
    }
    


    fn is_marked(&self, ident: &Ident, mark: MarkKind) -> bool {
        self.current_function.as_ref().unwrap().marks.contains_mark(ident, mark)
    }

    pub fn compile_program(&mut self) {
        for section in self.program.programs.clone().into_iter() {
            self.compile_section(&section);
        }

    }

    fn compile_section(&mut self, section: &ProgramSection) {
        match section {
            ProgramSection::Function(func) => {
                self.compile_function(func);
            },
            ProgramSection::ExternalFunction(func) => {
                self.compile_external_function(func);
            },
            ProgramSection::TypeDecl(e, f) => {
                let tys = self.cast_type(self.context, f.clone(), true).get_type();
                self.struct_map.insert(e.get_ident(), tys.into_struct_type());
            }
        }

        for ptr in self.allocas_ptr.clone() {
            ptr.erase_from_parent();
        }
    }

    fn compile_function(&mut self, func: &Function) {
        self.current_function = Some(func.clone());
        let void_type = RawType::void_type();
        let args = func
            .args
            .iter()
            .map(|arg|
                self.cast_type(self.context, arg.clone(), false).get_type()
            ).collect::<Vec<_>>();
        let func_ty = if func.ret == Types::Unit {
            void_type.func(args, false)
        } else {
            let ret_ty = self.cast_type(self.context, func.ret.clone(), false).get_type();
            ret_ty.func(args, false)
        };
        let llvm_func = self.module.add_function(&func.name, func_ty);

        self.functions_map.insert(func.name.clone(), llvm_func);

        let entry = self.context.append_basic_block("entry", llvm_func);

        for arg in llvm_func.get_all_params() {
            self.env.push(arg);
        }
        self.builder.position_at_end(entry);

        for stmt in &func.stmts {
            self.compile_stmt(stmt);
        }

        self.env.clear();
        self.current_function = None;

    }

    fn compile_external_function(&mut self, func: &ExternalFunction) {
        let ret_ty = self.cast_type(self.context, func.clone().ret, false).get_type();
        let args = func
            .args
            .iter()
            .map(|arg|
                self.cast_type(self.context, arg.clone(), false).get_type()
            ).collect::<Vec<_>>();

        let func_ty = ret_ty.func(args, func.is_var_arg);
        let r = self.module.add_function(&func.name, func_ty);
        self.functions_map.insert(func.name.clone(), r);
    }


    pub fn compile_stmt(&mut self, stmt: &Statement) {
        match stmt.kind.clone() {
            StmtKind::Assign(assign) => {
                let ident = assign.ident;
                let val = self.compile_command(&assign.command, Some(ident.clone())).unwrap();
                let index = ident.get_index() as usize;
                self.env.insert(index, val);

            },
            StmtKind::Command(command) => {
                self.compile_command(&command, None);
            },
            StmtKind::LetDecl(decl) => {
                let zero = self.context.i64_type().int(0, false);
                self.env.push(zero.to_value_enum());
            }
        }
    }

    pub fn compile_command(&mut self, command_enum: &CommandEnum, ident: Option<Ident>) -> Option<ValueEnum> {

        Some(match command_enum {
            CommandEnum::Const(c) => {
                self.compile_const(&c.kind)
            },
            CommandEnum::LLVMLoadPtr(ptr) => {
                let ptr_val = self.env[ptr.ptr.get_index() as usize].into_ptr_value();
                let ptr_ty = self.cast_type(self.context, ptr.as_type.clone(), false).get_type();
                self.builder.build_load(ptr_ty, ptr_val, "")
            },
            CommandEnum::Call(func) => {
                let l_func = *self.functions_map.get(&func.function).unwrap();
                let args = func.args.iter().map(|arg| self.compile_expr(arg)).collect::<Vec<_>>();
                return self.builder.build_call(l_func, args.as_slice(), "");
            },
            CommandEnum::CopyVal(val) => {
                self.compile_expr(&val.val)
            },
            CommandEnum::LLVMStore(ptr) => {
                let val = self.env[ptr.ptr.get_index() as usize];
                let ptr = self.builder.build_alloca(self.cast_type(self.context, ptr.as_type.clone(), false).get_real_type(), "");
                self.builder.build_store(val, ptr);
                ptr.to_value_enum()
            },
            CommandEnum::Ret(val) => {
                if val.value.is_null() {
                    self.builder.build_ret(None);
                    return None;
                }
                let val = self.compile_expr(&val.value);
                self.builder.build_ret(Some(val));
                return None;
            },
            CommandEnum::Add(add) => {
                let lhs = self.compile_expr(&add.left).into_int_value();
                let rhs = self.compile_expr(&add.right).into_int_value();
                self.builder.build_int_add(lhs, rhs, MathOpType::None, "")
            },
            CommandEnum::Sub(sub) => {
                let lhs = self.compile_expr(&sub.left).into_int_value();
                let rhs = self.compile_expr(&sub.right).into_int_value();
                self.builder.build_int_sub(lhs, rhs, MathOpType::None, "")
            },
            CommandEnum::Mul(mul) => {
                let lhs = self.compile_expr(&mul.left).into_int_value();
                let rhs = self.compile_expr(&mul.right).into_int_value();
                self.builder.build_int_mul(lhs, rhs, MathOpType::None, "")
            },
            CommandEnum::GetElementPtrStruct(gep) => {
                let struct_ty = self.struct_map.get(&gep.struct_id.get_ident()).unwrap().clone();
                let mut val = self.env.get(gep.ptr.get_index() as usize).unwrap();
                let ptr= val.into_ptr_value();
                let index = self.compile_expr(&gep.index);
                let zero = self.context.i32_type().int(0, false);
                let array = vec![zero, index.into_int_value()];
                let ptr = self.builder.build_inbound_get_element_ptr(struct_ty.to_type_enum(), ptr, &array, "");
                if self.is_marked(&ident.unwrap(), MarkKind::Ptr) {
                    ptr
                } else {
                    self.builder.build_load(self.context.i64_type().to_type_enum(), ptr.into_ptr_value(), "")
                }
            },
            
            CommandEnum::Write(write) => {
                let val = self.compile_expr(&write.value);
                let ptr = self.env.get(write.ptr.get_index() as usize).unwrap();
                self.builder.build_store(val, ptr.into_ptr_value());
                return None;
            },
            _ => unimplemented!()


        })
    }

    fn compile_expr(&mut self, expr: &Expr) -> ValueEnum {
        match expr {
            Expr::Const(consts) => {
                self.compile_const(consts)
            },
            Expr::Ident(ident) => {
                let val = self.env[ident.get_index() as usize];
                if let Some(e) = self.allocas_ptr.iter().position(|x| x == &val) {
                    self.allocas_ptr.remove(e);
                }

                val
            },
            _ => unimplemented!()
        }
    }

    fn compile_const(&mut self, c: &ConstKind) -> ValueEnum {
        match c {
            ConstKind::Int(i) => self.int_value(*i),
            ConstKind::Float(f) => self.context.float_type().float(*f).to_value_enum(),
            ConstKind::Str(s) => {
                let s = crate::string::replace_sc_string(s);
                self.builder.build_global_string("", &s)
            },
            ConstKind::Bool(b) => self.context.bool_type().bool(*b).to_value_enum(),
            ConstKind::List(l) => {
                let ty = self.cast_type(self.context, l[0].get_type(), false).get_type();
                let arr = ty.array(l.len() as u64);
                let arr = arr.const_array(l.iter().map(|c| self.compile_expr(c)).collect::<Vec<_>>().as_slice());
                arr.to_value_enum()
            },
            ConstKind::Struct(s, elt) => {

                let args = elt.iter().map(|c| self.compile_expr(c)).collect::<Vec<_>>();

                let s = self.struct_map.get(&s.get_ident()).unwrap();
                let alloca = self.builder.build_alloca(s.to_type_enum(), "");
                let zero = self.context.i32_type().int(0, false);
                for (i, arg) in args.iter().enumerate() {
                    let index = self.context.i32_type().int(i as u32, false);
                    let array = vec![zero, index];
                    let ptr = self.builder.build_inbound_get_element_ptr(s.to_type_enum(), alloca, &array, "");
                    self.builder.build_store(*arg, ptr.into_ptr_value());
                }
                let int64 = self.context.i64_type();
                let loaded = self.builder.build_load(int64.to_type_enum(), alloca, "");
                loaded
            },
            _ => unimplemented!()
        }
    }

    pub fn print_to_string(&self) -> String {
        self.module.print_to_string()
    }
}
