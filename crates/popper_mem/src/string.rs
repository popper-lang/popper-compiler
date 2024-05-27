use std::borrow::Cow;
use std::ffi::{CStr, CString};
// use crate::array::RawArray;

pub fn to_c_str<'s>(mut s: &'s str) -> Cow<'s, CStr> {
    if s.is_empty() {
        s = "\0";
    }

    // Start from the end of the string as it's the most likely place to find a null byte
    if !s.chars().rev().any(|ch| ch == '\0') {
        let ptr = Cow::from(CString::new(s).expect("unreachable since null bytes are checked"));
        return ptr;
    }

    let ptr  = unsafe { Cow::from(CStr::from_ptr(s.as_ptr() as *const _)) };
    ptr

}
