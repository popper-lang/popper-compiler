use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool

    let b = "abc".len()
    io::println(b)

    "#, None));
}