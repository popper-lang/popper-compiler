use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool

    let b = (a) { a + 1 }
    let c = (a) { a + 2 }

    io::println(b(3))
    itertool::map((n) { n * 3 }, 1:10)

    "#, None));
}