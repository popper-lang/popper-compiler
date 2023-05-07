pub mod class;
pub mod function;
pub mod get;
pub mod instance;
pub mod list;
pub mod int;
pub mod callable;
pub mod operation;
pub mod namespace;
pub mod struct_type;
pub mod range;
pub mod stdlib;
pub mod string;
pub mod boolean;



use std::{
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};
use std::borrow::Cow;
use crate::value::struct_type::BuiltinStruct;

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
    Mod(Rc<dyn operation::Mod>),
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
    pub value: Value,
    pub tags: Vec<Tag>,

}

#[derive(Clone, Default)]
pub enum Tag {
    Mutable,
    Immutable,
    #[default]
    Public,
    Private,
    Return
}



impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.value == other.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
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
    BuiltinStruct(BuiltinStruct),
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*match self {
            Value::Int(e) => Cow::Owned(e.to_string()),
            Value::String(e) => Cow::Borrowed(e.as_str()),
            Value::Bool(e) => Cow::Owned(e.to_string()),
            Value::List(e) => {
                let mut s = String::new();
                s.push('[');
                for i in e {
                    s.push_str(i.to_string().as_str());
                    s.push_str(", ");
                }
                s.push(']');
                Cow::Owned(format!("{:?}", e))
            }
            Value::None => Cow::Borrowed("None"),
            Value::Function => Cow::Borrowed("<function>"),
            Value::Instance(e) => Cow::Owned(format!("<instance of <class {}>>", e.name)),
            Value::Class(e) => Cow::Owned(format!("<class {}>", e.name)),
            Value::Namespace(_) => Cow::Borrowed("<namespace>"),
            Value::Struct(_) => Cow::Borrowed("<struct>"),
            Value::InstanceStruct(_) => Cow::Borrowed("<instance struct>"),
            Value::Type(e) => Cow::Owned(format!("<type {}>", e)),
            Value::BuiltinStruct(_) => Cow::Borrowed("<builtin struct>"),
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


#[macro_export]
macro_rules! impl_into {
    ($type_:ident, $valuename:ident) => {
        impl Into<$type_> for Object {
            fn into(self) -> $type_ {
                if let Value::$valuename(n) = self.value {
                    n
                } else {
                    panic!("Cannot convert {} to {}", self, stringify!($type_))
                }
            }
        }

        impl Into<$type_> for &mut Object {
            fn into(self) -> $type_ {
                if let Value::$valuename(n) = self.value.clone() {
                    n
                } else {
                    panic!("Cannot convert {} to {}", self, stringify!($type_))
                }
            }
        }
    };
}

