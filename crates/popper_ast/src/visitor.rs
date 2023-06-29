use crate::*;

macro_rules! visit {
    ($name:ident, $($field_name: ident => $field_ty:ty),*) => {
        fn $name(&mut self, $($field_name: $field_ty),*) -> Result<Self::Output, Self::Error>;
    };
}


pub trait ExprVisitor {
    type Output;
    type Error;

    visit!(visit_constant, constant => Constant );
    visit!(visit_bin_op, bin_op => BinOp );
    visit!(visit_unary_op, unary_op => UnaryOp );
    visit!(visit_group, group => ParenGroup );
    visit!(visit_expr, expr => Expression );
}

pub trait StmtVisitor {
    type Output;
    type Error;

    visit!(visit_expr_stmt, expr => Expression);
    visit!(visit_let_stmt, let_stmt => LetStmt);
    visit!(visit_stmt, stmt => Statement);
    visit!(visit_block, block => Block);
    visit!(visit_while_stmt, while_stmt => While);
}