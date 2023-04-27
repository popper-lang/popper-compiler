
fn main() {
    let body = r#"
    let a = 5
    let b = 9
    let c = a + b

    fun d(e) {
        let f = 5
        let g = 9
        let h = f + g
        h
    }


    print(a)
    "#;
    dbg!("{}", body.chars().nth(168));
    popper::execute(body);
}
