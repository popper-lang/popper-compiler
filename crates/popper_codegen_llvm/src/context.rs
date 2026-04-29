use crate::values::LLVMValue;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::StructType;
use popper_ast::ast::TypeDeclKind;
use popper_ast::type_::Type;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
pub struct CodegenCtxLLVM(Context);

impl Deref for CodegenCtxLLVM {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CodegenCtxLLVM {
    pub fn new() -> Self {
        CodegenCtxLLVM(Context::create())
    }

    pub fn llvm_ctx(&self) -> &Context {
        &self.0
    }
}

pub struct CodegenStateLLVM<'m> {
    pub module: Module<'m>,
    pub builder: Builder<'m>,
}

#[derive(Clone)]
pub struct CodegenObjLLVM<'state> {
    pub state: Arc<CodegenStateLLVM<'state>>,
    pub value: LLVMValue<'state>,
    pub popper_ty: Option<Type>,
}

impl<'state> CodegenObjLLVM<'state> {
    pub fn new(state: Arc<CodegenStateLLVM<'state>>, value: LLVMValue<'state>) -> Self {
        CodegenObjLLVM {
            state,
            value,
            popper_ty: None,
        }
    }

    pub fn with_popper_ty(mut self, ty: Type) -> Self {
        self.popper_ty = Some(ty);
        self
    }

    pub fn can_be_loaded(&self) -> bool {
        self.popper_ty.is_some() && matches!(self.value, LLVMValue::BasicValue(_))
    }
}

#[derive(Clone, Debug)]
pub struct AggregateInfo<'state> {
    pub ty: StructType<'state>,
    pub fields: HashMap<String, u32>,
    pub type_decl_kind: TypeDeclKind,
}

#[derive(Clone, Debug)]
pub struct VaContext<'state> {
    pub types_ptr: inkwell::values::PointerValue<'state>,
    pub args_ptr: inkwell::values::PointerValue<'state>,
    pub index_ptr: inkwell::values::PointerValue<'state>,
    pub count_val: inkwell::values::IntValue<'state>,
}
