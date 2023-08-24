use popper_lang::popper_compile;

fn main() {
    let s = popper_compile(r#"
    func add(a: int, b: int): int {
        return a + b;
    }

    let x = sum(3 + 5, 4);
    "#, "<main>");

    println!("{}", s);
}