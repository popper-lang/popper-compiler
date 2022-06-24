use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::value::Value;
use crate::errors::Error;
use crate::ast::Expr;

#[derive(Clone)]
pub struct List {
    pub elems: Vec<Expr>,
}


impl Evaluateur for List {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut list = Vec::new();
        for elem in self.elems.iter() {
            list.push(elem.eval(vm)?);
        }
        Ok(Value::List(list))
    }
}
