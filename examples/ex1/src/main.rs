use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    struct A {
        b: int,
        c: bool
    }

    let d = init A {
        b: 4,
        c: true
    }

    println(d.b)

    "#))
}