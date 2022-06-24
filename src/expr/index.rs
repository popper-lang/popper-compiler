use crate::ast::Expr;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::value::Value;
use crate::value::Type;
use super::ident::Ident;
use crate::errors::*;

#[derive(Clone)]
pub struct Index {
    pub name: Ident,
    pub index: Box<Expr>,
}


impl Evaluateur for Index {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let real_name = match self.name.clone() {
            Ident(name) => name.clone(),
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::None,
                    found: Type::None,
                }))
            }
        };
        let copy_vm = vm.clone();
        let list = match self.name.eval(vm)? {
            Value::List(list) => list,
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::List,
                    found: self.name.eval(vm)?.get_type(),
                }))
            }
        };

        let index = self.index.eval(vm)?;
        match index {
            Value::Number(num) => {
                if num < 0.0 {
                    return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                        index: num as i32,
                        name: real_name,
                    }));
                }
                if num as usize >= list.len() {
                    return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                        index: num as i32,
                        name: real_name,
                    }));
                }
                Ok(list[num as usize].clone())
            }
            Value::Range(r) => {
                if r.start >= list.len() as isize {
                    return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                        index: r.start as i32,
                        name: real_name,
                    }));
                }

                if r.end > list.len() as isize {
                    return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                        index: r.end as i32,
                        name: real_name,
                    }));
                }

                Ok(Value::List(list[r.start as usize..r.end as usize].to_vec()))
            }
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: index.get_type(),
            }))
        }
    }
}
