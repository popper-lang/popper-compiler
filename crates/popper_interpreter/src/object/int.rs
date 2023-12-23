use crate::object::{FromPopObject, PopObject, PopOperator};

#[derive(Debug, PartialEq, Clone)]
pub struct PopInt {
    pub value: i64,
}

impl PopInt {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl From<i64> for PopInt {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}


impl FromPopObject for PopInt {
    fn from_pop_object(pop_object: &PopObject) -> Option<&Self> {
        match pop_object {
            PopObject::Int(e) => Some(e),
            _ => None,
        }
    }
}

impl PopOperator for PopInt {
    fn add(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_int(self.value + other.value))
    }
    
    fn sub(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_int(self.value - other.value))
    }
    
    fn mul(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_int(self.value * other.value))
    }
    
    fn div(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_int(self.value / other.value))
    }
    
    fn rem(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_int(self.value % other.value))
    }
    
    fn pow(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_int(self.value.pow(other.value as u32)))
    }
    
    fn eq(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_boolean(self.value == other.value))
    }
    
    fn ne(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_boolean(self.value != other.value))
    }
    
    fn lt(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_boolean(self.value < other.value))
    }
    
    fn gt(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_boolean(self.value > other.value))
    }
    
    fn le(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_boolean(self.value <= other.value))
    }
    
    fn ge(&self, other: PopObject) -> Option<PopObject> {
        let other = other.require::<Self>()?;
        
        Some(PopObject::new_boolean(self.value >= other.value))
    }
}