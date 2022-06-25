use super::ident::Ident;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone)]
pub struct Enum {
    pub name: String,
    pub fields: Vec<String>,
}

impl Evaluateur for Enum {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        vm.set_ident(
            Ident(self.name.clone()),
            Var {
                value: Value::Enum {
                    variants: self.fields.clone(),
                },
                type_: Type::Enum,
                mutable: false,
            },
        );
        Ok(Value::None)
    }
}
