pub mod class;
pub mod function;
pub mod get;
pub mod instance;
pub mod list;
pub mod litteral;
pub mod callable;
pub mod operation;
pub mod namespace;
pub mod struct_type;

use crate::interpreter::Interpreter;
use crate::{
    error,
    errors::{Error, ErrorType},
};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};
use std::borrow::Cow;
use std::fmt::Write;

static BUILTIN_TYPE: &[Type; 4] = &[Type::Int, Type::Bool, Type::String, Type::List];

// a trait Object that represents a object in popper

// this is a enum that represents the implementation of a object
// this is used for the new object system
// WARNING: this enum is not finished
#[derive(Clone)]
pub enum Implementation {
    Add(Rc<dyn operation::Add>),
    Sub(Rc<dyn operation::Sub>),
    Mul(Rc<dyn operation::Mul>),
    Div(Rc<dyn operation::Div>),
    Pow(Rc<dyn operation::Pow>),
    PartialEq(Rc<dyn operation::PartialEq>),
    PartialOrd(Rc<dyn operation::PartialOrd>),
    Get(Rc<dyn get::Getter>),
    Set(Rc<dyn get::Setter>),
    Call(Rc<dyn callable::Callable>),
    NsGet(Rc<dyn get::NsGetter>),
}

// this struct is a 2th version of object
// this struct is used for the new object system
// WARNING: this struct is not finished
// TODO: finish struct
#[derive(Clone)]
pub struct Object {
    pub type_: Type,
    pub implementations: Vec<Implementation>,
    pub value: RustValue
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.value == other.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustValue {
    Int(i32),
    String(String),
    Bool(bool),
    List(Vec<Object>),
    None,
    Function,
    Instance(instance::Instance),
    Class(class::Class),
    Namespace(namespace::Namespace),
    Struct(struct_type::StructType),
    InstanceStruct(struct_type::StructInstance),
    Type(Type),
}



#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Type {
    Int,
    String,
    Bool,
    List,
    Func,
    Range,
    Type(String),
    Any,
    None,
    Function,
    Instance(String),
    Class(String),
    Namespace,
    Struct(String),
    InstanceStruct
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub value: Object,
    pub mutable: bool,
    pub type_: Type,
}

impl Display for Object  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value.to_string().as_str())
    }
}

impl Display for RustValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*match self {
            RustValue::Int(e) => Cow::Owned(e.to_string()),
            RustValue::String(e) => Cow::Borrowed(e.as_str()),
            RustValue::Bool(e) => Cow::Owned(e.to_string()),
            RustValue::List(e) => {
                let mut s = String::new();
                s.push('[');
                for i in e {
                    s.push_str(i.to_string().as_str());
                    s.push_str(", ");
                }
                s.push(']');
                Cow::Owned(format!("{:?}", e))
            }
            RustValue::None => Cow::Borrowed("None"),
            RustValue::Function => Cow::Borrowed("<function>"),
            RustValue::Instance(e) => Cow::Owned(format!("<instance of <class {}>>", e.name)),
            RustValue::Class(e) => Cow::Owned(format!("<class {}>", e.name)),
            RustValue::Namespace(_) => Cow::Borrowed("<namespace>"),
            RustValue::Struct(_) => Cow::Borrowed("<struct>"),
            RustValue::InstanceStruct(_) => Cow::Borrowed("<instance struct>"),
            RustValue::Type(e) => Cow::Owned(format!("<type {}>", e)),
        })
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}: {}", self.type_, self.value).as_str())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*match self {
            Type::Int => Cow::Borrowed("<int>"),
            Type::String => Cow::Borrowed("<string>"),
            Type::Bool => Cow::Borrowed("<bool>"),
            Type::Func => Cow::Borrowed("<func>"),
            Type::List => Cow::Borrowed("<list>"),
            Type::Range => Cow::Borrowed("<range>"),
            Type::None => Cow::Borrowed("<none>"),
            Type::Type(e) => Cow::Owned(format!("<type {}>", e)),
            Type::Any => Cow::Borrowed("<any>"),
            Type::Function => Cow::Borrowed("<function>"),
            Type::Instance(e) => Cow::Owned(format!("<instance of <class {}>>", e)),
            Type::Class(e) => Cow::Owned(format!("<class {}>", e)),
            Type::Namespace => Cow::Borrowed("<namespace>"),
            Type::Struct(e) => Cow::Owned(format!("<struct {}>", e)),
            Type::InstanceStruct => Cow::Borrowed("<instance struct>"),
        })
    }
}



#[macro_export]
macro_rules! get_impl_if_exist {
    ($type:ident, $obj:expr) => {
        $obj.implementations.iter().find_map(|e| {
            if let Implementation::$type(e) = e {
                Some(e.clone())
            } else {
                None
            }
        })
    };
}

