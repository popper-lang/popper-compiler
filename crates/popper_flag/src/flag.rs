use crate::variable_flag::VariableFlag;
use crate::value_flag::ValueFlag;

pub enum Flag {
    Variable(VariableFlag),
    Value(ValueFlag),
}