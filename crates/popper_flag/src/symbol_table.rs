use crate::{Environment, Flag, ScopeFlag, ValueFlag, VariableFlag};
use popper_ast::Span;
use std::collections::HashMap;

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

    pub fn set_actual_value(&mut self, value: ValueFlag) -> &mut Self {
        self.symbols.iter_mut().for_each(|flag| {
            if let Flag::Value(v) = flag {
                *v = value.clone();
            } else if let Flag::Variable(v) = flag {
                v.value = SymbolFlags::new(self.span).add_flag(
                    Flag::Value(value.clone()),
                ).clone();
            }
        });

        self
    }

    pub fn add_flag(&mut self, flag: Flag) -> &mut Self {
        self.symbols.push(flag);
        self
    }

    pub fn set_value(&mut self, value: ValueFlag) -> &mut Self {
        self.add_flag(Flag::Value(value))
    }

    pub fn set_integer(&mut self) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::Integer));
        self
    }

    pub fn set_float(&mut self) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::Float));
        self
    }

    pub fn set_string(&mut self, size: u32) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::String(size)));
        self
    }

    pub fn set_boolean(&mut self) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::Boolean));
        self
    }

    pub fn set_list(&mut self, value_flag: ValueFlag, size: usize) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::List(Box::new(value_flag), size)));
        self
    }

    pub fn set_function(
        &mut self,
        args: Vec<ValueFlag>,
        returnty: ValueFlag,
        is_var_args: bool,
    ) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::Function(
            args,
            Box::new(returnty),
            is_var_args,
        )));

        self
    }

    pub fn set_struct(&mut self, fields: HashMap<String, ValueFlag>) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::Struct(fields)));
        self
    }

    pub fn set_pointer(&mut self, value_flag: ValueFlag) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::Pointer(Box::new(value_flag))));
        self
    }

    pub fn set_none(&mut self) -> &mut Self {
        self.add_flag(Flag::Value(ValueFlag::None));
        self
    }

    pub fn set_init_variable(
        &mut self,
        name: String,
        value: SymbolFlags,
        scope: ScopeFlag,
        mutable: bool,
        span: Span,
    ) -> &mut Self {
        self.vars
            .add_variable(VariableFlag::new(name, value, scope, mutable, span));
        self
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableFlag> {
        self.vars.get_variable(name)
    }

    pub fn get_function(&self) -> Option<(&Vec<ValueFlag>, &Box<ValueFlag>, bool)> {
        self.symbols.iter().find_map(|s| match s {
            Flag::Value(ValueFlag::Function(args, ret, var_args)) => Some((args, ret, *var_args)),
            _ => None,
        })
    }

    pub fn is_integer(&self) -> bool {
        self.symbols
            .iter()
            .any(|s| s == &Flag::Value(ValueFlag::Integer))
    }

    pub fn is_float(&self) -> bool {
        self.symbols
            .iter()
            .any(|s| s == &Flag::Value(ValueFlag::Float))
    }

    pub fn is_string(&self) -> bool {
        self.symbols
            .iter()
            .any(|s| matches!(s, Flag::Value(ValueFlag::String(_))))
    }

    pub fn is_boolean(&self) -> bool {
        self.symbols
            .iter()
            .any(|s| s == &Flag::Value(ValueFlag::Boolean))
    }

    pub fn is_list(&self, value_flag: ValueFlag, length: usize) -> bool {
        self.symbols
            .iter()
            .any(|s| s == &Flag::Value(ValueFlag::List(Box::new(value_flag.clone()), length)))
    }

    pub fn is_iterable(&self) -> bool {
        self.symbols
            .iter()
            .any(|s| matches!(s, Flag::Value(ValueFlag::List(_, _))))
    }

    pub fn is_struct(&self) -> bool {
        self.symbols
            .iter()
            .any(|s| matches!(s, Flag::Value(ValueFlag::Struct(_))))
    }

    pub fn is_pointer(&self) -> bool {
        self.get_value().map(|x| matches!(x, ValueFlag::Pointer(_))).unwrap_or(false)
    }

    pub fn get_value(&self) -> Option<ValueFlag> {
        self.symbols.iter().find_map(|s| match s {
            Flag::Value(v) => Some(v.clone()),
            Flag::Variable(b) => b.value.get_value(),
        })
    }

    pub fn get_list(&self) -> Option<(ValueFlag, usize)> {
        self.symbols.iter().find_map(|s| match s {
            Flag::Value(ValueFlag::List(v, l)) => Some((*v.clone(), *l)),
            _ => None,
        })
    }

    pub fn get_minor_type(&self) -> Option<ValueFlag> {
        self.symbols.iter().find_map(|s| match s {
            Flag::Value(ValueFlag::Pointer(v)) => Some(*v.clone()),
            Flag::Variable(v) => v.value.get_minor_type(),
            _ => None,
        })
    }

    pub fn is_same_value(&self, other: Self) -> bool {
        if self.get_value().is_none() || other.get_value().is_none() {
            return false;
        }

        self.get_value() == other.get_value()
    }

    pub fn expect_variable(&self) -> Option<VariableFlag> {
        self.symbols.iter().find_map(|s| match s {
            Flag::Variable(v) => Some(v.clone()),
            _ => None,
        })
    }

    pub fn span(self) -> Span {
        self.span
    }
}
