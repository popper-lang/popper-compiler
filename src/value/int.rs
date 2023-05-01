use std::fmt::Display;
use super::{Object, Type, Implementation};
use std::rc::Rc;
use crate::value::RustValue;
use crate::value::operation::{Add, Sub, Mul, Div, Mod, PartialOrd, PartialEq};
use crate::value::list::PopperList;


#[derive(Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Copy)]
pub struct PopperInt {
    value: i32
}

impl Display for PopperInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<PopperInt> for i32 {
    fn from(value: PopperInt) -> Self {
        value.value
    }
}

impl From<i32> for PopperInt {
    fn from(value: i32) -> Self {
        Self {
            value
        }
    }
}


pub fn number(n: PopperInt) -> Object {
    Object {
        type_: Type::Int,
        implementations: vec![
            Implementation::Add(Rc::new(n)),
            Implementation::Sub(Rc::new(n)),
            Implementation::Mul(Rc::new(n)),
            Implementation::Div(Rc::new(n)),
            Implementation::Mod(Rc::new(n)),
            Implementation::PartialEq(Rc::new(n)),
            Implementation::PartialOrd(Rc::new(n)),
        ],
        value: RustValue::Int(n)
    }
}

impl Add for PopperInt {
    fn add(&self, other: Object) -> Object {
        if let RustValue::Int(ref n) = other.value {
            number((self.value + n.value).into())
        } else {
            panic!("Cannot add {} to {}", self, other)
        }
    }
}

impl Sub for PopperInt {
    fn sub(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number((self.value - n.value).into())
        } else {
            panic!("Cannot substract {} to {}", self, other)
        }
    }
}

impl Mul for PopperInt {
    fn mul(&self, other: Object) -> Object {
        println!("BREAK 13");
        if let RustValue::Int(n) = other.value {
            println!("BREAK 14");
            let k = number((self.value * n.value).into());
            println!("BREAK 15");
            k
        } else {
            panic!("Cannot multiply {} to {}", self, other)
        }
    }
}

impl Div for PopperInt {
    fn div(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number((self.value / n.value).into())
        } else {
            panic!("Cannot divide {} to {}", self, other)
        }
    }
}



impl Mod for PopperInt {
    fn modulo(&self, rhs: Object) -> Object {
        if let RustValue::Int(n) = rhs.value {
            number((self.value % n.value).into())
        } else {
            panic!("Cannot modulo {} to {}", self, rhs)
        }
    }
}

impl PartialEq for PopperInt {
    fn eq(&self, other: Object) -> bool {
        if let RustValue::Int(ref n) = other.value {
            self == n
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}

impl PartialOrd for PopperInt {
    fn partial_cmp(&self, other: Object) -> Option<std::cmp::Ordering> {
        if let RustValue::Int(ref n) = other.value {
            if self.value < n.value {
                Some(std::cmp::Ordering::Less)
            } else if self.value > n.value {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}

pub fn none() -> Object {
    Object {
        type_: Type::None,
        implementations: vec![],
        value: RustValue::None
    }
}