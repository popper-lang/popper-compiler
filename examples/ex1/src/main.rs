use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool

    let b = (a) { a + 1 }
    let c = (a) { a + 2 }
    io::println(b(5))
    io::println(
        itertool::map((a) { a * 2 }, [1, 2, 3, 4, 5])
    )
    "#, None));
}