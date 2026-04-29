use crate::attribute::Attribute;
use crate::file::SourceFileInfo;
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
        // check if the symbol already exists
        for (i, symbol) in self.symbol_table.symbols.iter().enumerate() {
            if symbol.name == name {
                return SymbolId(i as u32);
            }
        }
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

    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
    /*
    pub fn dumps(&self) -> String {
        let node = self.get(self.root);
        self.dumps_node(node)
    }

    pub fn dumps_nodes(&self, nodes: &[LangNodeId]) -> String {
        let mut result = String::new();
        for &id in nodes {
            result.push_str(&self.dumps_node(self.get(id)));
        }
        result
    }

    fn dumps_node(&self, node: &LangNode) -> String {
        match &node.kind {
            LangNodeKind::Expr(expr) => self.dumps_expr(expr),
            LangNodeKind::Assign { lhs, rhs } => {
                format!(
                    "{} = {}",
                    self.dumps_node(self.get(*lhs)),
                    self.dumps_node(self.get(*rhs))
                )
            }
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
            LangNodeKind::While { condition, body } => {
                format!(
                    "While {} do {}",
                    self.dumps_node(self.get(*condition)),
                    self.dumps_node(self.get(*body))
                )
            }
            LangNodeKind::FunctionDef {
                name,
                attrs,
                params,
                ret,
                body,
                is_expr,
                is_vararg: _,
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
            LangNodeKind::TypeDecl { name, kind, fields } => {
                let fields_str: Vec<String> = fields
                    .iter()
                    .map(|field| format!("{}: {:?}", field.name.0.index(), field.ty))
                    .collect();
                format!(
                    "{} {} {{ {} }}",
                    match kind {
                        TypeDeclKind::Struct => "Struct",
                        TypeDeclKind::Union => "Union",
                    },
                    self.dumps_symbol(name.0),
                    fields_str.join(", ")
                )
            }
            LangNodeKind::Const(c) => {
                format!(
                    "Const {}: {:?} = {}",
                    self.dumps_symbol(c.name.0),
                    c.ty,
                    self.dumps_node(self.get(c.value))
                )
            }
            LangNodeKind::MacroDef(m) => {
                format!("MacroDef {}(...) {{ ... }}", self.dumps_symbol(m.name.0))
            }
            LangNodeKind::Comptime(n) => {
                format!("Comptime {{ {} }}", self.dumps_node(self.get(*n)))
            }
            LangNodeKind::DeDecl {
                target_type,
                methods,
            } => {
                format!(
                    "ExtendDecl {{ target_type: {}, methods: [{}] }}",
                    target_type,
                    self.dumps_nodes(methods)
                )
            }
            LangNodeKind::ContextDecl { name, extensions } => {
                format!("ContextDecl {{ name:")
            }
            LangNodeKind::RequireContextDecl { name, items } => {
                format!("RequireContextDecl {{ name: {}, items: [{}] }}", self.dumps_symbol(name.0), self.dumps_nodes(items))
            }

            LangNodeKind::ImportDecl { path, context_mappings } => {
                format!("ImportDecl {{ path: {}, context_mappings: {:?} }}", path, context_mappings)
            }
        }
    }

    fn dump_attribute(&self, attribute: Attribute) -> String {
        match attribute {
            Attribute::StdCallC => "C".to_string(),
            Attribute::Comptime => "comptime".to_string(),
        }
    }

    fn dumps_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Ident(ident) => format!("Ident({})", self.dumps_symbol(ident.0)),
            Expr::Int(value) => format!("Int({})", value),
            Expr::Float(value) => format!("Float({})", value),
            Expr::Char(c) => format!("Char({})", c),
            Expr::String(value) => format!("String({})", value),
            Expr::Bool(value) => format!("Bool({})", value),
            Expr::Ref(r) => {
                format!("Ref({})", self.dumps_node(self.get(*r)))
            }
            Expr::Deref(d) => {
                format!("Deref({})", self.dumps_node(self.get(*d)))
            }
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
            Expr::Eq(lhs, rhs) => format!(
                "Eq({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::Lt(lhs, rhs) => format!(
                "Lt({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::LtEq(lhs, rhs) => format!(
                "LtEq({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::Gt(lhs, rhs) => format!(
                "Gt({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::GtEq(lhs, rhs) => format!(
                "GtEq({}, {})",
                self.dumps_node(self.get(*lhs)),
                self.dumps_node(self.get(*rhs))
            ),
            Expr::FieldAccess { base, field } => format!(
                "FieldAccess({}, {})",
                self.dumps_node(self.get(*base)),
                self.dumps_symbol(field.0)
            ),
            Expr::TypeDeclInstance {
                type_name: struct_name,
                fields,
            } => {
                let fields_str: Vec<String> = fields
                    .iter()
                    .map(|(name, value)| {
                        format!(
                            "{}: {}",
                            self.dumps_symbol(name.0),
                            self.dumps_node(self.get(*value))
                        )
                    })
                    .collect();
                format!(
                    "StructInstance {} {{ {} }}",
                    self.dumps_symbol(struct_name.0),
                    fields_str.join(", ")
                )
            }
            Expr::List(elements) => {
                let elements_str: Vec<String> = elements
                    .iter()
                    .map(|&id| self.dumps_node(self.get(id)))
                    .collect();
                format!("List([{}])", elements_str.join(", "))
            }
            Expr::Index { base, index } => format!(
                "Index({}, {})",
                self.dumps_node(self.get(*base)),
                self.dumps_node(self.get(*index))
            ),
            Expr::BuiltinCall { name, args } => {
                let args_str: Vec<String> =
                    args.iter().map(|a| self.dumps_node(self.get(*a))).collect();
                format!("@{} ({})", name, args_str.join(", "))
            }
            Expr::Type(ty) => format!("Type({:?})", ty),
        }
    }

    fn dumps_symbol(&self, id: SymbolId) -> String {
        self.symbol_table.get(id).name.clone()
    }
    */
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
    While {
        condition: LangNodeId,
        body: LangNodeId,
    },
    FunctionCall {
        function: LangNodeId,
        args: Vec<LangNodeId>,
    },
    Return(LangNodeId),
    FunctionDef {
        name: Ident,
        attrs: Vec<Attribute>,
        params: Vec<ParamDef>,
        ret: Type,
        body: Option<LangNodeId>,
        is_expr: bool,
        is_vararg: bool,
    },
    TypeDecl {
        name: Ident,
        kind: TypeDeclKind,
        fields: Vec<ParamDef>,
    },
    Assign {
        lhs: LangNodeId,
        rhs: LangNodeId,
    },
    ExtendDecl {
        target_type: Type,
        methodes: Vec<LangNodeId>,
    },
    ContextDecl {
        name: Ident,
        extensions: Vec<LangNodeId>,
    },
    RequireContextDecl {
        name: Ident,
        items: Vec<LangNodeId>,
    },

    ImportDecl {
        path: String,
        context_mappings: Vec<(Ident, Ident)>,
    },
    InContext {
        context_name: Ident,
        body: LangNodeId,
    },
    Const(Const),
    MacroDef(MacroDef),
    Comptime(LangNodeId),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TypeDeclKind {
    Struct,
    Union,
}

