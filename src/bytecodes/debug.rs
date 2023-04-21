use crate::bytecodes::bytecode::Bytecode;
use crate::bytecodes::bytecode::Operand;


pub fn debug_bytecode(bytecode: &Bytecode) {
    for (i, instruction) in bytecode.instructions.iter().enumerate() {
        println!("{}:  [{}] {:?} {:?}", i, instruction.len(), instruction.opcode, match instruction.operand {
            Some(Operand::Int(i)) => i.to_string(),
            Some(Operand::Float(f)) => f.to_string(),
            Some(Operand::Bool(b)) => b.to_string(),
            None => "".to_string(),
        });
    }
}

