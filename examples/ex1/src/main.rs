use popper::execute;


fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io;
    let e = 5;

    let k = [93, 43, 21];
    k.push(44);

    io::println(k.3.to_string());


    "#, None));
}