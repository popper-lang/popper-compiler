use popper_lang::popper_compile;

fn main() {
    let s = popper_compile(r#"
    func hello(name: int): string {
        name + 8;
    }
    "#, "<main>");

    println!("{}", s);
}