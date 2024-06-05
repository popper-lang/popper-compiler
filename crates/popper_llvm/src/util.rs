use std::borrow::Cow;
use std::ffi::{CStr, CString};


trait IsNull {
    fn _is_null(&self) -> bool;
}

impl<T> IsNull for *const T {
    fn _is_null(&self) -> bool {
        self.is_null()
    }
}

impl<T> IsNull for *mut T {
    fn _is_null(&self) -> bool {
        self.is_null()
    }
}


pub fn ptr_to_option<T: IsNull>(ptr: T) -> Option<T> {
    if ptr._is_null() {
        None
    } else {
        Some(ptr)
    }
}


pub fn to_c_str(mut s: &str) -> Cow<CStr> {
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
