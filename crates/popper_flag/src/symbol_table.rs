use popper_ast::Span;
use crate::{Flag, TypeFlag, ValueFlag, ScopeFlag, Environment, VariableFlag};


/// save all flags in this struct
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolFlags {
    pub symbols: Vec<Flag>,
    pub span: Span,
    vars: Environment,
}

impl SymbolFlags {
    pub fn new(span: Span) -> Self {
        Self {
            symbols: Vec::new(),
            span,
            vars: Environment::new(),
        }
    }

    pub fn add_flag(&mut self, flag: Flag) -> &mut Self {
        self.symbols.push(flag);
        self
    }

    pub fn set_integer(&mut self) -> &mut Self {
        self.add_flag(
            Flag::Value(
                ValueFlag::Integer
            )
        );
        self
    }

    pub fn set_float(&mut self) -> &mut Self {
        self.add_flag(
            Flag::Value(
                ValueFlag::Float
            )
        );
        self
    }

    pub fn set_string(&mut self) -> &mut Self {
        self.add_flag(
            Flag::Value(
                ValueFlag::String
            )
        );
        self
    }

    pub fn set_boolean(&mut self) -> &mut Self {
        self.add_flag(
            Flag::Value(
                ValueFlag::Boolean
            )
        );
        self
    }

    pub fn set_array(&mut self, type_flag: TypeFlag) -> &mut Self {
        self.add_flag(
            Flag::Value(
                ValueFlag::Array(type_flag)
            )
        );
        self
    }

    pub fn set_none(&mut self) -> &mut Self {
        self.add_flag(
            Flag::Value(
                ValueFlag::None
            )
        );
        self
    }

    pub fn set_init_variable(&mut self, name: String, value: SymbolFlags, scope: ScopeFlag, mutable: bool) -> &mut Self {
        self.vars.add_variable(
            VariableFlag::new(
                name,
                value,
                scope,
                mutable
            )
        );
        self
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableFlag> {
        self.vars.get_variable(name)
    }

    pub fn is_integer(&self) -> bool {
        self.symbols.iter().any(|s| s == &Flag::Value(ValueFlag::Integer))
    }

    pub fn is_float(&self) -> bool {
        self.symbols.iter().any(|s| s == &Flag::Value(ValueFlag::Float))
    }

    pub fn is_string(&self) -> bool {
        self.symbols.iter().any(|s| s == &Flag::Value(ValueFlag::String))
    }

    pub fn is_boolean(&self) -> bool {
        self.symbols.iter().any(|s| s == &Flag::Value(ValueFlag::Boolean))
    }

    pub fn is_array(&self, type_flag: TypeFlag) -> bool {
        self.symbols.iter().any(|s|
            s == &Flag::Value(
                ValueFlag::Array(type_flag.clone())
            )
        )
    }

    pub fn get_value(&self) -> Option<ValueFlag> {
        self.symbols.iter().find_map(|s| {
            match s {
                Flag::Value(v) => Some(v.clone()),
                _ => None
            }
        })
    }



    pub fn is_same_value(&self, other: Self) -> bool {
        if self.get_value() == None || other.get_value() == None {
            return false;
        }

        self.get_value() == other.get_value()
    }


    pub fn span(self) -> Span {
        self.span
    }
}
