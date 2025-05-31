use crate::tokenize;
use popper_ast::token::{Token, TokenKind};

#[test]
fn test_identifier() {
    let input = "foo";
    let tokens = tokenize(input).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].value, "foo");
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_number() {
    let input = "123";
    let tokens = tokenize(input);
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].value, "123");
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_string() {
    let input = "\"foo\"";
    let tokens = tokenize(input);
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, "foo");
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_keyword() {
    let input = "let";
    let tokens = tokenize(input);
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::KeywordLet);
    assert_eq!(tokens[0].value, "let");
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}
