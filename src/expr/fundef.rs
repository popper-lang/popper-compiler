use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;

use crate::errors::*;
use crate::vm::function;
use crate::vm::Vm;

use super::ident::Ident;
use crate::ast::Expr;

#[derive(Clone)]
pub struct FunDef {
    pub name: String,
    pub args: Vec<(Ident, Expr)>,
    pub body: Box<Expr>,
}

impl Evaluateur for FunDef {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut args_vec = Vec::new();
        for arg in self.args.clone() {
            let Ident(arg_name) = arg.clone().0;
            args_vec.push((
                arg_name,
                match arg.clone().1 {
                    Expr::TypeExpr(type_expr) => type_expr.0,
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::None,
                            found: Type::None,
                        }))
                    }
                },
            ));
        }

        vm.set_ident(
            Ident(self.name.clone()),
            Var {
                value: Value::Function {
                    name: self.name.clone(),
                    func: function(*self.body.clone()),
                    args: args_vec.clone(),
                },
                type_: Type::Func,
                mutable: false,
            },
        );
        Ok(Value::None)
    }
}
