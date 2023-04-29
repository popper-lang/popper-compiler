
fn main() {
    let body = r#"
    use "io.pop" as io
    io::println("Hello world")

    "#;
    dbg!("{}", body.chars().nth(168));
    popper::execute(body, None);
}
