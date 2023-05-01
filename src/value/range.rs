use super::Object;
use crate::value::RustValue;
use crate::value::list::list;
use crate::value::int::number;

pub fn range(start: Object, end: Object) -> Object {
    let mut v = vec![];
    let mut i = if let RustValue::Int(n) = start.value {
        n
    } else {
        panic!("Cannot create range from {} to {}", start, end)
    };

    let end = if let RustValue::Int(n) = end.value {
        n
    } else {
        panic!("Cannot create range from {} to {}", start, end)
    };
    while i != end {
        v.push(number(i.clone()));
        i = i + 1;
    }
    list(v)
}