
use popper::execute;


fn main() {
    execute(r#"
    let a = 5
    {
        let b = 4
        println(b)
    }
    println(b)
"#)
}