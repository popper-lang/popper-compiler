use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool


    fun fib(n) {
        if n < 2 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    io::println(
        fib(10)
    )
    io::println(
        itertool::map(fib, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    )


    "#, None));
}