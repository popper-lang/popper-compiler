use std::collections::HashMap;
use crate::bytecodes::bytecode::Bytecode;
use crate::bytecodes::bytecode::Opcode;
use crate::bytecodes::bytecode::Operand;


pub struct Vm {
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
    pub constants: Vec<Value>,
    pub ip: usize,
    pub frames: Vec<Frame>,
}

pub struct Frame {
    pub ip: usize,
    pub slots: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil
}


impl Vm {

    pub fn new() -> Self {
        Vm {
            stack: vec![],
            globals: HashMap::new(),
            constants: vec![],
            ip: 0,
            frames: vec![],
        }
    }

    pub fn run(&mut self, bytecode: &Bytecode) -> Result<(), String> {
        self.frames.push(Frame {
            ip: 0,
            slots: vec![],
        });
        loop {

            let frame = self.frames.last_mut().unwrap();
            if frame.ip >= bytecode.instructions.len() {
                return Ok(());
            }
            let instruction = bytecode.instructions[frame.ip];
            frame.ip += 1;
            match instruction.opcode {
                Opcode::LoadConst => {

                    let constant = if let Some(Operand::Int(i)) = instruction.operand {
                        Value::Number(i as f64)
                    } else if let Some(Operand::Float(f)) = instruction.operand {
                        Value::Number(f as f64)
                    } else if let Some(Operand::Bool(b)) = instruction.operand {
                        Value::Boolean(b)
                    }  else {
                        return Err("Invalid operand".to_string());
                    };
                    self.stack.push(constant);
                }
                Opcode::Add => {
                    dbg!(&self.stack);
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a, b) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        _ => return Err("Operands must be numbers".to_string()),
                    };
                    self.stack.push(result);
                }
                Opcode::Subtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a, b) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                        _ => return Err("Operands must be numbers".to_string()),
                    };
                    self.stack.push(result);
                }
                Opcode::Multiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a, b) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                        _ => return Err("Operands must be numbers".to_string()),
                    };
                    self.stack.push(result);
                }
                Opcode::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a, b) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                        _ => return Err("Operands must be numbers".to_string()),
                    };
                    self.stack.push(result);
                }
                Opcode::Negate => {
                    let a = self.stack.pop().unwrap();
                    let result = match a {
                        Value::Number(a) => Value::Number(-a),
                        _ => return Err("Operand must be a number".to_string()),
                    };
                    self.stack.push(result);
                },
                Opcode::If => {
                    let condition = self.stack.pop().unwrap();
                    let offset = if let Some(Operand::Int(i)) = instruction.operand {
                        i
                    } else {
                        return Err("Invalid operand".to_string());
                    };
                    if let Value::Boolean(true) = condition {
                        frame.ip = (offset - 1) as usize;
                    } else {
                        frame.ip += 1;
                    }
                }
                Opcode::Jump => {
                    let offset = if let Some(Operand::Int(i)) = instruction.operand {
                        i
                    } else {
                        return Err("Invalid operand".to_string());
                    };
                    frame.ip = (offset - 1) as usize;
                }
            }
        }
    }


}