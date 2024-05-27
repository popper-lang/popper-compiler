use inkwell::{
    basic_block::BasicBlock as LLVMBasicBlock, 
    builder::Builder as LLVMBuilder, 
    context::Context as LLVMContext, 
    module::Module as LLVMModule, 
    AddressSpace as LLVMAddressSpace,
    types::{
        BasicType, 
        BasicTypeEnum as LLVMBasicTypeEnum
    }, values::{
        BasicValue as LLVMBasicValue, 
        BasicValueEnum as LLVMBasicValueEnum, 
        FunctionValue as LLVMFunctionValue, 
        IntValue as LLVMIntValue, 
        PointerValue as LLVMPointerValue
    }, IntPredicate as LLVMIntPredicate
};
use popper_common::hash::hash_file;
use popper_mir::{
    Program,
    program::ProgramSection,
    program::ExternalFunction,
    function::Function,
    stmt::*,
    types::Types,
    command::*,
    expr::Expr,
    consts::ConstKind
};
use std::{cell::{Cell, RefCell}, collections::HashMap, env::var, hash::Hash, sync::Arc};

use crate::value::{Flag, LLVMValue};
use std::rc::Rc;

macro_rules! cmd {
    ($cmd:ident $($e:expr)* ) => {
        std::process::Command::new(stringify!($cmd))
            $(.arg($e))*
            .output()
            .expect(concat!("Failed to execute ", stringify!($cmd)))
    };
}

#[derive(Debug, Clone)]
pub struct Compiler<'a> {
    file_name: &'a str,
    mir: Program,
    llvm_context: Rc<&'a LLVMContext>,
    llvm_builder: Arc<LLVMBuilder<'static>>,
    llvm_module: LLVMModule<'a>,
    llvm_block: HashMap<usize, LLVMBasicBlock<'a>>,
    env: HashMap<u64, LLVMBasicValueEnum<'a>>

}

impl<'a>  Compiler<'a> {
    pub fn new(file_name: &'a str, mir: Program, llvm_context: &'a LLVMContext) -> Self {
        let module = llvm_context.create_module(file_name);
        let builder = llvm_context.create_builder();
        Self {
            file_name,
            mir,
            llvm_context: Rc::new(llvm_context),
            llvm_builder: Rc::new(builder),
            llvm_module: module,
            llvm_block: HashMap::new(),
            env: HashMap::new()
        }
    }

    pub fn get_type(&self, ty: Types) -> LLVMBasicTypeEnum {
        match ty {
            Types::Int => self.llvm_context.i32_type().into(),
            Types::Float => self.llvm_context.f32_type().into(),
            Types::Bool => self.llvm_context.bool_type().into(),
            Types::LLVMPtr => self.llvm_context.i8_type().ptr_type(LLVMAddressSpace::default()).into(),
            _ => panic!("Unknown type")

        }
    }

    pub fn compile_program(&mut self) {
        for function in self.mir.programs.clone() {
            self.compile_section(function);
        }
    }

    pub fn compile_section(&mut self, section: ProgramSection) {
        match section {
            ProgramSection::Function(function) => {
                self.compile_function(function);
            }
            ProgramSection::ExternalFunction(external_function) => {
                self.compile_external_function(external_function);

            }
        }
    }

