use crate::tree::Expr;
use crate::tree::Literal;
use crate::tree::Op;
use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    name: String,
    args: Vec<Ident>,
    body: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Function(Function),
    List(Vec<Value>),
    None,
}

impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err("invalid operation".to_string()),
        }
    }
    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn modulo(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn eq(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn neq(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn gt(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a > *b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn lt(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a < *b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn ge(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a >= *b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn le(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a <= *b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn and(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
            _ => Err("invalid operation".to_string()),
        }
    }

    pub fn or(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
            _ => Err("invalid operation".to_string()),
        }
    }
}

pub struct Vm(std::collections::HashMap<Ident, Value>);

impl Vm {
    pub fn new() -> Self {
        Vm(HashMap::new())
    }
    pub fn from(map: HashMap<Ident, Value>) -> Self {
        Vm(map)
    }
    pub fn eval_expr(&mut self, expr: Expr) -> Result<Value, String> {
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
            Expr::Ident { name } => Ok(self.get_ident(Ident(name))),
            Expr::BinOp { op, left, right } => {
                let left = self.eval_expr(*left)?;
                let right = self.eval_expr(*right)?;
                return Ok(match op {
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
                    _ => return Err(String::from("Unknown operator")),
                });
            }
            Expr::IfThen { cond, then } => {
                if let Value::Bool(c) = self.eval_expr(*cond)? {
                    if c {
                        return Ok(self.eval_expr(*then)?);
                    } else {
                        return Ok(Value::None);
                    }
                } else {
                    return Err("condition is not bool".to_string());
                }
            }
            Expr::IfThenElse { cond, then, else_ } => {
                if let Value::Bool(n) = self.eval_expr(*cond)? {
                    if n {
                        return self.eval_expr(*then);
                    } else {
                        return self.eval_expr(*else_);
                    }
                    return self.eval_expr(*then);
                } else {
                    return Err("condition is not bool".to_string());
                }
            }
            Expr::Assign { name, value } => {
                let value_evaluate = self.eval_expr(*value)?;
                self.set_ident(Ident(name), value_evaluate.clone());
                return Ok(Value::None);
            }
            Expr::While { ref cond, ref body } => {
                while self.eval_expr(*cond.clone())? == Value::Bool(true) {
                    self.eval_expr(*body.clone())?;
                }
                return Ok(Value::None);
            }
            Expr::For {
                ref name,
                ref iter,
                ref body,
            } => {
                let iter = self.eval_expr(*iter.clone())?;
                let n = match iter {
                    Value::Number(n) => n,
                    _ => return Err("iter is not number".to_string()),
                };
                let mut last = Value::None;
                for i in 0..(n + 1.0) as i32 {
                    self.set_ident(
                        Ident(match **name {
                            Expr::Ident { ref name } => name.clone().to_string(),
                            _ => return Err("name is not identifier".to_string()),
                        }),
                        Value::Number(i as f64),
                    );
                    last = self.eval_expr(*body.clone())?;
                }
                return Ok(last);
            },
            Expr::FunDef {
                ref name,
                ref args,
                ref body,
            } => {
                let mut args_vec = Vec::new();
                for arg in args {
                    let arg_name = match arg {
                        Expr::Ident { ref name } => name.clone(),
                        _ => return Err("arg is not identifier".to_string()),
                    };
                    args_vec.push(Ident(arg_name));
                }
                self.set_ident(Ident(name.clone()), Value::Function(Function {
                    name: name.clone(),
                    args: args_vec.clone(),
                    body: *body.clone()
                }));
                return Ok(Value::None);
            },
            Expr::Call {
                ref name,
                ref args,
            } => {
                let mut new_vm = Vm::from(self.0.clone());
                let mut args_vec = Vec::new();
                for arg in args {
                    let arg_value = self.eval_expr(arg.clone())?;
                    args_vec.push(arg_value);
                }
                let function = match self.get_ident(Ident(name.clone())) {
                    Value::Function(f) => f,
                    _ => return Err("function not found".to_string()),
                };
                new_vm.set_ident(Ident(function.name.clone()), Value::Function(function.clone()));
                if args_vec.len() != function.args.len() {
                    return Err("wrong number of arguments".to_string());
                }
                for (arg, arg_value) in function.args.iter().zip(args_vec.iter()) {
                    new_vm.set_ident(arg.clone(), arg_value.clone());
                }
                return new_vm.eval_expr(function.body);
            },
            Expr::List { ref elems } => {
                let mut list = Vec::new();
                for elem in elems {
                    list.push(self.eval_expr(elem.clone())?);
                }
                return Ok(Value::List(list));
            },
        }
    }

    pub fn set_ident(&mut self, ident: Ident, value: Value) {
        self.0.insert(ident, value);
    }

    pub fn get_ident(&mut self, ident: Ident) -> Value {
        match self.0.get(&ident) {
            Some(v) => v.clone(),
            None => Value::None,
        }
    }

    
}
