use crate::ast::expr::Expr;
use crate::interpreter::Interpreter;
use crate::value::Object;

pub trait Getter {
    fn fetch(&self, _interpreteur: &mut Interpreter, name: &mut Object, _key: Expr) -> Option<Object> {
        None
    }
}

// Same thing for the trait Getter but for the namespace( `::` ) operator
pub trait NsGetter {
    fn fetch(&self,_interpreteur: &mut Interpreter, _key: Expr) -> Option<Object> {
        None
    }
}

pub trait Setter {
    fn fetch(&self, _key: String) -> Option<Object> {
        None
    }


    fn modif(&mut self, _key: String, _value: Object) -> Option<Object> {
        None
    }
}
