use crate::variable_flag::VariableFlag;
use crate::value_flag::ValueFlag;


#[derive(PartialEq, Clone, Debug)]
/// Flag in `Popper-Lang`, flag is used for save program information as a flag(s)
pub enum Flag {
    Variable(VariableFlag),
    Value(ValueFlag),
}


impl Flag {
    /// this method expect a value, else it's panicing
    pub fn expect_value(&self) -> &ValueFlag {
        match self {
            Self::Value(value) => value,
            e => panic!("Expected ValueFlag, found {:?}", e),
        }
    }
}