impl LangNodeKind {
    pub fn get_kind_name(&self) -> &'static str {
        match self {
            LangNodeKind::Expr(e) => e.get_kind_name(),
            LangNodeKind::Let(_) => "Let",
            LangNodeKind::Block(_) => "Block",
            LangNodeKind::If { .. } => "If",
            LangNodeKind::FunctionCall { .. } => "FunctionCall",
            LangNodeKind::Return(_) => "Return",
            LangNodeKind::FunctionDef { .. } => "FunctionDef",
            LangNodeKind::Assign { .. } => "Assign",
            LangNodeKind::TypeDecl { .. } => "StructDef",
            LangNodeKind::While { .. } => "While",
            LangNodeKind::Const(_) => "Const",
            LangNodeKind::MacroDef(_) => "MacroDef",
            LangNodeKind::ContextDecl { .. } => "ContextDecl",
            LangNodeKind::RequireContextDecl { .. } => "RequireContextDecl",

            LangNodeKind::ImportDecl { .. } => "ImportDecl",
            LangNodeKind::ExtendDecl { .. } => "ExtendDecl",
            LangNodeKind::InContext { .. } => "InContext",
            LangNodeKind::Comptime(_) => "Comptime",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamDef {
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
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    UnaryOp(UnaryOpKind, LangNodeId),
    Add(LangNodeId, LangNodeId),
    Sub(LangNodeId, LangNodeId),
    Mul(LangNodeId, LangNodeId),
    Div(LangNodeId, LangNodeId),
    Eq(LangNodeId, LangNodeId),
    Lt(LangNodeId, LangNodeId),
    LtEq(LangNodeId, LangNodeId),
    Gt(LangNodeId, LangNodeId),
    GtEq(LangNodeId, LangNodeId),
    Ref(LangNodeId),
    Deref(LangNodeId),
    FieldAccess {
        base: LangNodeId,
        field: Ident,
    },
    TypeDeclInstance {
        type_name: Ident,
        fields: Vec<(Ident, LangNodeId)>,
    },
    List(Vec<LangNodeId>),
    Index {
        base: LangNodeId,
        index: LangNodeId,
    },
    BuiltinCall {
        name: String,
        args: Vec<LangNodeId>,
    },
    Type(Type),
}

impl Expr {
    pub fn get_kind_name(&self) -> &'static str {
        match self {
            Expr::Ident(_) => "Ident",
            Expr::Int(_) => "Int",
            Expr::Float(_) => "Float",
            Expr::String(_) => "String",
            Expr::Char(_) => "Char",
            Expr::Bool(_) => "Bool",
            Expr::UnaryOp(_, _) => "UnaryOp",
            Expr::Add(_, _) => "Add",
            Expr::Sub(_, _) => "Sub",
            Expr::Mul(_, _) => "Mul",
            Expr::Div(_, _) => "Div",
            Expr::Eq(_, _) => "Eq",
            Expr::Lt(_, _) => "Lt",
            Expr::LtEq(_, _) => "LtEq",
            Expr::Gt(_, _) => "Gt",
            Expr::GtEq(_, _) => "GtEq",
            Expr::Ref(_) => "Ref",
            Expr::Deref(_) => "Deref",
            Expr::FieldAccess { .. } => "FieldAccess",
            Expr::TypeDeclInstance { .. } => "StructInstance",
            Expr::List(_) => "List",
            Expr::Index { .. } => "Index",
            Expr::BuiltinCall { .. } => "BuiltinCall",
            Expr::Type(_) => "Type",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Const {
    pub name: Ident,
    pub value: LangNodeId,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct MacroDef {
    pub name: Ident,
    pub params: Vec<ParamDef>,
    pub body: LangNodeId,
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
    pub fn from_span(span: Span, source_file_info: &SourceFileInfo) -> LineInfo {
        let mut line = 0;
        let mut current_start = 0;
        let mut current_end = 0;
        let mut found = false;

        for (i, s) in source_file_info.source().lines().enumerate() {
            let line_len = s.len() + 1; // +1 for the newline character
            current_end += line_len;

            if span.lo >= current_start && span.hi <= current_end {
                line = i + 1; // Lines are 1-indexed
                found = true;
                break;
            }

            current_start += line_len;
        }

        if !found {
            // If not found, return the last line info
            line = source_file_info.source().lines().count();
            current_start = source_file_info
                .source()
                .rfind('\n')
                .map_or(0, |pos| pos + 1);
            current_end = source_file_info.source().len();
        }

        LineInfo::new(line, current_start, current_end)
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