    pub fn compile_external_function<'b>(&'a mut self, external_function: ExternalFunction) {
        let mut args = vec![];
        for arg in external_function.args {
            let arg_type = self.get_type(arg);
            args.push(arg_type.into());
        }
        let ret_type = self.get_type(external_function.ret);
        let func_type = ret_type.fn_type(&args, false);
        self.llvm_module.add_function(&external_function.name, func_type, None);
    }

    pub fn compile_function(&mut self, function: Function) {
        let mut args = vec![];
        for arg in function.args {
            let arg_type = self.get_type(arg);
            args.push(arg_type.into());
        }
        let ret_type = self.get_type(function.ret);
        let func_type = ret_type.fn_type(&args, false);
        let func = self.llvm_module.add_function(&function.name, func_type, None);
        let entry = self.llvm_context.append_basic_block(func, "entry");
        self.llvm_builder.position_at_end(entry);

        for stmt in function.stmts {
            self.compile_statement(stmt);
        }
        
    }

    pub fn compile_statement(&mut self, stmt: Statement) {
        match stmt.kind {
            StmtKind::LetDecl(let_decl) => {
                let ty = self.get_type(let_decl.ty);
                let alloca = self.llvm_builder
                    .build_alloca(ty, &let_decl.ident.get_index().to_string())
                    .expect("Failed to build alloca");

                self.env.insert(let_decl.ident.get_index(), alloca.into());
            },
            StmtKind::Assign(assign) => {
                let ident = assign.ident;
                let command = assign.command;
                let value = self.compile_command(command).unwrap();
                let alloca = self.env.get(&ident.get_index()).unwrap();
                self.llvm_builder.build_store(alloca.into_pointer_value(), value);
            },
            StmtKind::Command(command) => {
                self.compile_command(command);
            }
         }
        
    }

    pub fn compile_command(&mut self, command: CommandEnum) -> Option<LLVMBasicValueEnum> {
        let res = match command {
            CommandEnum::Add(add) => {
                let lhs = self.compile_expr(add.left);
                let rhs = self.compile_expr(add.right);
                self.llvm_builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::Sub(sub) => {
                let lhs = self.compile_expr(sub.left);
                let rhs = self.compile_expr(sub.right);
                self.llvm_builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::Mul(mul) => {
                let lhs = self.compile_expr(mul.left);
                let rhs = self.compile_expr(mul.right);
                self.llvm_builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::Div(div) => {
                let lhs = self.compile_expr(div.left);
                let rhs = self.compile_expr(div.right);
                self.llvm_builder.build_int_unsigned_div(lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::CmpLt(cmp_lt) => {
                let lhs = self.compile_expr(cmp_lt.left);
                let rhs = self.compile_expr(cmp_lt.right);
                self.llvm_builder.build_int_compare(LLVMIntPredicate::SLT, lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::CmpGt(cmp_gt) => {
                let lhs = self.compile_expr(cmp_gt.left);
                let rhs = self.compile_expr(cmp_gt.right);
                self.llvm_builder.build_int_compare(LLVMIntPredicate::SGT, lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::CmpLe(cmp_le) => {
                let lhs = self.compile_expr(cmp_le.left);
                let rhs = self.compile_expr(cmp_le.right);
                self.llvm_builder.build_int_compare(LLVMIntPredicate::SLE, lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::CmpGe(cmp_ge) => {
                let lhs = self.compile_expr(cmp_ge.left);
                let rhs = self.compile_expr(cmp_ge.right);
                self.llvm_builder.build_int_compare(LLVMIntPredicate::SGE, lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::CmpEq(cmp_eq) => {
                let lhs = self.compile_expr(cmp_eq.left);
                let rhs = self.compile_expr(cmp_eq.right);
                self.llvm_builder.build_int_compare(LLVMIntPredicate::EQ, lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::CmpNe(cmp_ne) => {
                let lhs = self.compile_expr(cmp_ne.left);
                let rhs = self.compile_expr(cmp_ne.right);
                self.llvm_builder.build_int_compare(LLVMIntPredicate::NE, lhs.into_int_value(), rhs.into_int_value(), "").unwrap().into()
            },
            CommandEnum::Call(call) => {
                let func = self.llvm_module.get_function(&call.function).unwrap();
                let mut args = vec![];
                for arg in call.args {
                    let arg = self.compile_expr(arg);
                    args.push(arg.into());
                }
                if func.get_type().get_return_type().is_none() {
                    return None;
                }
                self.llvm_builder.build_call(func, &args, "").unwrap().try_as_basic_value().unwrap_left()
            },
            CommandEnum::Const(c) => {
                self.compile_const(c.kind)
            },
            CommandEnum::Ref(r) => {
                let alloca = self.env.get(&r.ident.get_index()).unwrap();
                alloca.clone()
            },
            CommandEnum::LLVMStore(llvm_store) => {
                let ptr = self.env.get(&llvm_store.ptr.get_index()).unwrap();
                ptr.clone()
            },
            CommandEnum::LLVMLoadPtr(llvm_load_ptr) => {
                let ptr = self.env.get(&llvm_load_ptr.ptr.get_index()).unwrap();
                self.llvm_builder.build_load(ptr.get_type(), ptr.into_pointer_value(), "").unwrap().into()
            },
            CommandEnum::CopyVal(c) => {
                let val = self.compile_expr(c.val);
                val.clone()
            },
            CommandEnum::Br(br) => {
                let cond = self.compile_expr(br.cond);
                let tblock = self.llvm_block.get(&br.true_branch.id).unwrap();
                let fblock = self.llvm_block.get(&br.false_branch.id).unwrap();

                self.llvm_builder.build_conditional_branch(cond.into_int_value(), tblock.clone(), fblock.clone());
                return None;

            },
            CommandEnum::Ret(r) => {
                let val = self.compile_expr(r.value);
                self.llvm_builder.build_return(Some(&val));
                return None;
            }
        };

        Some(res)
    }

    pub fn compile_expr<'b>(&'a mut self, expr: Expr) -> LLVMBasicValueEnum<'b> {
        match expr {
            Expr::Const(c) => {
                self.compile_const(c)
            },
            Expr::Ident(i) => {
                let alloca = self.env.get(&i.get_index()).unwrap();
                alloca.clone()
            },

            _ => panic!("Unknown expression")

        }
    }

    pub fn compile_const<'b>(&'a mut self, c: ConstKind) -> LLVMBasicValueEnum<'b> {
        match c {
            ConstKind::Bool(b) => {
                self.llvm_context.bool_type().const_int(1, false).into()
            },
            ConstKind::Null => {
                self.llvm_context.i32_type().const_zero().into()
            },
            ConstKind::Float(f) => {
                self.llvm_context.f32_type().const_float(f).into()
            },
            ConstKind::Int(i) => {
                self.llvm_context.i32_type().const_int(i as u64, false).into()
            },
            ConstKind::Str(s) => {
                let s = self.llvm_context.const_string(s.as_bytes(), false);
                let ptr = unsafe {
                    self.llvm_builder.build_global_string(s, "str").unwrap().as_basic_value_enum()
                };
                ptr
            },
            ConstKind::List(l) => {
                let mut values = vec![];
                for val in l {
                    let val = self.compile_expr(val);
                    values.push(val);
                }
                let last = values.last().unwrap().get_type();
                let array = last.array_type(values.len() as u32).const_array(values);
                array.into()
            }
        }

    }


}