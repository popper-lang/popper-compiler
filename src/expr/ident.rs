use crate::errors::*;
use crate::value::Value;
use crate::vm::{Evaluateur, Vm};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Ident(pub String);

impl Evaluateur for Ident {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        match vm.get_ident(self.clone()) {
            Some(var) => Ok(var.value.clone()),
            None => Err(Error::VarNotFound(VarNotFoundError {
                var_name: self.clone().0,
            })),
        }
    }
}
