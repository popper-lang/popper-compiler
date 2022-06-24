use crate::vm::Vm;
use crate::vm::Evaluateur;
use crate::errors::*;
use crate::value::Value;
use crate::value::Type;
use crate::value::Var;

use super::ident::Ident;

#[derive(Clone)]
pub struct EnumCall {
    pub name: String,
    pub field: String
}

impl Evaluateur for EnumCall {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        match vm.get_ident(Ident(self.name.to_string())) {
            Some(Var {value: Value::Enum { variants: fields }, ..}) => {
                if fields.contains(&self.field) {
                    Ok(Value::EnumCall { name: self.name.clone(), field: self.field.clone() })
                } else {
                    Err(Error::FieldEnumNotFound(FieldEnumNotFoundError {
                        name: self.name.clone(),
                        field: self.field.clone(),
                    }))
                }
            },
            Some(e) => {
                Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Enum,
                    found: e.value.get_type(),
                }))
            }
            _ => {
                Err(Error::EnumNotFound(EnumNotFoundError {
                    name: self.name.clone()
                }))
            }
        }
    }
}
