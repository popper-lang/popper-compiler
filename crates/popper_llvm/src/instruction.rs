use crate::value::RawValue;
use llvm_sys::core::*;
use llvm_sys::LLVMOpcode;
use llvm_sys::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum InstructionOpCode {
    Ret = 1,
    Br = 2,
    Switch = 3,
    IndirectBr = 4,
    Invoke = 5,
    Unreachable = 7,
    CallBr = 67,
    FNeg = 66,
    Add = 8,
    FAdd = 9,
    Sub = 10,
    FSub = 11,
    Mul = 12,
    FMul = 13,
    UDiv = 14,
    SDiv = 15,
    FDiv = 16,
    URem = 17,
    SRem = 18,
    FRem = 19,
    Shl = 20,
    LShr = 21,
    AShr = 22,
    And = 23,
    Or = 24,
    Xor = 25,
    Alloca = 26,
    Load = 27,
    Store = 28,
    GetElementPtr = 29,
    Trunc = 30,
    ZExt = 31,
    SExt = 32,
    FPToUI = 33,
    FPToSI = 34,
    UIToFP = 35,
    SIToFP = 36,
    FPTrunc = 37,
    FPExt = 38,
    PtrToInt = 39,
    IntToPtr = 40,
    BitCast = 41,
    AddrSpaceCast = 60,
    ICmp = 42,
    FCmp = 43,
    PHI = 44,
    Call = 45,
    Select = 46,
    UserOp1 = 47,
    UserOp2 = 48,
    VAArg = 49,
    ExtractElement = 50,
    InsertElement = 51,
    ShuffleVector = 52,
    ExtractValue = 53,
    InsertValue = 54,
    Freeze = 68,
    Fence = 55,
    AtomicCmpXchg = 56,
    AtomicRMW = 57,
    Resume = 58,
    LandingPad = 59,
    CleanupRet = 61,
    CatchRet = 62,
    CatchPad = 63,
    CleanupPad = 64,
    CatchSwitch = 65,

}

impl InstructionOpCode {
    pub fn new(op: u8) -> Self {
        match op {
            1 => Self::Ret,
            2 => Self::Br,
            3 => Self::Switch,
            4 => Self::IndirectBr,
            5 => Self::Invoke,
            7 => Self::Unreachable,
            67 => Self::CallBr,
            66 => Self::FNeg,
            8 => Self::Add,
            9 => Self::FAdd,
            10 => Self::Sub,
            11 => Self::FSub,
            12 => Self::Mul,
            13 => Self::FMul,
            14 => Self::UDiv,
            15 => Self::SDiv,
            16 => Self::FDiv,
            17 => Self::URem,
            18 => Self::SRem,
            19 => Self::FRem,
            20 => Self::Shl,
            21 => Self::LShr,
            22 => Self::AShr,
            23 => Self::And,
            24 => Self::Or,
            25 => Self::Xor,
            26 => Self::Alloca,
            27 => Self::Load,
            28 => Self::Store,
            29 => Self::GetElementPtr,
            30 => Self::Trunc,
            31 => Self::ZExt,
            32 => Self::SExt,
            33 => Self::FPToUI,
            34 => Self::FPToSI,
            35 => Self::UIToFP,
            36 => Self::SIToFP,
            37 => Self::FPTrunc,
            38 => Self::FPExt,
            39 => Self::PtrToInt,
            40 => Self::IntToPtr,
            41 => Self::BitCast,
            60 => Self::AddrSpaceCast,
            42 => Self::ICmp,
            43 => Self::FCmp,
            44 => Self::PHI,
            45 => Self::Call,
            46 => Self::Select,
            47 => Self::UserOp1,
            48 => Self::UserOp2,
            49 => Self::VAArg,
            50 => Self::ExtractElement,
            51 => Self::InsertElement,
            52 => Self::ShuffleVector,
            53 => Self::ExtractValue,
            54 => Self::InsertValue,
            68 => Self::Freeze,
            55 => Self::Fence,
            56 => Self::AtomicCmpXchg,
            57 => Self::AtomicRMW,
            58 => Self::Resume,
            59 => Self::LandingPad,
            61 => Self::CleanupRet,
            62 => Self::CatchRet,
            63 => Self::CatchPad,
            64 => Self::CleanupPad,
            65 => Self::CatchSwitch,
            _ => panic!("Invalid opcode")
        }
    }
}

impl From<LLVMOpcode> for InstructionOpCode {
    fn from(opcode: LLVMOpcode) -> Self {
        Self::new(opcode as u8)
    }
}


#[derive(Debug, Copy)]
pub struct InstructionValue {
    value: RawValue
}


impl InstructionValue {
    pub fn new(value: RawValue) -> Self {
        Self { value }
    }


    pub fn get_llvm_ref(&self) -> LLVMValueRef {
        self.value.as_llvm_ref()
    }

    pub fn get_op_code(&self) -> InstructionOpCode {
        let op = unsafe { LLVMGetInstructionOpcode(self.value.as_llvm_ref()) };
        InstructionOpCode::new(op as u8)
    }

    pub fn get_name(&self) -> Option<String> {
        self.value.get_named_value()
    }
    pub fn set_name(&self, name: &str) {
        self.value.set_name(name)
    }
    
    pub fn dump(&self) {
        self.value.dump()
    }
    
    pub fn erase_from_parent(&self) {
        self.value.erase_from_parent()
    }
    
    pub fn print_to_string(&self) -> String {
        self.value.print_to_string()
    }
    
    pub fn replace_all_uses_with(&self, new_value: RawValue) {
        self.value.replace_all_uses_with(new_value)
    }
    
    
    
}


impl Clone for InstructionValue {
    fn clone(&self) -> Self {
        unsafe {
            Self::new(
                RawValue::new(
                    LLVMInstructionClone(self.get_llvm_ref())
                )
            )
        }
    }
}
