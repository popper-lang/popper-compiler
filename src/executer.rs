use crate::tree::Expr;
use crate::tree::Literal;
use crate::tree::Op;
use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
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
            }
            _ => Err(format!("Unknown expression : {:?}", expr)),
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

    pub fn eval_many_expr(&mut self, exprs: Vec<Expr>) -> Result<Value, String> {
        let mut last = Value::None;
        for expr in exprs {
            last = self.eval_expr(expr)?;
        }
        Ok(last)
    }
}
