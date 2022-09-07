use std::rc::Rc;

use super::Object;



pub trait Getter {
    fn fetch(&self, key: String) -> Rc<dyn Object>;
}

pub trait Setter {
    fn fetch(&self, key: String) -> Rc<dyn Object>;
    fn modif(&mut self, key: String, value: Rc<dyn Object>);
}