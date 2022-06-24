use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;

use crate::vm::Vm;
use crate::vm::function;
use crate::errors::*;

use super::ident::Ident;
use crate::ast::Expr;


#[derive(Clone)]
pub struct FunDef {
    pub name: String,
    pub args: Vec<(Ident, Type)>,
    pub body: Box<Expr>
}

impl Evaluateur for FunDef {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut args_vec = Vec::new();
                for arg in self.args.clone() {
                    let arg_name = match arg.clone().0 {
                        Ident(ident) => ident.clone(),
                        _ => {
                            return Err(Error::TypeMismatch(TypeMismatchError {
                                expected: Type::None,
                                found: Type::None,
                            }))
                        }
                    };
                    args_vec.push((arg_name, arg.clone().1));
                }
                
                vm.set_ident(
                    Ident(self.name.clone()),
                    Var {
                        value: Value::Function { name: self.name.clone(), func: function( *self.body.clone()), args: args_vec.clone() },
                        type_: Type::Func,
                        mutable: false,
                    },
                );
                Ok(Value::None)
    }
}
