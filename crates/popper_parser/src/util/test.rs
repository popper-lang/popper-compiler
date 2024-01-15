use popper_ast::Span;
use crate::cursor::Cursor;
use crate::lexer::{Lt, Gt, Space};
use crate::parse::Parser;
use super::or::Or;
use super::ignore::Ignore;

#[test]
fn test_or() {
    let mut cursor = Cursor::new(">".chars());
    let res = Or::<Lt, Gt, _>::parse(&mut cursor);
    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res.span, Span::new(0, 1));
    assert!(res.p1.is_none());
    assert!(res.p2.is_some());
    let res = res.p2.unwrap();
    assert_eq!(res.span, Span::new(0, 1));
    assert_eq!(res.to_string(), "Gt");
}

#[test]
fn test_ignore() {
    let mut cursor = Cursor::new("  > ".chars());
    let res = Ignore::<Space, _>::parse(&mut cursor);

    assert_eq!(cursor.collect(), vec!['>']);

}