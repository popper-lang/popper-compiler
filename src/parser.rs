use crate::lexer::Token;
use crate::lexer::Keyword;
use crate::lexer::Seperator;
use crate::lexer::Operator;
use crate::lexer::Identifier;

use crate::tree::Expr;
use crate::tree::Op;

trait PartialExpr  {
    type Error;
    fn to_partial_expr<T: PartialExpr>(tokens: Vec<Token>) -> Result<T, Self::Error>;
    fn to_expr(self) -> Result<Expr, Self::Error>;
    fn can_expr(self) -> bool {
        false
    };
}


struct LetPartialExpr;
struct LetIdPartialExpr {
    name: String
};
struct<T: PartialExpr> LetEqualPartialExpr{
    parent: LetEqualsPartialExpr,
    value: T
};

impl PartialExpr for LetPartialExpr {
    type Error = String;
    fn to_partial_expr<T: PartialExpr>(tokens: Vec<Token>) -> Result<LetIdPartialExpr, Self::Error> {
        if tokens.len() > 1 {
            return Err("not enough tokens".to_string());
        }

        if let Keyword::Ident(name) = tokens[0] {
            return LetIdPartialExpr {
                name
            };

        } else {
            return Err("not an identifier".to_string());
        }
    }
}

impl PartialExpr for LetIdPartialExpr {
    type Error = String;
    fn to_partial_expr<T: PartialExpr>(&self, tokens: Vec<Token>) -> Result<LetEqualPartialExpr, Self::Error> {
        if tokens.len() > 1 {
            return Err("not enough tokens".to_string());
        }
        if tokens[0] == Token::Operator(Operator::Equal) {
            return LetEqualPartialExpr {
                parent: self 
            };
        } else {
            return Err("not an equal sign".to_string());
        }
    }
}

impl PartialExpr for LetEqualPartialExpr {
    type Error = String;
    fn can_expr(self) -> bool {
        true
    }
    fn to_partial_expr<T: PartialExpr>(&self, tokens: Vec<Token>) -> Result<T, Self::Error> {
        return T::to_partial_expr(tokens);
    }
    fn to_expr(self) -> Result<Expr, Self::Error> {
        Ok(Expr::Let {
            name: self.parent.name,
            value: self.value.to_expr()?
        })



        







