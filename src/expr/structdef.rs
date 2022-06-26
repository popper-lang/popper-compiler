use super::ident::Ident;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Ident>,
}

impl Evaluateur for StructDef {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        vm.set_ident(
            Ident(self.name.clone()),
            Var {
                value: Value::DefStruct {
                    name: self.name.clone(),
                    fields: self.fields.clone(),
                    function: HashMap::new(),
                },
                type_: Type::Struct(self.name.clone()),
                mutable: false,
            },
        );
        Ok(Value::None)
    }
}
