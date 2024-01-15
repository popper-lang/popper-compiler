

use super::*;
use super::If as IfKeyword;
use super::While as WhileKeyword;
use super::Return as ReturnKeyword;
use super::External as ExternalKeyword;
use popper_ast::*;

#[test]
fn test_lt() {
    let mut cursor = Cursor::new("<".chars());
    let got = Lt::parse(&mut cursor).unwrap();
    let expected = Lt {
        span: Span::new(0, 1),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_gt() {
    let mut cursor = Cursor::new(">".chars());
    let got = Gt::parse(&mut cursor).unwrap();
    let expected = Gt {
        span: Span::new(0, 1),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_le() {
    let mut cursor = Cursor::new("<=".chars());
    let got = Le::parse(&mut cursor).unwrap();
    let expected = Le {
        span: Span::new(0, 2),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_ge() {
    let mut cursor = Cursor::new(">=".chars());
    let got = Ge::parse(&mut cursor).unwrap();
    let expected = Ge {
        span: Span::new(0, 2),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_eq() {
    let mut cursor = Cursor::new("==".chars());
    let got = Eq::parse(&mut cursor).unwrap();
    let expected = Eq {
        span: Span::new(0, 2),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_ne() {
    let mut cursor = Cursor::new("!=".chars());
    let got = Ne::parse(&mut cursor).unwrap();
    let expected = Ne {
        span: Span::new(0, 2),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_plus() {
    let mut cursor = Cursor::new("+".chars());
    let got = Plus::parse(&mut cursor).unwrap();
    let expected = Plus {
        span: Span::new(0, 1),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_minus() {
    let mut cursor = Cursor::new("-".chars());
    let got = Minus::parse(&mut cursor).unwrap();
    let expected = Minus {
        span: Span::new(0, 1),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_star() {
    let mut cursor = Cursor::new("*".chars());
    let got = Star::parse(&mut cursor).unwrap();
    let expected = Star {
        span: Span::new(0, 1),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_slash() {
    let mut cursor = Cursor::new("/".chars());
    let got = Slash::parse(&mut cursor).unwrap();
    let expected = Slash {
        span: Span::new(0, 1),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_percent() {
    let mut cursor = Cursor::new("%".chars());
    let got = Percent::parse(&mut cursor).unwrap();
    let expected = Percent {
        span: Span::new(0, 1),
    };
    assert_eq!(got, expected);
}


#[test]
fn test_let() {
    let mut cursor = Cursor::new("let".chars());
    let got = Let::parse(&mut cursor).unwrap();
    let expected = Let {
        span: Span::new(0, 3),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_if() {
    let mut cursor = Cursor::new("if".chars());
    let got = IfKeyword::parse(&mut cursor).unwrap();
    let expected = IfKeyword {
        span: Span::new(0, 2),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_else() {
    let mut cursor = Cursor::new("else".chars());
    let got = Else::parse(&mut cursor).unwrap();
    let expected = Else {
        span: Span::new(0, 4),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_while() {
    let mut cursor = Cursor::new("while".chars());
    let got = WhileKeyword::parse(&mut cursor).unwrap();
    let expected = WhileKeyword {
        span: Span::new(0, 5),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_struct() {
    let mut cursor = Cursor::new("struct".chars());
    let got = Struct::parse(&mut cursor).unwrap();
    let expected = Struct {
        span: Span::new(0, 6),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_func() {
    let mut cursor = Cursor::new("func".chars());
    let got = Func::parse(&mut cursor).unwrap();
    let expected = Func {
        span: Span::new(0, 4),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_return() {
    let mut cursor = Cursor::new("return".chars());
    let got = ReturnKeyword::parse(&mut cursor).unwrap();
    let expected = ReturnKeyword {
        span: Span::new(0, 6),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_import() {
    let mut cursor = Cursor::new("import".chars());
    let got = Import::parse(&mut cursor).unwrap();
    let expected = Import {
        span: Span::new(0, 6),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_exportal() {
    let mut cursor = Cursor::new("external".chars());
    let got = ExternalKeyword::parse(&mut cursor).unwrap();
    let expected = ExternalKeyword {
        span: Span::new(0, 8),
    };
    assert_eq!(got, expected);
}

#[test]
fn test_string_literal() {
    let mut cursor = Cursor::new("\"hello\"".chars());
    let got = StringLiteral::parse(&mut cursor).unwrap();
    let expected = StringLiteral::new(Span::new(0, 7), "hello".to_string());
    assert_eq!(got, expected);
}

#[test]
fn test_ident() {
    let mut cursor = Cursor::new("hello".chars());
    let got = Ident::parse(&mut cursor).unwrap();
    let expected = Ident::new(Span::new(0, 5), "hello".to_string());
    assert_eq!(got, expected);
}

#[test]
fn test_int() {
    let mut cursor = Cursor::new("123".chars());
    let got = Int::parse(&mut cursor).unwrap();
    let expected = Int::new(Span::new(0, 3), 123);
    assert_eq!(got, expected);
}