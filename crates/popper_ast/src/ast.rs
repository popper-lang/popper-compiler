use crate::attribute::Attribute;
use crate::layer::{Ast, Layer};
use crate::token::TokenKind;
use crate::type_::Type;
use popper_index::Idx;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Span {
        Span { lo, hi }
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }
}

impl From<(usize, usize)> for Span {
    fn from((lo, hi): (usize, usize)) -> Span {
        Span { lo, hi }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LangNodeId(pub u32);

impl Idx for LangNodeId {
    const MAX_ID: usize = u32::MAX as usize;
    const MAX: Self = LangNodeId(u32::MAX);
    fn new(val: usize) -> Self {
        LangNodeId(u32::new(val))
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolId(u32);

impl Idx for SymbolId {
    const MAX_ID: usize = u32::MAX as usize;
    const MAX: Self = SymbolId(u32::MAX);
    fn new(val: usize) -> Self {
        SymbolId(u32::new(val))
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ident(pub SymbolId);

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: Vec::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> SymbolId {
        for (i, symbol) in self.symbols.iter().enumerate() {
            if symbol.name == name {
                return SymbolId(i as u32);
            }
        }

        let symbol = Symbol {
            name: name.to_string(),
        };
        self.symbols.push(symbol);
        SymbolId((self.symbols.len() - 1) as u32)
    }

    pub fn get(&self, id: SymbolId) -> &Symbol {
        &self.symbols[id.0 as usize]
    }
}

#[derive(Debug, Clone)]
pub struct LangAst {
    pub root: LangNodeId,
    pub nodes: Vec<LangNode>,
    symbol_table: SymbolTable,
}

impl LangAst {
    pub fn new() -> LangAst {
        LangAst {
            root: LangNodeId(0),
            nodes: Vec::new(),
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn add_symbol(&mut self, name: &str) -> SymbolId {
        self.symbol_table.intern(name)
    }

    pub fn get_symbol(&self, id: SymbolId) -> &Symbol {
        self.symbol_table.get(id)
    }

    pub fn set_root(&mut self, id: LangNodeId) {
        self.root = id;
    }

    pub fn get(&self, id: LangNodeId) -> &LangNode {
        &self.nodes[id.0 as usize]
    }

    pub fn get_mut(&mut self, id: LangNodeId) -> &mut LangNode {
        &mut self.nodes[id.0 as usize]
    }

    pub fn dumps(&self) -> String {
        let node = self.get(self.root);
        self.dumps_node(node)
    }

    fn dumps_node(&self, node: &LangNode) -> String {
        match &node.kind {
            LangNodeKind::Expr(expr) => self.dumps_expr(expr),
            LangNodeKind::Let(let_) => format!(
                "Let {} = {}",
                self.dumps_symbol(let_.name.0),
                self.dumps_node(self.get(let_.value))
            ),
            LangNodeKind::Block(nodes) => {
                let mut result = String::new();
                for &id in nodes {
                    result.push_str(&self.dumps_node(self.get(id)));
                    result.push('\n');
                }
                result
            }
            LangNodeKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut result = format!(
                    "If {} then {} ",
                    self.dumps_node(self.get(*condition)),
                    self.dumps_node(self.get(*then_branch))
                );
                if let Some(else_id) = else_branch {
                    result.push_str(&format!("else {}", self.dumps_node(self.get(*else_id))));
                }
                result
            }
            LangNodeKind::FunctionCall { function, args } => {
                let func_str = self.dumps_node(self.get(*function));
                let args_str: Vec<String> = args
                    .iter()
                    .map(|&id| self.dumps_node(self.get(id)))
                    .collect();
                format!("{}.call([{}])", func_str, args_str.join(", "))
            }
            LangNodeKind::Return(expr) => format!("Return({})", self.dumps_node(self.get(*expr))),
            LangNodeKind::FunctionDef {
                name,
                attrs,
                params,
                ret,
                body,
                is_expr,
            } => {
                let params_str: Vec<String> = params
                    .iter()
                    .map(|param| format!("{}: {:?}", param.name.0.index(), param.ty))
                    .collect();
                let attr_str: Vec<String> = attrs.iter().map(|x| self.dump_attribute(*x)).collect();
                let s = if let Some(body) = body {
                    if *is_expr {
                        format!("= {}", self.dumps_node(self.get(*body)))
                    } else {
                        format!("{}", self.dumps_node(self.get(*body)))
                    }
                } else {
                    ";".to_string()
                };
                format!(
                    "FunctionDef({}) {}({}) -> {:?} {}",
                    attr_str.join(","),
                    self.dumps_symbol(name.0),
                    params_str.join(", "),
                    ret,
                    s
                )
            }
        }
    }

    fn dump_attribute(&self, attribute: Attribute) -> String {
        match attribute {
            Attribute::StdCallC => "C".to_string(),
        }
    }

    fn dumps_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Ident(ident) => format!("Ident({})", self.dumps_symbol(ident.0)),
            Expr::Int(value) => format!("Int({})", value),
            Expr::String(value) => format!("String({})", value),
            Expr::UnaryOp(op, node_id) => {
                let op_str = match op {
                    UnaryOpKind::Negate => "!",
                    UnaryOpKind::ArithmeticNegate => "-",
                    UnaryOpKind::ArithmeticPlus => "+",
                };
                format!(
                    "UnaryOp({}, {})",
                    op_str,
                    self.dumps_node(self.get(*node_id))
                )
            }
            Expr::Add(lhs, rhs) => format!(
                "Add({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::Sub(lhs, rhs) => format!(
                "Sub({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::Mul(lhs, rhs) => format!(
                "Mul({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::Div(lhs, rhs) => format!(
                "Div({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
        }
    }

    fn dumps_symbol(&self, id: SymbolId) -> String {
        self.symbol_table.get(id).name.clone()
    }
}

impl Ast for LangAst {
    type NodeId = LangNodeId;
    type Node = LangNode;

    fn add(&mut self, node: Self::Node) -> Self::NodeId {
        let id = LangNodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    fn get(&self, node: Self::NodeId) -> &Self::Node {
        &self.nodes[node.0 as usize]
    }

    fn get_mut(&mut self, node: Self::NodeId) -> &mut Self::Node {
        &mut self.nodes[node.0 as usize]
    }

    fn nodes(&self) -> impl Iterator<Item = Self::NodeId> {
        (0..self.nodes.len()).map(|i| LangNodeId(i as u32))
    }

    fn root(&self) -> Self::NodeId {
        self.root
    }
}

#[derive(Debug, Clone)]
pub struct LangNode {
    pub kind: LangNodeKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LangNodeKind {
    Expr(Expr),
    Let(Let),
    Block(Vec<LangNodeId>),
    If {
        condition: LangNodeId,
        then_branch: LangNodeId,
        else_branch: Option<LangNodeId>,
    },
    FunctionCall {
        function: LangNodeId,
        args: Vec<LangNodeId>,
    },
    Return(LangNodeId),
    FunctionDef {
        name: Ident,
        attrs: Vec<Attribute>,
        params: Vec<ArgumentParamDef>,
        ret: Type,
        body: Option<LangNodeId>,
        is_expr: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentParamDef {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpKind {
    Negate,           // Negation (logical negation)
    ArithmeticPlus,   // Arithmetic plus (+3)
    ArithmeticNegate, // Arithmetic negation (-3)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Int(i64),
    String(String),
    UnaryOp(UnaryOpKind, LangNodeId),
    Add(LangNodeId, LangNodeId),
    Sub(LangNodeId, LangNodeId),
    Mul(LangNodeId, LangNodeId),
    Div(LangNodeId, LangNodeId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Let {
    pub name: Ident,
    pub value: LangNodeId,
}

#[derive(Debug, Clone)]
pub struct LineInfo {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl LineInfo {
    pub fn from_span(span: Span, line: usize) -> LineInfo {
        LineInfo::new(line, span.lo, span.hi)
    }

    pub fn new(line: usize, start: usize, end: usize) -> LineInfo {
        LineInfo { line, start, end }
    }
}

impl From<&str> for LineInfo {
    fn from(s: &str) -> LineInfo {
        LineInfo::new(0, 0, s.len())
    }
}
