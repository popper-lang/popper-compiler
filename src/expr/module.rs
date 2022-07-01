use crate::errors::Error;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::execute_file;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use std::path;

use super::ident::Ident;

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub as_name: String,
}

impl Evaluateur for Module {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let vm_of_module = match execute_file(self.name.as_str()) {
            Ok(vm) => vm,
            Err(e) => panic!("{}", e),
        };
        

        vm.set_ident(
            Ident(self.as_name.clone()),
            Var {
                value: Value::Module {
                    context: vm_of_module.0,
                    name: self.as_name.clone(),
                },
                type_: Type::Module(self.as_name.clone()),
                mutable: false,
            },
        );
        Ok(Value::None)
    }
}
