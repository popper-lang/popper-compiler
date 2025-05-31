use popper_ast::ast::{ArgumentParamDef, Ident, LangNode, LangNodeId, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::type_::Type;
use popper_ast::token::TokenKind;
use crate::Parser;

impl Parser {
    
    pub(crate) fn parse_function_stmt(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordFunc)?;
        let mut attributes = vec![];
        if self.match_token(TokenKind::ParenL) {
            let mut i = 0;
            while !self.match_token(TokenKind::ParenR) {
                if i > 0 {
                    self.expect(TokenKind::Comma)?;
                }
                let attribute = self.parse_attribute()?;
                attributes.push(attribute);
                i += 1;
            }
        }
        let identifier = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::ParenL)?; // Expect opening parenthesis for parameters
        let mut parameters = Vec::new();
        let mut i = 0;
        while self.cursor.peek_token()?.kind != TokenKind::ParenR {
            if i > 0 {
                self.expect(TokenKind::Comma)?;
            }
            let param = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_ty()?;
            let ident = Ident(self.ast.add_symbol(&param.value));
            parameters.push(
                ArgumentParamDef {
                    name: ident,
                    ty,
                }
            );
            
            i += 1;
        }
        
        self.expect(TokenKind::ParenR)?; 
        let return_type = if self.match_token(TokenKind::Arrow) {
            self.parse_ty()?
        } else {
            Type::Void
        };
        let (body, is_expr) = if self.match_token(TokenKind::Eq) {
            (Some(self.parse_expr()?), true)
        } else if self.match_token(TokenKind::Semicolon) {
            (None, false)
        } else {
            (Some(self.parse_block()?), false)
        };
        let end = self.cursor.pos();
        
        let node = LangNode {
            kind: LangNodeKind::FunctionDef {
                name: Ident(self.ast.add_symbol(&identifier.value)),
                attrs: attributes,
                params: parameters,
                ret: return_type,
                body,
                is_expr
            },
            span: Span::new(start.span.lo, end),
        };
        
        let id = self.ast.add(node);
        Ok(id)
        
    }
    
    pub(crate) fn parse_return(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordReturn)?;
        let expr = self.parse_expr()?;
        let end = self.expect(TokenKind::Semicolon)?;
        
        let node = LangNode {
            kind: LangNodeKind::Return(expr),
            span: start.span.merge(end.span)
        };
        
        Ok(self.ast.add(node))
    }
    
    pub(crate) fn parse_function_call(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.cursor.pos();
        let mut literal = self.parse_literal()?;
        if self.match_token(TokenKind::ParenL) {
            let mut arguments = Vec::new();
            let mut i = 0;
            while !self.match_token(TokenKind::ParenR) {
                if i > 0 {
                    self.expect(TokenKind::Comma)?;
                }
                
                let arg = self.parse_expr()?;
                arguments.push(arg);
                i += 1;
            }
            
            let end = self.cursor.pos();
            
            let node = LangNode {
                kind: LangNodeKind::FunctionCall {
                    function: literal,
                    args: arguments
                },
                span: Span::new(start, end),
            };
            literal = self.ast.add(node);
        }
        Ok(literal)
    }
}