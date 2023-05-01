use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool

    fun v(s) {
        s * 3
    }
    let d = itertool::map(v, [1, 2, 3])
    io::println(d)
    "#, None));
}