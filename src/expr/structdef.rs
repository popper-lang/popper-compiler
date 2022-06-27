use super::ident::Ident;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::ast::Expr;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<(Ident, Expr)>,
}

impl Evaluateur for StructDef {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut fields_vec = Vec::new();
        for field in self.fields.clone() {

            fields_vec.push((
                field.0.clone(),
                match field.clone().1 {
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
                value: Value::DefStruct {
                    name: self.name.clone(),
                    fields: fields_vec,
                    function: HashMap::new(),
                },
                type_: Type::Struct(self.name.clone()),
                mutable: false,
            },
        );
        Ok(Value::None)
    }
}
