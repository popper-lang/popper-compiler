use crate::variable_flag::VariableFlag;
use crate::value_flag::ValueFlag;


#[derive(PartialEq, Clone, Debug)]
pub enum Flag {
    Variable(VariableFlag),
    Value(ValueFlag),
}


impl Flag {
    pub fn expect_value(&self) -> &ValueFlag {
        match self {
            Self::Value(value) => value,
            _ => panic!("Expected ValueFlag, found other"),
        }
    }
}