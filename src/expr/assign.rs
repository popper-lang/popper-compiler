use crate::ast::Expr;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::errors::*;
use super::ident::Ident;
use crate::value::Value;
use crate::value::Type;
use crate::value::Var;

#[derive(Clone)]
pub struct Assign {
    pub name: String,
    pub value: Box<Expr>,
    pub mutable: bool,
    pub type_: Option<Type>,
}

impl Evaluateur for Assign {

    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let value_evaluate = self.value.eval(vm)?;
        if vm.get_ident(Ident(self.name.clone())).is_some() {
            return Err(Error::VarAlreadyDefined(VarAlreadyDefinedError {
                var_name: self.name.clone(),
            }));
        }
        match self.type_.clone() {
            Some(type_) => {
                if value_evaluate.get_type() != type_ {
                    return Err(Error::TypeMismatch(TypeMismatchError {
                        expected: type_,
                        found: value_evaluate.get_type(),
                    }));
                }
            },
            None => {},
        }

        vm.set_ident(Ident(self.name.clone()), Var {
            value: value_evaluate.clone(),
            type_: value_evaluate.get_type(),
            mutable: self.mutable,
        });
        Ok(Value::None)
    }
}