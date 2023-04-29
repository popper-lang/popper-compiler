use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io
    use "itertool.pop" as itertool

    fun syracus(n) {
        if n == 1 {
            n
        } else if n % 2 == 0 {
                syracus(n / 2)
        } else {
            syracus(3 * n + 1)
        }

    }

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
        syracus(10)
    )
    io::println(
        itertool::map(fib, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    )

    io::println(
        itertool::map(syracus, 1:10)
    )

    io::println(
        0:10
    )


    "#, None));
}