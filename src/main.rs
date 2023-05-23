use popper_macro::string_to_ident;

macro_rules! a {
    ($d:ident) => {
        fn $d() {
            println!("hello");
        }
    };
}
fn main() {
    a!(string_to_ident!("a"));
    popper::execute(body, None);
}
