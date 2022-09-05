use super::Object;



pub trait Getter {
    fn fetch(&self, key: String) -> Box<dyn Object>;
}

pub trait Setter {
    fn fetch(&self, key: String) -> Box<dyn Object>;
    fn modif(&mut self, key: String, value: Box<dyn Object>);
}