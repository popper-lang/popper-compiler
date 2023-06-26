use crate::instr::Bytecode;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(StrPtr),
    Boolean(bool),
    Null
}

impl Bytecode for Literal {
    fn to_bytecode(&self) -> Vec<u8> {
        match self {
            Literal::Integer(value) => {
                let mut bytecode = vec![0x01];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::Float(value) => {
                let mut bytecode = vec![0x02];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::String(value) => {
                let mut bytecode = vec![0x03];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::Boolean(value) => {
                let mut bytecode = vec![0x04];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::Null => vec![0x05],
        }
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        match bytecode[0] {
            0x01 => Literal::Integer(i64::from_bytecode(bytecode[1..].to_vec())),
            0x02 => Literal::Float(f64::from_bytecode(bytecode[1..].to_vec())),
            0x03 => Literal::String(StrPtr::from_bytecode(bytecode[1..].to_vec())),
            0x04 => Literal::Boolean(bool::from_bytecode(bytecode[1..].to_vec())),
            0x05 => Literal::Null,
            _ => panic!("Invalid bytecode for Literal"),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct StrPtr {
    ptr: *const u8,
    len: usize,
}

impl PartialEq for StrPtr {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr, self.clone().len);
            let other_slice = std::slice::from_raw_parts(other.ptr, other.clone().len);
            slice == other_slice
        }
    }
}

impl StrPtr {
    pub fn new(ptr: *const u8, len: usize) -> Self {
        Self { ptr, len }
    }

    pub unsafe fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr, self.clone().len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    pub fn from_str(string: &str) -> Self {
        let ptr = string.as_ptr();
        let len = string.len();
        Self { ptr, len }
    }
}

type U8Ptr = *const u8;

impl Bytecode for StrPtr {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.ptr.to_bytecode());
        bytecode.extend(self.len.to_bytecode());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {

        let ptr = U8Ptr::from_bytecode(bytecode[0..8].to_vec());
        let len = usize::from_bytecode(bytecode[8..16].to_vec());
        Self { ptr, len }
    }
}

impl Bytecode for U8Ptr {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut vec = vec![];
        unsafe {
            let bytes = std::slice::from_raw_parts(*self, 8);
            vec.extend(bytes);
        };

        vec.into_iter().cloned().collect()
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        bytecode.into_raw_parts().0
    }
}

impl Bytecode for usize {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.to_le_bytes());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&bytecode);
        usize::from_le_bytes(bytes)
    }
}

impl Bytecode for i64 {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.to_le_bytes());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&bytecode);
        i64::from_le_bytes(bytes)
    }
}

impl Bytecode for f64 {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.to_le_bytes());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&bytecode);
        f64::from_le_bytes(bytes)
    }
}

impl Bytecode for bool {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        let n = if *self { 1 } else { 0 };
        bytecode.push(n);
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        match bytecode[0] {
            0 => false,
            1 => true,
            _ => panic!("Invalid bytecode for bool"),
        }
    }
}

