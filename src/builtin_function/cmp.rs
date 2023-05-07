use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, Value, Type};
use std::rc::Rc;

use crate::value::callable::Callable;
use crate::get_impl_if_exist;
use crate::value::int::{none, number};
use crate::value::string::string;
use crate::value::boolean::boolean;
use crate::{create, call_function_with_vec}; // File : src/builtin_function/mod.rs
use super::panic_if_is_outside_std;
use crate::define_function;


define_function!(IsEqual(left: Object, right: Object) {
    if left.type_ != right.type_ {
        return boolean(false)
    } else {
        return boolean(left.value == right.value)
    }
}, function_name = "_is_equal");

define_function!(IsNotEqual(left: Object, right: Object) {
    if left.type_ != right.type_ {
        return boolean(true)
    } else {
        return boolean(left.value != right.value)
    }
}, function_name = "_is_not_equal");

