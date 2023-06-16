use crate::type_flag::TypeFlag;

pub struct ValueFlag {
    pub r#type: TypeFlag,
}

impl ValueFlag {
    pub fn new(r#type: TypeFlag) -> Self {
        Self {
            r#type
        }
    }
}