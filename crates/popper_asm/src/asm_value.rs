use crate::register::Register;

#[derive(Clone, Debug)]
pub enum AsmValue {
    Register(Register),
    Immediate(Immediate),
    Memory(Memory),
}

#[derive(Clone, Debug)]
pub enum Immediate {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Clone, Debug)]
pub enum Memory {
    RegisterOffset(Register, Immediate),
    Label(String)
}