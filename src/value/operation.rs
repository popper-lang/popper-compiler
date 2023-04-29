use std::cmp::Ordering;
use crate::value::Object;

pub trait PartialEq {
    fn eq(&self, other: Object) -> bool;
    fn ne(&self, other: Object) -> bool { !self.eq(other) }
}

pub trait PartialOrd {
    fn partial_cmp(&self, other: Object) -> Option<Ordering>;
    fn lt(&self, other: Object) -> bool { self.partial_cmp(other) == Some(Ordering::Less) }
    fn le(&self, other: Object) -> bool { self.partial_cmp(other) != Some(Ordering::Greater) }
    fn gt(&self, other: Object) -> bool { self.partial_cmp(other) == Some(Ordering::Greater) }
    fn ge(&self, other: Object) -> bool { self.partial_cmp(other) != Some(Ordering::Less) }
}

pub trait Add {
    fn add(&self, other: Object) -> Object;
}

pub trait Sub {
    fn sub(&self, other: Object) -> Object;
}

pub trait Mul {
    fn mul(&self, other: Object) -> Object;
}

pub trait Div {
    fn div(&self, other: Object) -> Object;
}

pub trait Pow {
    fn pow(&self, other: Object) -> Object;
}

pub trait Mod {
    fn modulo(&self, other: Object) -> Object;
}