
fn main() {
    let body = r#"
    if 1 == 1 {
        1
    } else {
        2
    }
    "#;
    popper::execute(body, None);
}
