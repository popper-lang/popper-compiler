use popper_parser_new::cursor::Cursor;
use popper_parser_new::parse::Parser;
use popper_parser_new::lexer::*;

fn main() {
    let mut cursor = Cursor::new("<=".chars());
    assert_eq!(cursor.clone().len(), 2);
    let got = Le::parse(&mut cursor).unwrap();
    println!("{:?}", got);

}