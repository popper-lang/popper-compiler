use std::ffi::CString;


#[no_mangle]
pub extern "C" fn print(string: *const u8) -> i32 {
    println!("{}", unsafe { CString::from_raw(string as *mut i8).into_string().unwrap() });
    0
}