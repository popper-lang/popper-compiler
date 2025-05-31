use popper_ast::attribute::Attribute;
use popper_ast::token::TokenKind;
use crate::Parser;

impl Parser {
    pub(crate) fn parse_attribute(&mut self) -> crate::error::Result<Attribute> {
        let ident = self.expect(TokenKind::Identifier)?;
        match ident.value.as_str() { 
            "C" => Ok(Attribute::StdCallC),
            _ => {
                Err(crate::parse_error!(expect a valid attribute but got (ident)))
            }
        }
    }
}