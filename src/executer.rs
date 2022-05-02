use crate::tree::Expr; 
use crate::tree::Op;
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Ident(String);

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Error,
    None
}

impl Value {
    fn add(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                _ => Value::Error
            }
        }

    }
    fn sub(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                _ => Value::Error
            }
        }

    }

    fn mul(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                _ => Value::Error
            }
        }

    }

    fn div(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                _ => Value::Error
            }
        }

    }

    fn modulo(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
                _ => Value::Error
            }
        }

    }

    fn eq(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Bool(a == b),
                (Value::String(a), Value::String(b)) => Value::Bool(a == b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                _ => Value::Error
            }
        }

    }

    fn neq(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Bool(a != b),
                (Value::String(a), Value::String(b)) => Value::Bool(a != b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
                _ => Value::Error
            }
        }

    }

    fn gt(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Bool(*a > *b),
                _ => Value::Error
            }
        }

    }

    fn lt(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Bool(*a < *b),
                _ => Value::Error
            }
        }

    }

    fn ge(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Bool(*a >= *b),
                _ => Value::Error
            }
        }

    }

    fn le(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Number(a), Value::Number(b)) => Value::Bool(*a <= *b),
                _ => Value::Error
            }
        }

    }

    fn and(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(*a && *b),
                _ => Value::Error
            }
        }

    }

    fn or(&self, other: &Value) -> Value {
        if self != other  {
            Value::Error 
        } else {
            match (self, other) {
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(*a || *b),
                _ => Value::Error
            }
        }

    }
}

struct Vm(std::collections::HashMap<Ident, Value>);

impl Vm {
    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Number { value }  => Value::Number(value),
            Expr::String { value } => Value::String(value),
            Expr::Bool { value } => Value::Bool(value),
            Expr::Identifier{ name } => self.get_ident(Ident(name)),
            Expr::BinOp { op, left, right } => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);
                return match op {
                    Op::Add => left.add(&right),
                    Op::Sub => left.sub(&right),
                    Op::Mul => left.mul(&right),
                    Op::Div => left.div(&right),
                    Op::Mod => left.modulo(&right),
                    Op::Eq => left.eq(&right),
                    Op::Neq => left.neq(&right),
                    Op::Gt => left.gt(&right),
                    Op::Lt => left.lt(&right),
                    Op::Ge => left.ge(&right),
                    Op::Le => left.le(&right),
                    Op::And => left.and(&right),
                    Op::Or => left.or(&right),
                    _ => Value::Error

                };
            },
            Expr::IfThen { cond, then } => {
                if self.eval_expr(*cond) == Value::Bool(true) {
                    return self.eval_expr(*then);
                }
                else {
                    return Value::None;
                }
            },
            Expr::IfThenElse { cond, then, else_ } => {
                if self.eval_expr(*cond) == Value::Bool(true) {
                    return self.eval_expr(*then);
                }
                else {
                    return self.eval_expr(*else_);
                }
            },
            Expr::Assign { name, value } => {
                let value_evaluate = self.eval_expr(*value);
                self.set_ident(Ident(name), value_evaluate);
                return value_evaluate;
            },
            Expr::While { cond, body } => {
                while self.eval_expr(*cond) == Value::Bool(true) {
                    self.eval_expr(body as tree::Expr);
                }
                return Value::None;
            },
            Expr::For {name, iter, body} => {
                let iter = self.eval_expr(*iter);
                let Value::Number(n) = iter;
                for i in 0..n as i32 {
                    self.set_ident(Ident(name), Value::Number(i as f64));
                    self.eval_expr(*body.clone());
                }
                return Value::None;
            },
        }
    }

    fn set_ident(&mut self, ident: Ident, value: Value) {
        self.0.insert(ident, value);
    }

    fn get_ident(&mut self, ident: Ident) -> Value {
        match self.0.get(&ident) {
            Some(v) => v.clone(),
            None => Value::None
        }
    }
}