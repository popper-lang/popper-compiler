use super::ident::Ident;
use crate::ast::Expr;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone, Debug)]
pub struct Assign {
    pub name: String,
    pub value: Box<Expr>,
    pub mutable: bool,
    pub type_: Option<Box<Expr>>,
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
                let type_expr = match *type_ {
                    Expr::TypeExpr(type_expr) => type_expr.0,
                    Expr::Typeof(type_of) => match type_of.eval(vm)? {
                        Value::Type(type_) => type_,
                        _ => {
                            return Err(Error::TypeMismatch(TypeMismatchError {
                                expected: Type::None,
                                found: Type::None,
                            }));
                        }
                    },
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::None,
                            found: Type::None,
                        }));
                    }
                };
                if value_evaluate.get_type() != type_expr {
                    return Err(Error::TypeMismatch(TypeMismatchError {
                        expected: type_expr,
                        found: value_evaluate.get_type(),
                    }));
                }
            }
            None => {}
        }

        vm.set_ident(
            Ident(self.name.clone()),
            Var {
                value: value_evaluate.clone(),
                type_: value_evaluate.get_type(),
                mutable: self.mutable,
            },
        );
        Ok(Value::None)
    }
}
