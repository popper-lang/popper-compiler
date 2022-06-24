use crate::ast::Expr;
use crate::value::Type;
use crate::value::Value;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::errors::*;

#[derive(Clone)]
pub struct Range {
    pub start: Box<Expr>,
    pub end: Box<Expr>,
}

impl Evaluateur for Range {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let start = self.start.eval(vm)?;
        let end = self.end.eval(vm)?;
        let start = match start {
            Value::Number(n) => n,
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Int,
                    found: start.get_type(),
                }))
            }
        };
        let end = match end {
            Value::Number(n) => n,
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Int,
                    found: end.get_type(),
                }))
            }
        };

        Ok(Value::Range(start as isize..end as isize))
    }
}