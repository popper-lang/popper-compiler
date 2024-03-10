use crate::command::CommandEnum;
use crate::consts::Ident;
use crate::expr::Expr;
use crate::function::Function;
use crate::types::Types;

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StmtKind,
}

impl Statement {
    pub fn new(kind: StmtKind) -> Self {
        Self { kind }
    }
    pub fn get_kind(&self) -> &StmtKind {
        &self.kind
    }

    pub fn new_let_decl(ident: Ident, ty: Types) -> Self {
        Self {
            kind: StmtKind::LetDecl(LetDecl::new(ident, ty))
        }
    }

    pub fn new_assign(ident: Ident, command: CommandEnum) -> Self {
        Self {
            kind: StmtKind::Assign(Assign::new(ident, command))
        }
    }

    pub fn new_command(command: CommandEnum) -> Self {
        Self {
            kind: StmtKind::Command(command)
        }
    }
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    LetDecl(LetDecl),
    Assign(Assign),
    Command(CommandEnum)
}

#[derive(Debug, Clone)]
pub struct LetDecl {
    pub ident: Ident,
    pub ty: Types
}

impl LetDecl {
    pub fn new(ident: Ident, ty: Types) -> Self {
        Self {
            ident,
            ty
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub ident: Ident,
    pub command: CommandEnum
}

impl Assign {
    pub fn new(ident: Ident, command: CommandEnum) -> Self {
        Self {
            ident,
            command
        }
    }
}
