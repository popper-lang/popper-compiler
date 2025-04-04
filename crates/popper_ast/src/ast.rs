use popper_index::Idx;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,

}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Span {
        Span { lo, hi }
    }
}

impl From<(usize, usize)> for Span {
    fn from((lo, hi): (usize, usize)) -> Span {
        Span { lo, hi }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NodeId(u32);


impl Idx for NodeId {
    const MAX_ID: usize = u32::MAX as usize;
    const MAX: Self = NodeId(u32::MAX);
    fn new(val: usize) -> Self {
        NodeId(u32::new(val))
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
pub struct Ident {
    symbol: SymbolId,
}

impl Ident {
    pub fn new(symbol: SymbolId) -> Ident {
        Ident { symbol }
    }
}

pub struct Symbol {
    name: String,
}

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
pub struct Ast {
    pub root: NodeId,
    pub nodes: Vec<Node>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            root: NodeId(0),
            nodes: Vec::new(),
        }
    }

    pub fn add(&mut self, node: Node) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    pub fn get(&self, id: NodeId) -> &Node {
        &self.nodes[id.0 as usize]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0 as usize]
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}


#[derive(Debug, Clone)]
pub enum NodeKind {
    Expr(Expr),
    Function(Function),
    Block(Vec<NodeId>),

}
#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Int(i64),
    Add(NodeId, NodeId),
    Sub(NodeId, NodeId),
    Mul(NodeId, NodeId)
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Ident,
    pub body: NodeId,
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
