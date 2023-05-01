use std::collections::HashMap;
use crate::interpreter::Interpreter;
use crate::value::{Implementation, Type};
use crate::value::function::Function;
use crate::value::Object;
use crate::value::RustValue;
use crate::ast::expr::Expr;
use crate::ast::expr::ExprType;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct StructType {
    pub fields: Vec<StructField>,
    pub functions: Vec<Function>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructInstance {
    pub struct_type: StructType,
    pub fields: HashMap<String, Object>,
}

pub fn struct_instance(struct_type: StructType, fields: HashMap<String, Object> ) -> Object {
    let instance = StructInstance {
        struct_type,
        fields
    };
    Object {
        type_: Type::InstanceStruct,
        implementations: vec![
            Implementation::Get(Rc::new(instance.clone()))
        ]
        ,
        value: RustValue::InstanceStruct(instance)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructField {
    pub name: String,
    pub type_: Type,
}


impl StructType {
    fn new() -> Self {
        StructType {
            fields: Vec::new(),
            functions: Vec::new(),
        }
    }


}

impl crate::value::get::Getter for StructInstance {
    fn fetch(&self, _interpreteur: &mut Interpreter, _obj: Object, name: Expr) -> Option<Object> {
        match *name.expr_type {
            ExprType::Ident { ident } => {
                let name = ident.lexeme;
                if let Some(field) = self.fields.get(&name) {
                    return Some(field.clone());
                } else {
                    return None;
                }

            },
            _ => None

        }
    }
}




