use crate::ast::Expr;
use crate::vm::Vm;
use crate::vm::Evaluateur;
use crate::errors::*;
use crate::value::Value;
use crate::value::Var;
use super::ident::Ident;

#[derive(Clone)]
pub struct SetVar {
    pub name: String,
    pub value: Box<Expr>
}

impl Evaluateur for SetVar {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let v = self.value.eval(vm)?;
        if let None = vm.get_ident(Ident(self.name.clone())) {
            return Err(Error::VarNotFound(VarNotFoundError {
                var_name: self.name.clone(),
            }));
        } else if let Some(var) = vm.get_ident(Ident(self.name.clone())) {
            if ! var.mutable {
                return Err(Error::ItsAConstant(ItsAConstantError {
                    var_name: self.name.clone()
                }))
            }
            if var.type_ != v.get_type() {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: var.type_.clone(),
                    found: v.get_type()
                }))
            }
        }

        
        vm.set_ident(Ident(self.name.clone()), Var {value: v.clone(), type_: v.get_type(), mutable: true});
        Ok(Value::None)
    }
}