

use ast::*;
use crate::errors::TypeMismatch;
use popper_flag::{
    ScopeFlag,
    Flag,
    TypeFlag,
    VariableFlag,
    Environment,
    SymbolFlags,
};


pub struct StmtAnalyzer {
    env: Environment
}

impl StmtAnalyzer {
    pub fn new(env: Environment) -> Self {
        Self { env }
    }
}

impl StmtVisitor for StmtAnalyzer {
    type Output = SymbolFlags;
    type Error = Box<dyn Error>;
    fn visit_expr(&self, expr: Expression) -> Result<Self::Output, Self::Error> {
        let mut analyzer = ExprAnalyzer::new(self.env);

        analyzer.visit_expr(expr)

    }
}

