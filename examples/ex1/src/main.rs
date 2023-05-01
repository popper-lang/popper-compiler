use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    

    let b = (a) { a + 1 }
    let c = (a) { a + 2 }
    io::println(b(5))
    io::println(
        itertool::map((a) { a * 2 }, [1, 2, 3, 4, 5])
    )
    "#, None));
}