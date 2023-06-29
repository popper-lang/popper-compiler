
use popper_sbc::instr::Bytecode;


#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum Register {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    RNP,
    RBP,
    Inc(usize, Box<Register>)
}



impl Bytecode for Register {
    fn to_bytecode(&self) -> Vec<u8> {
        match self {
            Register::R1 => vec![0x01],
            Register::R2 => vec![0x02],
            Register::R3 => vec![0x03],
            Register::R4 => vec![0x04],
            Register::R5 => vec![0x05],
            Register::R6 => vec![0x06],
            Register::R7 => vec![0x07],
            Register::R8 => vec![0x08],
            Register::R9 => vec![0x09],
            Register::R10 => vec![0x0A],
            Register::R11 => vec![0x0B],
            Register::R12 => vec![0x0C],
            Register::R13 => vec![0x0D],
            Register::R14 => vec![0x0E],
            Register::R15 => vec![0x0F],
            Register::RNP => vec![0x10],
            Register::RBP => vec![0x11],
            Register::Inc(value, register) => {
                let mut bytecode = vec![0x12];
                bytecode.extend(value.to_bytecode());
                bytecode.extend(register.to_bytecode());
                bytecode
            }
        }
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        match bytecode[0] {
            0x01 => Register::R1,
            0x02 => Register::R2,
            0x03 => Register::R3,
            0x04 => Register::R4,
            0x05 => Register::R5,
            0x06 => Register::R6,
            0x07 => Register::R7,
            0x08 => Register::R8,
            0x09 => Register::R9,
            0x0A => Register::R10,
            0x0B => Register::R11,
            0x0C => Register::R12,
            0x0D => Register::R13,
            0x0E => Register::R14,
            0x0F => Register::R15,
            0x10 => Register::RNP,
            0x11 => Register::RBP,
            0x12 => {
                let value = usize::from_bytecode(bytecode[1..9].to_vec());
                let register = Register::from_bytecode(bytecode[9..10].to_vec());
                Register::Inc(value, Box::new(register))
            }
            _ => panic!("Invalid bytecode for Register")
        }
    }
}
