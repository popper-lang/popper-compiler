
fn main() {
    let body = r#"
    let a = 5
    let b = 9
    a + b

    fun hello(a) {
        let b = 5
        a + b
    }

    hello(5)

    "#;
    dbg!("{}", body.chars().nth(168));
    popper::execute(body);
}
