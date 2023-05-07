use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::interpreter::Interpreter;
use crate::value::{get, Implementation, Type};
use crate::value::function::Function;
use crate::value::Object;
use crate::value::Value;
use crate::ast::expr::Expr;
use crate::ast::expr::ExprType;
use std::rc::Rc;
use crate::get_impl_if_exist;


pub trait Struct {
    fn get(&self, name: &str) -> Option<Object>;
    fn set(&mut self, name: &str, value: Object);
}

#[derive(Clone)]
pub struct BuiltinStruct(Rc<dyn Struct>, String);

impl Debug for BuiltinStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("StructBuiltin({})", self.1).as_str())
    }
}

impl PartialEq for BuiltinStruct {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}


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
    let mut instance = StructInstance {
        struct_type: struct_type.clone(),
        fields
    };

    for function in struct_type.functions {
        println!("function: {:?}", function);
        instance.fields.insert(function.name, Function::create_function(function.declaration));
    }
    Object {
        type_: Type::InstanceStruct,
        implementations: vec![
            Implementation::Get(Rc::new(instance.clone()))
        ]
        ,
        value: Value::InstanceStruct(instance),
        tags: std::default::Default::default()
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
    fn fetch(&self, interpreteur: &mut Interpreter, obj: &mut Object, name: Expr) -> Option<Object> {
        match *name.expr_type {
            ExprType::Ident { ident } => {
                let name = ident.lexeme;
                if let Some(field) = self.fields.get(&name) {
                    return Some(field.clone());
                } else {
                    return None;
                }

            },
            ExprType::Call { name: f_name, args} => {
                let func = self.fetch(interpreteur, obj, f_name)?;
                let callable = get_impl_if_exist!(Call, func);
                if let Some(e) = callable {
                    let mut new_args = vec![];
                    for arg in args {
                        new_args.push(arg.accept(interpreteur));
                    }

                    Some(e.method(interpreteur, obj, &mut new_args, name.file.as_str()))
                } else {
                    panic!("can't get")
                }
            }
            _ => None

        }
    }
}




