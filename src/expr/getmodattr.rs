use super::ident::Ident;
use crate::errors::*;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone)]
pub struct GetModAttr {
    pub mod_name: String,
    pub attr_name: String,
}

impl Evaluateur for GetModAttr {
    fn eval(&self, vm: &mut Vm) -> Result<Value, crate::errors::Error> {
        let vm = vm.clone();
        let mod_name = self.mod_name.clone();
        let attr_name = self.attr_name.clone();
        let value = match vm.get_ident(Ident(mod_name.clone())) {
            Some(var) => var,
            None => return Err(Error::VarNotFound(VarNotFoundError { var_name: mod_name })),
        };
        match value {
            Var {
                value: Value::Module { context, .. },
                ..
            } => {
                let value = context
                    .get(&Ident(attr_name.clone()))
                    .ok_or(Error::AttrNotFound(AttrNotFoundError {
                        attr_name: attr_name,
                    }))?;
                Ok(value.clone().value)
            }
            _ => Err(Error::AttrNotFound(crate::errors::AttrNotFoundError {
                attr_name: attr_name.clone(),
            })),
        }
    }
}
