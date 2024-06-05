use crate::value::RawValue;
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::LLVMInlineAsmDialect;
use crate::types::function_types::FunctionType;
use crate::types::RawType;
use crate::util::to_c_str;

#[derive(Debug, Copy, Clone)]
pub enum InlineAsmDialect {
    ATT,
    Intel
}

impl From<LLVMInlineAsmDialect> for InlineAsmDialect {
    fn from(value: LLVMInlineAsmDialect) -> Self {
        match value {
            LLVMInlineAsmDialect::LLVMInlineAsmDialectATT => InlineAsmDialect::ATT,
            LLVMInlineAsmDialect::LLVMInlineAsmDialectIntel => InlineAsmDialect::Intel
        }
    }
}

impl From<InlineAsmDialect> for LLVMInlineAsmDialect {
    fn from(value: InlineAsmDialect) -> Self {
        match value {
            InlineAsmDialect::ATT => LLVMInlineAsmDialect::LLVMInlineAsmDialectATT,
            InlineAsmDialect::Intel => LLVMInlineAsmDialect::LLVMInlineAsmDialectIntel
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InlineAsm {
    value: RawValue
}

impl InlineAsm {

    pub fn get(
        ty: RawType,
        asm_string: &str,
        constraints: &str,
        has_side_effects: bool,
        is_align_stack: bool,
        dialect: InlineAsmDialect,
        can_throw: bool
    ) -> Self {
        let asm_string_len = asm_string.len();
        let constraints_len = constraints.len();
        let asm_string = to_c_str(asm_string);
        let constraints = to_c_str(constraints);
        let has_side_effects = if has_side_effects { 1 } else { 0 };
        let is_align_stack = if is_align_stack { 1 } else { 0 };
        let dialect: LLVMInlineAsmDialect = dialect.into();
        let can_throw = if can_throw { 1 } else { 0 };

        let lref = unsafe {
            LLVMGetInlineAsm(
                ty.as_llvm_ref(),
                asm_string.as_ptr(),
                asm_string_len,
                constraints.as_ptr(),
                constraints_len,
                has_side_effects,
                is_align_stack,
                dialect,
                can_throw
            )
        };

        InlineAsm::new(lref)

    }
    pub fn new(lref: LLVMValueRef) -> Self {
        InlineAsm {
            value: RawValue::new(lref)
        }
    }

    pub fn as_llvm_ref(&self) -> LLVMValueRef {
        self.value.as_llvm_ref()
    }

    pub fn get_string(&self) -> String {
        unsafe {
            let mut len = 0;
            let c_str = LLVMGetInlineAsmAsmString(self.as_llvm_ref(), &mut len);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }
    
    pub fn get_constraints(&self) -> String {
        unsafe {
            let mut len = 0;
            let c_str = LLVMGetInlineAsmConstraintString(self.as_llvm_ref(), &mut len);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }
    
    pub fn get_dialect(&self) -> InlineAsmDialect {
        unsafe {
            InlineAsmDialect::from(LLVMGetInlineAsmDialect(self.as_llvm_ref()))
        }
    }
    
    pub fn get_function_type(&self) -> FunctionType {
        unsafe {
            FunctionType::new_with_llvm_ref(
                LLVMGetInlineAsmFunctionType(self.as_llvm_ref())
            )
            
        }
    }
    
    pub fn has_side_effects(&self) -> bool {
        unsafe {
            LLVMGetInlineAsmHasSideEffects(self.as_llvm_ref()) == 1
        }
    }
    
    pub fn is_align_stack(&self) -> bool {
        unsafe {
            LLVMGetInlineAsmNeedsAlignedStack(self.as_llvm_ref()) == 1
        }
    }
    
    pub fn can_unwind(&self) -> bool {
        unsafe {
            LLVMGetInlineAsmCanUnwind(self.as_llvm_ref()) == 1
        }
    }
    
    
}