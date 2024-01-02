
#[no_mangle]
pub extern "C" fn example_external(a: i32, b: i32) -> i32 {
    return a + b + (a * b) + 1;
}


#[no_mangle]
pub extern "C" fn print_int(a: i32) -> i32 {
    println!("{}", a);
    return a;
}