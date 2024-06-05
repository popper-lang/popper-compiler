use crate::command::{Add, Br, Call, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe, CommandEnum, Const, CopyVal, Div, LLVMLoadPtr, LLVMStore, Mul, Ref, Ret, Sub};

use crate::consts::{ConstKind, Ident, TypeId};
use crate::debug::VarDebugKind;
use crate::expr::Expr;
use crate::function::Function;
use crate::labels::Label;
use crate::marks::{Mark, MarkKind};
use crate::program::{ExternalFunction, Program};
use crate::stmt::Statement;
use crate::types::Types;
use crate::pretty::Pretty;

#[derive(Debug, Clone)]
pub struct Builder {
    current_function: Option<Function>,
    last_ident: Option<Ident>,
    program: Program,
    let_decl_index: usize,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            current_function: None,
            last_ident: None,
            program: Program::new(),
            let_decl_index: 0,
        }
    }

    pub fn new_ident(&mut self, types: Types) -> Ident {
        let ident = if let Some(ident) = &self.last_ident {
            Ident::new(ident.clone(), types)
        } else {
            Ident::create(0, types)
        };
        self.last_ident = Some(ident.clone());
        ident
    }

    pub fn build_function(&mut self, name: &str, args: Vec<Types>, ret: Types) {
        let function = Function::new(name.to_string(), args, ret);
        self.current_function = Some(function);
    }

    pub fn build_external_function(&mut self, name: &str, args: Vec<Types>, ret: Types, is_var_arg: bool) -> ExternalFunction {
        let function = ExternalFunction::new(name.to_string(), args, ret, is_var_arg);
        self.program.add_external_function(function.clone());
        function
    }

    pub fn start_function(&mut self, function: Function) {
        self.current_function = Some(function);
    }

    pub fn end_function(&mut self) -> Function {
        let func = self.current_function.take().unwrap();
        self.last_ident = None;
        self.program.add_function(func.clone());
        func
    }

    fn len(&self) -> usize {
        self.current_function.as_ref().unwrap().stmts.len()
    }

    pub fn build_let_decl(&mut self, ty: Types) -> Ident {
        let ident = self.new_ident(ty.clone());
        if self.let_decl_index > self.len() {
            self.let_decl_index = self.len();
        }
        self.current_function
            .as_mut()
            .unwrap()
            .stmts
            .insert(self.let_decl_index, Statement::new_let_decl(ident.clone(), ty.clone()));
        self.let_decl_index += 1;

        ident
    }

    pub fn build_call_command(&mut self, fn_name: String, args: Vec<Expr>, res: Ident) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    res,
                    CommandEnum::Call(
                        Call::new(fn_name, args)
                    )
                )
            );
    }

    pub fn build_single_call_command(&mut self, fn_name: String, args: Vec<Expr>) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_command(
                    CommandEnum::Call(
                        Call::new(fn_name, args)
                    )
                )
            );
    }

    pub fn build_const_command(&mut self, ident: Ident, consts: ConstKind) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::Const(
                        Const::new(consts)
                    )
                )
            );
    }

    pub fn build_ref_command(&mut self, ident: Ident, ref_ident: Ident) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::Ref(Ref::new(ref_ident))
                )
            );
    }

    pub fn build_llvm_load_ptr_command(&mut self, ident: Ident, ptr: Ident) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::LLVMLoadPtr(
                        LLVMLoadPtr::new(ptr)
                    )
                )
            );
    }

    pub fn build_llvm_store_command(&mut self, ident: Ident, ptr: Ident) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::LLVMStore(
                        LLVMStore::new(ptr)
                    )
                )
            );
    }

    pub fn build_cmp_eq_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::CmpEq(
                        CmpEq::new(left, right)
                    )
                )
            );
    }

    pub fn build_cmp_ne_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::CmpNe(
                        CmpNe::new(left, right)
                    )
                )
            );
    }

    pub fn build_cmp_gt_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::CmpGt(
                        CmpGt::new(left, right)
                    )
                )
            );
    }

    pub fn build_cmp_ge_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::CmpGe(
                        CmpGe::new(left, right)
                    )
                )
            );
    }

    pub fn build_cmp_lt_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::CmpLt(
                        CmpLt::new(left, right)
                    )
                )
            );
    }

    pub fn build_cmp_le_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::CmpLe(
                        CmpLe::new(left, right)
                    )
                )
            );
    }

    pub fn build_add_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::Add(
                        Add::new(left, right)
                    )
                )
            );
    }

    pub fn build_sub_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::Sub(
                        Sub::new(left, right)
                    )
                )
            );
    }

    pub fn build_mul_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::Mul(
                        Mul::new(left, right)
                    )
                )
            );
    }

    pub fn build_div_command(&mut self, ident: Ident, left: Expr, right: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    ident,
                    CommandEnum::Div(
                        Div::new(left, right)
                    )
                )
            );
    }

    pub fn build_br_command(&mut self, cond: Expr, true_branch: Label, false_branch: Label) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_command(
                    CommandEnum::Br(
                        Br::new(cond, *true_branch.get_id(), *false_branch.get_id())
                    )
                )
            );
    }

    pub fn build_ret_command(&mut self, value: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_command(
                    CommandEnum::Ret(
                        Ret::new(value)
                    )
                )
            );
    }

    pub fn build_copy_command(&mut self, id: Ident, expr: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    id,
                    CommandEnum::CopyVal(
                        CopyVal::new(expr)
                    )
                )
            );
    }
    
    pub fn build_type_decl(&mut self, id: TypeId, ty: Types) {
        self.program.add_type_decl(id, ty);
    }
    
    pub fn build_gep_command(&mut self, id: Ident, ptr: Ident, target_type: Types, index: Expr) {
        self.current_function
            .as_mut()
            .unwrap()
            .add_stmt(
                Statement::new_assign(
                    id,
                    CommandEnum::GetElementPtr(
                        crate::command::GetElementPtr::new(ptr, index, target_type)
                    )
                )
            );
    }

    pub fn set_debug_info(&mut self, id: Ident, dbg_kind: VarDebugKind) {
        self.current_function
            .as_mut()
            .unwrap()
            .set_debug_info(id, dbg_kind);
    }

    pub fn use_ident(&mut self, id: Ident) {
        self.current_function
            .as_mut()
            .unwrap()
            .use_ident(id);
    }

    pub fn remove_debug_info(&mut self, id: Ident) {
        self.current_function
            .as_mut()
            .unwrap()
            .remove_debug_info(id);
    }

    pub fn marks_ident(&mut self, id: Ident, marks: MarkKind) {
        self.current_function
            .as_mut()
            .unwrap()
            .marks
            .add_mark(
                Mark::new(id, marks)
            )
    }

    pub fn print_to_string(&self) -> String {
        let mut pretty = Pretty::new(self.program.clone());
        pretty.pretty_program();
        pretty.print_to_string()

    }

    pub fn print_to_file(&self, file: &str) {
        let mut pretty = Pretty::new(self.program.clone());
        pretty.pretty_program();
        pretty.print_to_file(file);
    }

    pub fn print_to_stdout(&self) {
        let mut pretty = Pretty::new(self.program.clone());
        pretty.pretty_program();
        pretty.print_to_stdout();
    }

    pub fn get_program(&self) -> Program {
        self.program.clone()
    }

}
