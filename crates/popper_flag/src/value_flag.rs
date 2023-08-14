use std::fmt::Display;


#[derive(PartialEq, Clone, Debug)]
pub enum ValueFlag {
    Integer,
    Float,
    String,
    Boolean,
    None,
    Array(Box<ValueFlag>),
    Function(Vec<ValueFlag>, Box<ValueFlag>)
}

impl Display for ValueFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueFlag::Integer => write!(f, "integer"),
            ValueFlag::Float => write!(f, "float"),
            ValueFlag::String => write!(f, "string"),
            ValueFlag::Boolean => write!(f, "boolean"),
            ValueFlag::None => write!(f, "none"),
            ValueFlag::Array(t) => write!(f, "array of {}", t.to_string()),
            ValueFlag::Function(args, returntype) => {
                let mut args_string = String::new();
                for arg in args {
                    args_string.push_str(&arg.to_string());
                    args_string.push_str(", ");
                }
                args_string.pop();
                args_string.pop();
                write!(f, "function with args: [{}] and return type: {}", args_string, returntype.to_string())
            }
        }
    }
}