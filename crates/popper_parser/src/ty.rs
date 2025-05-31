use popper_ast::type_::Type;
use popper_ast::token::TokenKind;
use crate::Parser;
use crate::error::ParserError;

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
            TokenKind::TypeString => {
                self.cursor.next_token()?;
                Ok(Type::String)
            }
            TokenKind::TypeVoid => {
                self.cursor.next_token()?;
                Ok(Type::Void)
            }
            _ => {
                Err(crate::parse_error!(
                    expect tokens [TypeInt, TypeFloat, TypeBool, TypeString, TypeVoid] but got (token)
                ))
            }
        }
    }
}