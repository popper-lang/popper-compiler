use crate::errors::Error;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::execute_file;
use crate::vm::Vm;
use std::path;

use super::ident::Ident;

#[derive(Clone)]
pub struct Module {
    pub name: String
}

impl Evaluateur for Module {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let vm_of_module = execute_file(self.name.as_str());
        let n = match path::Path::new(&self.name).file_name() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => self.name.clone(),
        };
        vm.set_ident(Ident(n.clone()), Var {
            value: Value::Module {
                context: vm_of_module.0,
                name: n.clone()
            },
            type_: Type::Module(n),
            mutable: false,
        });
        Ok(Value::None)
    }
}