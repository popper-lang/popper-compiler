use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool



    struct A {
        a: int,
        b: int,
        c: int
    }

    impl A {
        fun d(k) {
            this.b * k
        }
    }

    let f = init A {
        a: 26272,
        b: 2,
        c: 3
    }



    let g = f.d(5)

    io::println(g)



    "#, None));
}