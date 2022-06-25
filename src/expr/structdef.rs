use super::ident::Ident;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use std::collections::HashMap;

#[derive(Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Ident>,
}

impl Evaluateur for StructDef {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut f = Vec::new();

        for field in self.fields.clone() {
            match field.clone() {
                Ident(ref ident) => f.push(ident.clone()),
                _ => {
                    return Err(Error::TypeMismatch(TypeMismatchError {
                        expected: Type::None,
                        found: Type::None,
                    }))
                }
            }
        }
        let mut nf = Vec::new();
        for field in self.fields.clone() {
            nf.push(match field {
                Ident(ident) => Ident(ident.clone()),
                _ => {
                    return Err(Error::TypeMismatch(TypeMismatchError {
                        expected: Type::None,
                        found: Type::None,
                    }))
                }
            });
        }
        vm.set_ident(
            Ident(self.name.clone()),
            Var {
                value: Value::DefStruct {
                    name: self.name.clone(),
                    fields: nf,
                    function: HashMap::new(),
                },
                type_: Type::Struct(self.name.clone()),
                mutable: false,
            },
        );
        Ok(Value::None)
    }
}
