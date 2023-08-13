use popper_lang::popper_compile;

fn main() {
    let s = popper_compile(r#"
    while true {
        1 + 2;
    }
    3 + 4;
    "#, "<main>");

    println!("{}", s);
}