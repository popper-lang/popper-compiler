use crate::error::ParserError;
use crate::Parser;
use popper_ast::token::TokenKind;
use popper_ast::type_::Type;

impl Parser {
    pub(crate) fn parse_ty(&mut self) -> crate::error::Result<Type> {
        let token = self.cursor.peek_token()?;
        match token.kind {
            TokenKind::TypeInt => {
                self.cursor.next_token()?;
                Ok(Type::Int)
            }
            TokenKind::TypeFloat => {
                self.cursor.next_token()?;
                Ok(Type::Float)
            }
            TokenKind::TypeBool => {
                self.cursor.next_token()?;
                Ok(Type::Bool)
            }
            TokenKind::TypeChar => {
                self.cursor.next_token()?;
                Ok(Type::Char)
            }
            TokenKind::TypeString => {
                self.cursor.next_token()?;
                Ok(Type::Ptr(Box::new(Type::Char)))
            }
            TokenKind::TypeVoid => {
                self.cursor.next_token()?;
                Ok(Type::Void)
            }
            TokenKind::TypeType => {
                self.cursor.next_token()?;
                Ok(Type::Type(None))
            }
            TokenKind::Bang => {
                let _ = self.cursor.next_token()?;
                let inner = self.parse_ty()?;
                Ok(Type::Ptr(Box::new(inner)))
            }
            TokenKind::BracketL => {
                let _ = self.cursor.next_token()?;
                let inner = self.parse_ty()?;
                self.expect(TokenKind::Colon)?;
                let size = self.expect(TokenKind::Number)?;
                self.expect(TokenKind::BracketR)?;
                let size = size.value.parse().unwrap();
                Ok(Type::List(Box::new(inner), size))
            }
            TokenKind::Identifier => {
                self.cursor.next_token()?;
                Ok(Type::AggregateType(
                    token.value.clone(),
                    popper_ast::type_::Fields::new(),
                    popper_ast::ast::TypeDeclKind::Struct,
                ))
            }
            _ => Err(crate::parse_error!(
                expect tokens [TypeInt, TypeFloat, TypeBool, TypeString, TypeVoid, TypeType, BracketL, Identifier] but got (token)
            )),
        }
    }
}
