use crate::bytecode::BytecodeSerializable;

#[derive(Clone, Debug, PartialEq)]
pub enum BytecodeConstant {
    Int(i32),
    Float(f32),
    String(String),
    Null
}

impl BytecodeSerializable for BytecodeConstant {
    fn serialize(&self) -> Vec<u8> {
        match self {
            BytecodeConstant::Int(int) => {
                let mut vec = Vec::new();
                vec.push(0);
                vec.extend_from_slice(&int.to_le_bytes());
                vec
            },
            BytecodeConstant::Float(float) => {
                let mut vec = Vec::new();
                vec.push(1);
                vec.extend_from_slice(&float.to_le_bytes());
                vec
            },
            BytecodeConstant::String(string) => {
                let mut vec = Vec::new();
                vec.push(2);
                vec.extend_from_slice(string.as_bytes());
                vec
            },
            BytecodeConstant::Null => {
                let mut vec = Vec::new();
                vec.push(3);
                vec
            }
        }
    }

    fn deserialize(vec: Vec<u8>) -> Self {
        match vec[0] {
            0 => {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&vec[1..5]);
                BytecodeConstant::Int(i32::from_le_bytes(bytes))
            },
            1 => {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&vec[1..5]);
                BytecodeConstant::Float(f32::from_le_bytes(bytes))
            },
            2 => {
                let string = String::from_utf8(vec[1..].to_vec()).unwrap();
                BytecodeConstant::String(string)
            },
            3 => BytecodeConstant::Null,
            _ => panic!("Unknown constant type")
        }
    }
}