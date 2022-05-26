use crate::errors::CannotAddError;
use crate::errors::CannotCompareError;
use crate::errors::CannotDivError;
use crate::errors::CannotModError;
use crate::errors::CannotMulError;
use crate::errors::CannotSubError;
use crate::errors::Error;
use crate::errors::FunctionNotFoundError;
use crate::errors::IndexOutOfBoundsError;
use crate::errors::IsBuiltinError;
use crate::errors::TypeMismatchError;
use crate::errors::VarAlreadyDefinedError;
use crate::errors::VarNotFoundError;
use crate::tree::Expr;
use crate::tree::Literal;
use crate::tree::Op;
use std::fmt;
use std::ops::Range;

use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Ident(pub String);

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Function {
        name: String,
        args: Vec<Ident>,
        body: Expr,
    },
    List(Vec<Value>),
    Range(Range<isize>),
    None,
}

impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err(Error::CannotAdd(CannotAddError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }
    pub fn sub(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(Error::CannotSub(CannotSubError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(Error::CannotMul(CannotMulError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err(Error::CannotDiv(CannotDivError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn modulo(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            _ => Err(Error::CannotMod(CannotModError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn eq(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn neq(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn gt(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a > *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn lt(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a < *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn ge(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a >= *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn le(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a <= *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn and(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn or(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn display_value(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Function { .. } => "function".to_string(),
            Value::List(list) => {
                let mut s = String::new();
                s.push_str("[");
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&item.display_value());
                }
                s.push_str("]");
                s
            }
            Value::Range(_) => "range".to_string(),
            Value::None => "None".to_string(),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Value::Number(_) => "number".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Function { .. } => "function".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Range(_) => "range".to_string(),
            Value::None => "None".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_value())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vm(
    std::collections::HashMap<Ident, Value>,
    HashMap<String, fn(Vec<Value>) -> Value>,
);

impl Vm {
    pub fn new() -> Self {
        let mut map_builtins = HashMap::new();
        map_builtins.insert("print".to_string(), Vm::print as fn(Vec<Value>) -> Value);
        map_builtins.insert("println".to_string(), Vm::println);
        Vm(HashMap::new(), map_builtins)
    }
    /* pub fn from(
        map: HashMap<Ident, Value>,
        builtin: HashMap<String, fn(Vec<Value>) -> Value>,
    ) -> Self {
        Vm(map, builtin)
    } */
    pub fn eval_expr(&mut self, expr: Expr) -> Result<Value, Error> {
        match expr {
            Expr::Empty => Ok(Value::None),
            Expr::Block { body } => {
                let mut last = Value::None;
                for expr in body {
                    last = self.eval_expr(expr)?;
                }
                Ok(last)
            }
            Expr::Literal { value } => Ok(match value {
                Literal::Number(n) => Value::Number(n),
                Literal::String(s) => Value::String(s),
                Literal::Bool(b) => Value::Bool(b),
            }),
            Expr::Ident { ref ident } => match self.get_ident(Ident(ident.clone())) {
                Some(value) => Ok(value.clone()),
                None => Err(Error::VarNotFound(VarNotFoundError {
                    var_name: ident.clone(),
                })),
            },
            Expr::BinOp { op, left, right } => {
                let left = self.eval_expr(*left)?;

                let right = self.eval_expr(*right)?;
                Ok(match op {
                    Op::Add => left.add(&right)?,
                    Op::Sub => left.sub(&right)?,
                    Op::Mul => left.mul(&right)?,
                    Op::Div => left.div(&right)?,
                    Op::Mod => left.modulo(&right)?,
                    Op::Eq => left.eq(&right)?,
                    Op::Neq => left.neq(&right)?,
                    Op::Gt => left.gt(&right)?,
                    Op::Lt => left.lt(&right)?,
                    Op::Ge => left.ge(&right)?,
                    Op::Le => left.le(&right)?,
                    Op::And => left.and(&right)?,
                    Op::Or => left.or(&right)?,
                })
            }
            Expr::IfThen { cond, then } => {
                let v = self.eval_expr(*cond)?;
                if let Value::Bool(c) = v {
                    if c {
                        Ok(self.eval_expr(*then)?)
                    } else {
                        Ok(Value::None)
                    }
                } else {
                    Err(Error::TypeMismatch(TypeMismatchError {
                        expected: "bool".to_string(),
                        found: v.get_type(),
                    }))
                }
            }
            Expr::IfThenElse { cond, then, else_ } => {
                let v = self.eval_expr(*cond)?;
                if let Value::Bool(n) = v {
                    if n {
                        self.eval_expr(*then)
                    } else {
                        self.eval_expr(*else_)
                    }
                } else {
                    Err(Error::TypeMismatch(TypeMismatchError {
                        expected: "bool".to_string(),
                        found: v.get_type(),
                    }))
                }
            }
            Expr::Assign { name, value } => {
                let value_evaluate = self.eval_expr(*value)?;
                if self.get_ident(Ident(name.clone())).is_some() {
                    return Err(Error::VarAlreadyDefined(VarAlreadyDefinedError {
                        var_name: name,
                    }));
                }
                self.set_ident(Ident(name), value_evaluate.clone());
                Ok(Value::None)
            }
            Expr::While { ref cond, ref body } => {
                while self.eval_expr(*cond.clone())? == Value::Bool(true) {
                    self.eval_expr(*body.clone())?;
                }
                Ok(Value::None)
            }
            Expr::For {
                ref name,
                ref iter,
                ref body,
            } => {
                let name_str = match *name.clone() {
                    Expr::Ident { ident } => ident,
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: "string".to_string(),
                            found: "ident".to_string(),
                        }))
                    }
                };

                let iter = self.eval_expr(*iter.clone())?;
                match iter {
                    Value::List(ref l) => {
                        let mut last = Value::None;
                        for item in l {
                            self.set_ident(Ident(name_str.clone()), item.clone());
                            last = self.eval_expr(*body.clone())?;
                        }
                        Ok(last)
                    }
                    Value::Range(r) => {
                        let mut last = Value::None;
                        for i in r {
                            self.set_ident(Ident(name_str.clone()), Value::Number(i as f64));
                            last = self.eval_expr(*body.clone())?;
                        }
                        Ok(last)
                    }
                    _ => Err(Error::TypeMismatch(TypeMismatchError {
                        expected: "list".to_string(),
                        found: iter.get_type(),
                    })),
                }
            }
            Expr::FunDef {
                ref name,
                ref args,
                ref body,
            } => {
                if self.1.clone().into_iter().any(|x| x.0 == name.clone()) {
                    return Err(Error::IsBuiltin(IsBuiltinError { name: name.clone() }));
                }
                let mut args_vec = Vec::new();
                for arg in args {
                    let arg_name = match arg {
                        Expr::Ident { ref ident } => ident.clone(),
                        _ => {
                            return Err(Error::TypeMismatch(TypeMismatchError {
                                expected: "ident".to_string(),
                                found: "unknown".to_string(),
                            }))
                        }
                    };
                    args_vec.push(Ident(arg_name));
                }
                self.set_ident(
                    Ident(name.clone()),
                    Value::Function {
                        name: name.clone(),
                        args: args_vec.clone(),
                        body: *body.clone(),
                    },
                );
                Ok(Value::None)
            }
            Expr::Call {
                ref name, ref args, ..
            } => {
                let cparg = args.clone();
                let b = self.1.clone();
                if b.clone().into_iter().any(|x| x.0 == name.clone()) {
                    let v = b.get(name).unwrap().clone();
                    let args_iter = args.iter();

                    let args_map = args_iter.map(|x| -> Value {
                        match self.eval_expr(x.clone()) {
                            Ok(v) => v,
                            Err(_e) => Value::None,
                        }
                    });

                    let args = args_map.collect::<Vec<Value>>();
                    return Ok(v(args));
                }
                let mut new_vm = Vm::new();
                let mut copy_self = self.clone();
                match copy_self.get_ident(Ident(name.clone())) {
                    Some(f) => match f {
                        &Value::Function {
                            ref name,
                            ref args,
                            ref body,
                        } => {
                            new_vm.set_ident(Ident(name.clone()), f.clone());
                            for (i, arg) in args.iter().enumerate() {
                                let ev_arg = self.eval_expr(cparg[i].clone())?;
                                new_vm.set_ident(arg.clone(), ev_arg.clone());
                            }
                            new_vm.eval_expr(body.clone())
                        }
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: "function".to_string(),
                            found: f.get_type(),
                        })),
                    },
                    None => Err(Error::FunctionNotFound(FunctionNotFoundError {
                        name: name.clone(),
                    })),
                }
            }
            Expr::List { ref elems } => {
                let mut list = Vec::new();
                for elem in elems {
                    list.push(self.eval_expr(elem.clone())?);
                }
                Ok(Value::List(list))
            }
            Expr::Index {
                ref name,
                ref index,
            } => {
                let real_name = match **name {
                    Expr::Ident { ref ident } => ident.clone(),
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: "ident".to_string(),
                            found: "unknown".to_string(),
                        }))
                    }
                };
                let mut copy_vm = self.clone();
                let list = match copy_vm.get_ident(Ident(real_name.clone())) {
                    Some(Value::List(list)) => list,
                    None => {
                        return Err(Error::VarNotFound(VarNotFoundError {
                            var_name: real_name,
                        }))
                    }
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: "list".to_string(),
                            found: self.get_ident(Ident(real_name)).unwrap().get_type(),
                        }))
                    }
                };

                let index = self.eval_expr(*index.clone())?;
                match index {
                    Value::Number(num) => {
                        if num < 0.0 {
                            return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                                index: num as i32,
                                name: real_name,
                            }));
                        }
                        if num as usize >= list.len() {
                            return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                                index: num as i32,
                                name: real_name,
                            }));
                        }
                        Ok(list[num as usize].clone())
                    }
                    Value::Range(r) => {
                        if r.start >= list.len() as isize {
                            return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                                index: r.start as i32,
                                name: real_name,
                            }));
                        }

                        if r.end > list.len() as isize {
                            return Err(Error::IndexOutOfBounds(IndexOutOfBoundsError {
                                index: r.end as i32,
                                name: real_name,
                            }));
                        }

                        Ok(Value::List(list[r.start as usize..r.end as usize].to_vec()))
                    }
                    _ => Err(Error::TypeMismatch(TypeMismatchError {
                        expected: "number".to_string(),
                        found: index.get_type(),
                    })),
                }
            }
            Expr::Range { ref start, ref end } => {
                let start = self.eval_expr(*start.clone())?;
                let end = self.eval_expr(*end.clone())?;
                let start = match start {
                    Value::Number(n) => n,
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: "number".to_string(),
                            found: start.get_type(),
                        }))
                    }
                };
                let end = match end {
                    Value::Number(n) => n,
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: "number".to_string(),
                            found: end.get_type(),
                        }))
                    }
                };

                Ok(Value::Range(start as isize..end as isize))
            }
        }
    }

    pub fn set_ident(&mut self, ident: Ident, value: Value) {
        self.0.insert(ident, value);
    }

    pub fn get_ident(&mut self, ident: Ident) -> Option<&Value> {
        self.0.get(&ident)
    }

    pub fn print(args: Vec<Value>) -> Value {
        for i in args {
            print!("{}", i.display_value());
        }
        Value::None
    }

    pub fn println(args: Vec<Value>) -> Value {
        for i in args {
            print!("{}", i.display_value());
        }
        println!();
        Value::None
    }
}
