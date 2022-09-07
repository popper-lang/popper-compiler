use super::{Object, Type};

impl Object for Vec<Box<dyn Object>> {
    fn get_type(&self) -> Type {
        Type::List
    }

    fn display_value(&self) -> String {
        let mut res = String::from("[");
        for i in self {
            res += i.display_value().as_str();
            res += ",";
        }
        res += "]";
        res
    }
}
