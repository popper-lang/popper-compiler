use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;

use super::ident::Ident;

#[derive(Clone)]
pub struct GetAttr {
    pub name: String,
    pub attr: String,
}

impl Evaluateur for GetAttr {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        match vm.get_ident(Ident(self.name.clone())) {
            Some(Var {
                value: Value::CallStruct { ref fields, .. },
                ..
            }) => match fields.get(&Ident(self.attr.clone())) {
                Some(v) => return Ok(v.clone()),
                None => {
                    return Err(Error::AttrNotFound(AttrNotFoundError {
                        attr_name: self.attr.clone(),
                    }))
                }
            },
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Struct(self.name.clone()),
                    found: Type::None,
                }))
            }
        };
    }
}
