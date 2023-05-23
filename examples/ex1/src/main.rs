use popper::execute;

fn main() {
    println!("{:?}", execute(r#"
    use "io.pop" as io;
    let e = 5;
    for i in [3, 4, 5] {
        io::print(i.to_string());
    }

    "#, None));
}