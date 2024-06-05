use popper_llvm::builder::{Builder, MathOpType};
use popper_llvm::context::Context;
use popper_llvm::module::Module;
use popper_llvm::types::{Type, TypeBuilder};
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
use crate::cast::cast_type;
use std::collections::HashMap;
use popper_llvm::types::struct_type::StructType;
use popper_mir::types::Types;


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
    allocas_ptr: Vec<ValueEnum>
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
        }
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
                let tys = cast_type(self.context, f.clone());
                self.struct_map.insert(e.get_ident(), tys.into_struct_type());
                
            }
        }

        for ptr in self.allocas_ptr.clone() {
            ptr.erase_from_parent();
        }
    }

    fn compile_function(&mut self, func: &Function) {
        self.current_function = Some(func.clone());
        let ret_ty = cast_type(self.context, func.clone().ret);
        let args = func
            .args
            .iter()
            .map(|arg|
                cast_type(self.context, arg.clone())
            ).collect::<Vec<_>>();

        let func_ty = ret_ty.func(args, false);
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
        let ret_ty = cast_type(self.context, func.clone().ret);
        let args = func
            .args
            .iter()
            .map(|arg|
                cast_type(self.context, arg.clone())
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
                // let ty = cast_type(self.context, decl.ty.clone());
                // let val = self.builder.build_alloca(ty, "");
                // self.allocas_ptr.push(val.to_value_enum());
                // let index = decl.ident.get_index() as usize;
                // self.env.insert(index, val.to_value_enum());
            }
        }
    }

    pub fn compile_command(&mut self, command_enum: &CommandEnum, ident: Option<Ident>) -> Option<ValueEnum> {

        Some(match command_enum {
            CommandEnum::Const(c) => {
                self.compile_const(&c.kind)
            },
            CommandEnum::LLVMLoadPtr(ptr) => {
                let ptr = self.env[ptr.ptr.get_index() as usize];
                let ptr_ty = self.context.i8_type().ptr().to_type_enum();
                self.builder.build_load(ptr_ty, ptr.into_ptr_value(), "")
            },
            CommandEnum::Call(func) => {
                let l_func = self.functions_map.get(&func.function).unwrap().clone();
                let args = func.args.iter().map(|arg| self.compile_expr(arg)).collect::<Vec<_>>();
                self.builder.build_call(l_func, args.as_slice(), "")
            },
            CommandEnum::CopyVal(val) => {
                self.compile_expr(&val.val)
            },
            CommandEnum::LLVMStore(ptr) => {
                let val = self.env[ptr.ptr.get_index() as usize];
                let ptr = self.builder.build_alloca(val.get_type(), "");
                self.builder.build_store(val, ptr);
                ptr.to_value_enum()
            },
            CommandEnum::Ret(val) => {
                let val = self.compile_expr(&val.value);
                self.builder.build_ret(Some(val));
                return None;
            },
            CommandEnum::Add(add) => {
                let lhs = self.compile_expr(&add.left).into_int_value();
                let rhs = self.compile_expr(&add.right).into_int_value();
                let res = self.builder.build_int_add(lhs, rhs, MathOpType::None, "");
                res
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
            CommandEnum::GetElementPtr(gep) => {
                let target_type = cast_type(self.context, gep.target_type.clone());
                let ptr = self.env.get(gep.ptr.get_index() as usize).unwrap().clone().into_ptr_value();
                let index = self.compile_expr(&gep.index);
                //let zero = self.context.i64_type().int(0, false);
                let array = vec![index.into_int_value()];
                let ptr = self.builder.build_inbound_get_element_ptr(target_type, ptr, &array, "");
                let loaded = self.builder.build_load(target_type, ptr.into_ptr_value(), "");
                loaded
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
            ConstKind::Int(i) => self.context.i64_type().int(*i as u32, false).to_value_enum(),
            ConstKind::Float(f) => self.context.float_type().float(*f).to_value_enum(),
            ConstKind::Str(s) => {
                let s = crate::string::replace_sc_string(&s);
                let val = self.builder.build_global_string("", &s);
                val
            },
            ConstKind::Bool(b) => self.context.bool_type().bool(*b).to_value_enum(),
            ConstKind::List(l) => {
                let ty = cast_type(self.context, l[0].get_type());
                let arr = ty.array(l.len() as u64);
                let arr = arr.const_array(l.iter().map(|c| self.compile_expr(c)).collect::<Vec<_>>().as_slice());
                arr.to_value_enum()
            },
            ConstKind::Struct(s, elt) => {
                let args = elt.iter().map(|c| self.compile_expr(c)).collect::<Vec<_>>();
                let s = self.struct_map.get(&s.get_ident()).unwrap();
                s.const_struct(&args)
            },
            _ => unimplemented!()
        }
    }

    pub fn print_to_string(&self) -> String {
        self.module.print_to_string()
    }
}
