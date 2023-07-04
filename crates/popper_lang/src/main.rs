use popper_lang::popper_compile;

fn main() {
    let s = popper_compile(r#"
    1 + 2;
    while true {
        3 + 4;
    }
    "#, "<main>");

    println!("{}", s);
}