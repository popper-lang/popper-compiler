use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool

    let b = 8.sqrt()
    io::println(b)

    "#, None));
}