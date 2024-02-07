use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn println(string: *const u8) -> i32 {
    println!("{}", unsafe { CStr::from_ptr(string as *mut i8).to_str().unwrap() });
    0
}

#[no_mangle]
pub extern "C" fn println_int(int: i32) -> i32 {
    print!("{}", int);
    0
}

#[no_mangle]
pub extern "C" fn print(string: *const u8) -> i32 {
    print!("{}", unsafe { CStr::from_ptr(string as *mut i8).to_str().unwrap() });
    0
}

#[no_mangle]
pub extern "C" fn input(msg: *const u8) -> *const u8 {
    println(msg);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.to_string();
    let input = input.into_bytes();
    let input = input.as_ptr();
    input
}
