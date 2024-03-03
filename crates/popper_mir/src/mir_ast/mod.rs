pub mod pretty;

use std::fmt::Display;

pub trait MirCompile {
    fn compile(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String, // module <name>
    pub ir: Vec<Ir>,  // <ir>
}

impl Module {
    pub fn new(name: String, ir: Vec<Ir>) -> Self {
        Self { name, ir }
    }

    pub fn push(&mut self, ir: Ir) {
        self.ir.push(ir);
    }
}

impl MirCompile for Module {
    fn compile(&self) -> String {
        format!(
            "module {} {{{}}}",
            self.name,
            self.ir
                .iter()
                .map(|ir| ir.compile())
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ir {
    LoadModule(Module),      // load_module <path>
    LoadExternal(MirString), // load_external <path>
    Declare(Declare),        // declare <name> = args [<args>] ret <ret>
    Function(Function),      // func @<ret> <name>(<args>): <body>
}

impl MirCompile for Ir {
    fn compile(&self) -> String {
        match self {
            Ir::LoadModule(path) => {
                format!("{}", path.compile())
            }
            Ir::LoadExternal(path) => {
                format!("load_external {}", path.compile())
            }
            Ir::Declare(declare) => declare.compile(),
            Ir::Function(function) => function.compile(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirString {
    pub string: String,
}

impl MirString {
    pub fn new(string: String) -> Self {
        Self { string }
    }
}

impl Display for MirString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.string)
    }
}

impl MirCompile for MirString {
    fn compile(&self) -> String {
        format!("\"{}\"", self.string)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,                            // @int
    Float,                          // @float
    String(usize),                  // @string <size>
    Bool,                           // @bool
    Void,                           // @void
    List(Box<Type>, usize),         // @list[<type>: <size>]
    Function(Vec<Type>, Box<Type>), // @function(<args>) <ret>
    Struct(Vec<Type>),              // @struct { <fields> }
    Pointer(Box<Type>),             // @pointer <type>
}

impl Type {
    pub fn into_function(self) -> (Vec<Type>, Box<Type>) {
        match self {
            Type::Function(args, ret) => (args, ret),
            e => {
                panic!("Type is not a function: {:?}", e)
            }
        }
    }

    pub fn into_list(self) -> Option<(Box<Type>, usize)> {
        match self {
            Type::List(t, size) => Some((t, size)),
            _ => None,
        }
    }
}

impl MirCompile for Type {
    fn compile(&self) -> String {
        match self {
            Type::Int => "@int".to_string(),
            Type::Float => "@float".to_string(),
            Type::String(s) => {
                format!("@string {}", s)
            }
            Type::Bool => "@bool".to_string(),
            Type::Void => "@void".to_string(),
            Type::List(t, size) => {
                format!("@list[{}: {}]", t.compile(), size)
            }
            Type::Function(args, ret) => {
                format!(
                    "@function({}) {}",
                    args.iter()
                        .map(|arg| arg.compile())
                        .collect::<Vec<String>>()
                        .join(" "),
                    ret.compile()
                )
            }
            Type::Struct(fields) => {
                format!(
                    "@struct {{ {} }}",
                    fields
                        .iter()
                        .map(|field| field.compile())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Type::Pointer(t) => {
                format!("@pointer {}", t.compile())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declare {
    pub name: String,
    pub args: List<Type>,
    pub ret: Type,
    pub is_var_args: bool,
}

impl Declare {
    pub fn new(name: String, args: List<Type>, ret: Type, is_var_args: bool) -> Self {
        Self {
            name,
            args,
            ret,
            is_var_args,
        }
    }
}

impl MirCompile for Declare {
    fn compile(&self) -> String {
        format!(
            "declare {} = args {} ret {} {}",
            self.name,
            self.args.compile(),
            self.ret.compile(),
            if self.is_var_args { "..." } else { "" }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List<T: MirCompile + PartialEq> {
    pub list: Vec<T>, // [T]
}

impl<T: MirCompile + PartialEq> List<T> {
    pub fn new(list: Vec<T>) -> Self {
        Self { list }
    }
}

impl<T: MirCompile + PartialEq> MirCompile for List<T> {
    fn compile(&self) -> String {
        format!(
            "[{}]",
            self.list
                .iter()
                .map(|item| item.compile())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Arguments,
    pub ret: Type,
    pub is_var_args: bool,
    pub body: Body,
}

impl Function {
    pub fn new(name: String, args: Arguments, ret: Type, is_var_args: bool, body: Body) -> Self {
        Self {
            name,
            args,
            ret,
            is_var_args,
            body,
        }
    }
}

impl MirCompile for Function {
    fn compile(&self) -> String {
        format!(
            "func {} {} {}{}{{\n\t{}\n}}",
            self.ret.compile(),
            self.name,
            self.args.compile(),
            if self.is_var_args { " ... " } else { " " },
            self.body.compile()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    pub args: Vec<Argument>,
}

impl Arguments {
    pub fn new(args: Vec<Argument>) -> Self {
        Self { args }
    }
}

impl MirCompile for Arguments {
    fn compile(&self) -> String {
        format!(
            "({})",
            self.args
                .iter()
                .map(|arg| arg.compile())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub ty: Type,
}

impl Argument {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }
}

impl MirCompile for Argument {
    fn compile(&self) -> String {
        format!("{} {}", self.ty.compile(), self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub body: Vec<BodyFn>,
}

impl Body {
    pub fn new(body: Vec<BodyFn>) -> Self {
        Self { body }
    }
    pub fn push(&mut self, body_fn: BodyFn) {
        self.body.push(body_fn);
    }
}

impl MirCompile for Body {
    fn compile(&self) -> String {
        self.body
            .iter()
            .map(|body_fn| body_fn.compile())
            .collect::<Vec<String>>()
            .join("\n\t")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyFn {
    Alloc(Alloc),   // alloc <name>, <ty>
    Store(Store),   // store <name>, <value>
    Call(Call),     // call <name>, [<args>], <ret>
    Return(Return), // ret <value>
    Add(Add),       // add <name>, <value>, <res>
    Index(Index),   // index <res>, <list>, <index>
    VaArg(VaArg),   // va_arg <res>, <ty>
    Ref(Ref),       // ref <val>, <res>
    Deref(Deref),   // deref <val>, <res>
}

impl MirCompile for BodyFn {
    fn compile(&self) -> String {
        match self {
            BodyFn::Alloc(alloc) => alloc.compile(),
            BodyFn::Store(store) => store.compile(),
            BodyFn::Call(call) => call.compile(),
            BodyFn::Return(ret) => ret.compile(),
            BodyFn::Add(add) => add.compile(),
            BodyFn::Index(index) => index.compile(),
            BodyFn::VaArg(va_arg) => va_arg.compile(),
            BodyFn::Ref(r#ref) => r#ref.compile(),
            BodyFn::Deref(deref) => deref.compile(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alloc {
    pub name: String,
    pub ty: Type,
}

impl Alloc {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }
}

impl MirCompile for Alloc {
    fn compile(&self) -> String {
        format!("alloc {}, {}", self.name, self.ty.compile())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub name: String,
    pub value: Value,
}

impl Store {
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}

impl MirCompile for Store {
    fn compile(&self) -> String {
        format!("store {}, {}", self.name, self.value.compile())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub name: String,
    pub args: List<Value>,
    pub ret: String,
}

impl Call {
    pub fn new(name: String, args: List<Value>, ret: String) -> Self {
        Self { name, args, ret }
    }
}

impl MirCompile for Call {
    fn compile(&self) -> String {
        format!("call {}, {}, {}", self.name, self.args.compile(), self.ret)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Option<Value>,
}

impl Return {
    pub fn new(value: Option<Value>) -> Self {
        Self { value }
    }
}

impl MirCompile for Return {
    fn compile(&self) -> String {
        if let Some(value) = &self.value {
            format!("ret {}", value.compile())
        } else {
            "ret".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Add {
    pub name: String,
    pub lhs: Value,
    pub rhs: Value,
}

impl Add {
    pub fn new(name: String, lhs: Value, rhs: Value) -> Self {
        Self { name, lhs, rhs }
    }
}

impl MirCompile for Add {
    fn compile(&self) -> String {
        format!(
            "add {}, {}, {}",
            self.name,
            self.lhs.compile(),
            self.rhs.compile()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub res: String,
    pub list: Value,
    pub index: Value,
}

impl Index {
    pub fn new(res: String, list: Value, index: Value) -> Self {
        Self { res, list, index }
    }
}

impl MirCompile for Index {
    fn compile(&self) -> String {
        format!(
            "index {}, {}, {}",
            self.res,
            self.list.compile(),
            self.index.compile()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VaArg {
    pub res: String,
    pub ty: Type,
}

impl VaArg {
    pub fn new(res: String, ty: Type) -> Self {
        Self { res, ty }
    }
}

impl MirCompile for VaArg {
    fn compile(&self) -> String {
        format!("va_arg {}, {}", self.res, self.ty.compile())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ref {
    pub val: Value,
    pub res: String,
}

impl Ref {
    pub fn new(val: Value, res: String) -> Self {
        Self { val, res }
    }
}

impl MirCompile for Ref {
    fn compile(&self) -> String {
        format!("ref {}, {}", self.val.compile(), self.res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deref {
    pub ptr: Value,
    pub res: String,
}

impl Deref {
    pub fn new(ptr: Value, res: String) -> Self {
        Self { ptr, res }
    }
}

impl MirCompile for Deref {
    fn compile(&self) -> String {
        format!("deref {}, {}", self.ptr.compile(), self.res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirPtr {
    pub ty: Type,
}

impl MirPtr {
    pub fn new(ty: Type) -> Self {
        Self { ty }
    }
}

impl MirCompile for MirPtr {
    fn compile(&self) -> String {
        format!("ptr {}", self.ty.compile())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Const(Const),       // const <value>
    Variable(Variable), // id <name>
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Const(constant) => constant.get_type(),
            Value::Variable(variable) => variable.ty.clone(),
        }
    }

    pub fn into_array(&self) -> Option<&MirList> {
        match self {
            Value::Const(Const::List(l)) => Some(l),
            _ => None,
        }
    }

    pub fn get_minor_type(&self) -> Option<Type> {
        match self {
            Value::Const(Const::List(l)) => Some(l.get_minor_type()),
            Value::Const(Const::Ptr(p)) => Some(p.ty.clone()),
            Value::Variable(variable) => {
                if let Type::Pointer(p) = &variable.ty {
                    Some(*p.clone())
                } else if let Type::List(a, _) = &variable.ty {
                    Some(*a.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl MirCompile for Value {
    fn compile(&self) -> String {
        match self {
            Value::Const(constant) => constant.compile(),
            Value::Variable(variable) => variable.compile(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Int(MirInt),       // int <value>
    Float(MirFloat),   // float <value>
    String(MirString), // string <value>
    Bool(MirBool),     // bool <value>
    List(MirList),     // list <value>
    Ptr(MirPtr),       // ptr <value>
    Void,              // void
}

impl Const {
    pub fn get_type(&self) -> Type {
        match self {
            Const::Int(_) => Type::Int,
            Const::Float(_) => Type::Float,
            Const::String(s) => Type::String(s.string.len()),
            Const::List(l) => l.get_type(),
            Const::Bool(_) => Type::Bool,
            Const::Void => Type::Void,
            Const::Ptr(p) => Type::Pointer(Box::new(p.ty.clone())),
        }
    }
}

impl MirCompile for Const {
    fn compile(&self) -> String {
        match self {
            Const::Int(int) => int.compile(),
            Const::Float(float) => float.compile(),
            Const::String(string) => {
                format!("string {}", string.compile())
            }
            Const::Bool(bool) => bool.compile(),
            Const::List(list) => list.compile(),
            Const::Void => "void".to_string(),
            Const::Ptr(p) => p.compile(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirInt {
    pub value: i64,
}

impl MirInt {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl MirCompile for MirInt {
    fn compile(&self) -> String {
        format!("int {}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirFloat {
    pub value: f64,
}

impl MirFloat {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl MirCompile for MirFloat {
    fn compile(&self) -> String {
        format!("float {}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirBool {
    pub value: bool,
}

impl MirBool {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl MirCompile for MirBool {
    fn compile(&self) -> String {
        format!("bool {}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MirList {
    pub values: Vec<Value>,
}

impl MirList {
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }

    pub fn get_type(&self) -> Type {
        Type::List(Box::new(self.get_minor_type()), self.values.len())
    }

    pub fn get_minor_type(&self) -> Type {
        self.values.first().unwrap().get_type()
    }
}

impl MirCompile for MirList {
    fn compile(&self) -> String {
        let mut result = String::from("list [");
        for value in &self.values {
            result.push_str(&value.compile());
            result.push_str(", ");
        }
        result.pop();
        result.pop();
        result.push(']');
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub ty: Type,
}

impl Variable {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }
}

impl MirCompile for Variable {
    fn compile(&self) -> String {
        format!("{} {}", self.ty.compile(), self.name)
    }
}
