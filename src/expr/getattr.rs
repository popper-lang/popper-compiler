use crate::ast::Expr;
use crate::errors::*;
use crate::value::Object;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;

use super::ident::Ident;

#[derive(Clone, Debug)]
pub struct GetAttr {
    pub name: Box<Expr>,
    pub attr: String,
}

impl Evaluateur for GetAttr {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let value = self.name.eval(vm)?;
        match value.get_object() {
            Object {
                attr,
                ..
            } => {
                let attr = attr.get(&self.attr);
                match attr {
                    Some(attr) => Ok(attr.value.clone()),
                    None => Err(Error::AttrNotFound(AttrNotFoundError {
                        attr_name: self.attr.clone(),
                    })),
                }
            },
            
        }
    }
}
