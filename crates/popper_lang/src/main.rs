use popper_lang::popper_compile;

fn main() {
    let s = popper_compile(r#"
    3 + 4 + 9;
    "#, "<main>");

    println!("{}", s);
}