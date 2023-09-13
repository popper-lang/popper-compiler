use std::collections::HashMap;
use popper_ast::{
    BinOp,
    BinOpKind,
    Block,
    Call as AstCall,
    Constant,
    Expression,
    Function,
    If,
    IfElse,
    LetStmt,
    ParenGroup,
    Return,
    Statement,
    While,
    UnaryOp
};
use popper_ast::visitor::{ExprVisitor, StmtVisitor};
use popper_asm::ast::{
    Command,
    Expr,
    Label,
    MemoryFetching,
    Register
};
use popper_asm::ast::Program;
use popper_asm::ast::{
    Mov,
    Call,
    Add,
    Sub,
    Mul,
    Div
};

use popper_builtin::builtins;

pub struct Compiler {
    pub stack: Vec<Expr>,
    pub env: HashMap<String, MemoryFetching>,
    pub program: Vec<Statement>,
    pub asm: Program,
    current_label: Label,
    register_index: u8
}

impl Compiler {
    pub fn new(program: Vec<Statement>) -> Self {
        let mut labels = vec![];
        for builtin in builtins() {
            labels.push(
                Label::new(
                    builtin.lang_name(),
                    builtin.call()
                )
            );
        }
        Self {
            stack: Vec::new(),
            env: Default::default(),
            program,
            asm: Program::new(labels.clone()),
            current_label: Label::new("main".to_string(), vec![]),
            register_index: 0
        }
    }

    pub fn next_register(&mut self) -> Option<Register> {
        self.register_index += 1;
        if self.register_index > 15 {
            return None;
        }
        Some(Register::from(self.register_index))
    }

    pub fn peek_register(&self) -> Option<Register> {
        if self.register_index > 15 {
            return None;
        }
        Some(Register::from(self.register_index))
    }

    pub fn store_expr(&mut self, expr: Expr) {
        let reg = self.next_register().unwrap();
        self.current_label.program.push(Command::Mov(
            Mov(
                MemoryFetching::Register(reg.clone()),
                expr
            )
        ));

        self.stack.push(
            Expr::Memory(MemoryFetching::Register(reg))
        );
    }

    pub fn compile(&mut self) {
        for stmt in self.program.clone() {
            self.visit_stmt(stmt);
        }
        self.asm.labels.push(self.current_label.clone());
    }

}

impl ExprVisitor for Compiler {
    type Output = ();
    type Error = ();

    fn visit_constant(&mut self, constant: Constant) -> Result<Self::Output, Self::Error> {
        match constant {
            Constant::Int(i) => self.store_expr(
                Expr::Int(i.value as i32)
            ),
            Constant::Float(_f) => todo!(),
            Constant::Bool(_b) => todo!(),
            Constant::Null(_) => self.store_expr(
                Expr::Int(0)
            ),
            Constant::Ident(id) => self.store_expr(
                Expr::Memory(
                    self.env.get(&id.name).unwrap().clone()
                )
            ),
            Constant::StringLiteral(_s) => todo!()
        }

        Ok(())
    }

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<Self::Output, Self::Error> {
        self.visit_expr(*bin_op.lhs);
        self.visit_expr(*bin_op.rhs);
        let (rhs, lhs) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
        match bin_op.op {
            BinOpKind::Add => {
                self.current_label.program.push(
                    Command::Add(Add(
                        lhs.expect_memory(),
                        rhs
                    ))
                )
            },
            BinOpKind::Sub => {
                self.current_label.program.push(
                    Command::Sub(Sub(
                        lhs.expect_memory(),
                        rhs
                    ))
                )
            },
            BinOpKind::Mul => {
                self.current_label.program.push(
                    Command::Mul(Mul(
                        lhs.expect_memory()
                    ))
                )
            },
            BinOpKind::Div => {
                self.current_label.program.push(
                    Command::Div(Div(
                        lhs.expect_memory(),
                        rhs
                    ))
                )
            },
            e => todo!("not implemented: {:?}", e)

        }

        Ok(())

    }


    fn visit_unary_op(&mut self, _unary_op: UnaryOp) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_group(&mut self, group: ParenGroup) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_call(&mut self, call: AstCall) -> Result<Self::Output, Self::Error> {
        let label = self.asm.labels.clone().into_iter().find(|x| x.name == call.name).unwrap();
        let reg_state = self.register_index;
        self.register_index = 0;
        for args in call.arguments {
            self.visit_expr(args);
        }

        self.register_index = reg_state;

        self.current_label.program.push(
            Command::Call(
                Call(label.name.clone())
            )
        );

        Ok(())

    }

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        match expr {
            Expression::Constant(c) => self.visit_constant(c),
            Expression::BinOp(b) => self.visit_bin_op(b),
            Expression::UnaryOp(u) => self.visit_unary_op(u),
            Expression::Group(g) => self.visit_group(g),
            Expression::Call(c) => self.visit_call(c),
        }
    }
}

impl StmtVisitor for Compiler {
    type Error = ();
    type Output = ();

    fn visit_function(&mut self, function: Function) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_if_stmt(&mut self, if_stmt: If) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_while_stmt(&mut self, while_stmt: While) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_let_stmt(&mut self, let_stmt: LetStmt) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_block(&mut self, block: Block) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_expr_stmt(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        self.visit_expr(expr)
    }

    fn visit_if_else_stmt(&mut self, if_else_stmt: IfElse) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_return(&mut self, return_expr: Return) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_stmt(&mut self, stmt: Statement) -> Result<Self::Output, Self::Error> {
        match stmt {
            Statement::Function(f) => self.visit_function(f),
            Statement::If(i) => self.visit_if_stmt(i),
            Statement::While(w) => self.visit_while_stmt(w),
            Statement::Let(l) => self.visit_let_stmt(l),
            Statement::Block(b) => self.visit_block(b),
            Statement::IfElse(i) => self.visit_if_else_stmt(i),
            Statement::Return(r) => self.visit_return(r),
            Statement::Expression(e) => self.visit_expr_stmt(e),
        }
    }
}

