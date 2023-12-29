use std::ffi::CString;


extern "C" {
    fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
pub extern "C" fn print(string: *const u8) -> i32 {
    unsafe {
        let c_str = CString::new("%s\n").unwrap();
        printf(c_str.as_ptr() as *const u8, string);
    }
    0
}