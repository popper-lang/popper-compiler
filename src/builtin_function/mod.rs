pub mod io;
pub mod cmp;
pub mod list_util;

use crate::interpreter::STD_LIB_PATH;
use std::path::Path;


pub fn panic_if_is_outside_std(path: &str, function_name: &str) {
    let std_path = Path::new(STD_LIB_PATH);
    let path = Path::new(path);

    if ! path.starts_with(std_path) {
        panic!("You can't import the {} function from outside the standard library.", function_name);
    }
}