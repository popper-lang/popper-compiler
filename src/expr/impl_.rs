use super::ident::Ident;
use crate::ast::Expr;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::function;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone, Debug)]
pub struct Impl {
    pub name_struct: String,
    pub name_method: String,
    pub args: Vec<(Ident, Expr)>,
    pub body: Box<Expr>,
}

impl Evaluateur for Impl {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let fiw;
        let mut fuw;
        match vm.get_ident(Ident(self.name_struct.clone())) {
            Some(Var {
                value:
                    Value::DefStruct {
                        ref fields,
                        ref function,
                        ..
                    },
                ..
            }) => {
                fiw = fields.clone();
                fuw = function.clone();
                for (k, _) in fiw.clone() {
                    if k == Ident(self.clone().name_method) {
                        return Err(Error::MethodAlreadyExists(MethodAlreadyExistsError {
                            name: self.name_method.clone(),
                        }));
                    }
                }
            }
            None => {
                return Err(Error::StructNotFound(StructNotFoundError {
                    name: self.name_struct.clone(),
                }))
            }
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Struct(self.name_struct.clone()),
                    found: Type::None,
                }))
            }
        };

        let mut args_vec = Vec::new();
        for arg in self.args.clone() {
            let Ident(i) = arg.0;
            args_vec.push((
                i,
                match arg.1 {
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
        let f = Value::Function {
            name: self.name_method.clone(),
            func: function(*self.body.clone()),
            args: args_vec,
        };
        fuw.insert(self.name_method.clone(), f);
        vm.set_ident(
            Ident(self.name_struct.clone()),
            Var {
                value: Value::DefStruct {
                    name: self.name_struct.clone(),
                    fields: fiw,
                    function: fuw,
                },
                type_: Type::Struct(self.name_struct.clone()),
                mutable: false,
            },
        );
        Ok(Value::None)
    }
}
