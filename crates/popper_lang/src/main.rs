use popper_lang::popper_compile;

fn main() {
    let s = popper_compile(r#"
    func sum(a: int, b: int) : int {
        return a + b;
    }

    let c = sum(3, 4);
    "#, "<main>");

    println!("{}", s);
}