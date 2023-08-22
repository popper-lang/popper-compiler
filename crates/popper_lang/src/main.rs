use popper_lang::popper_compile;

fn main() {
    let s = popper_compile(r#"
    func add(a: int, b: int): int {
        return a + b;
    }

    add(3, 4);
    "#, "<main>");

    println!("{}", s);
}