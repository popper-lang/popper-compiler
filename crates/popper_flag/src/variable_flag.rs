use std::collections::HashMap;
use crate::scope_flag::ScopeFlag;

use crate::{SymbolFlags, ValueFlag};
use popper_ast::Span;

#[derive(PartialEq, Clone, Debug)]
/// variable flag is used to store Variable information
pub struct VariableFlag {
    pub name: String,
    pub value: SymbolFlags,
    pub used: bool,
    pub used_at: Vec<Span>,
    pub scope: ScopeFlag,
    pub mutable: bool,
    pub span: Span,
}

impl VariableFlag {
    pub fn new(
        name: String,
        value: SymbolFlags,
        scope: ScopeFlag,
        mutable: bool,
        span: Span,
    ) -> Self {
        Self {
            name,
            value,
            used: false,
            used_at: Vec::new(),
            scope,
            mutable,
            span,
        }
    }

    pub fn use_flag(&mut self, span: Span) -> &mut Self {
        self.used = true;
        self.used_at.push(span);
        self
    }
}

/// store Variable in environement
#[derive(Clone, PartialEq, Debug)]
pub struct Environment {
    pub variables: Vec<VariableFlag>,
    pub struct_env: HashMap<String, HashMap<String, ValueFlag>>
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            struct_env: HashMap::new()
        }
    }
    
    pub fn keep_static_member(&self) -> Self {
        let mut new_env = Environment::new();
        new_env.struct_env = self.struct_env.clone();
        for variable in &self.variables {
            if variable.value
                .get_value().unwrap().is_static() {
                new_env.variables.push(variable.clone());
            }
        }
        new_env
    }

    pub fn add_variable(&mut self, variable: VariableFlag) -> &mut Self {
        self.variables.push(variable);
        self
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableFlag> {
        self.variables.iter().find(|v| v.name == name)
    }

    pub fn get_variable_mut(&mut self, name: &str) -> Option<&mut VariableFlag> {
        self.variables.iter_mut().find(|v| v.name == name)
    }

    pub fn variables(&self) -> &Vec<VariableFlag> {
        &self.variables
    }

    pub fn variables_mut(&mut self) -> Vec<&mut VariableFlag> {
        self.variables.iter_mut().collect()
    }

    pub fn variables_in_scope(&self, scope: &ScopeFlag) -> Vec<&VariableFlag> {
        self.variables
            .iter()
            .filter(|v| &v.scope == scope)
            .collect()
    }

    pub fn check_variable(&self, name: &str) -> bool {
        self.variables.iter().any(|v| v.name == name)
    }

    pub fn set_variable(&mut self, name: &str, value: SymbolFlags) -> &mut Self {
        if let Some(variable) = self.get_variable_mut(name) {
            variable.value = value;
        }
        self
    }

    pub fn get_all_variables_name(&self) -> Vec<String> {
        self.variables.iter().map(|v| v.name.clone()).collect()
    }

    pub fn exist(&self, name: String) -> bool {
        self.variables.iter().filter(|x| x.name == name).count() > 0
    }

    pub fn extend(&mut self, other: &mut Environment) -> &mut Self {
        self.variables.append(&mut other.variables);
        self
    }
    
    pub fn add_struct(&mut self, name: String, fields: HashMap<String, ValueFlag>) -> &mut Self {
        self.struct_env.insert(name, fields);
        self
    }
    
    pub fn get_struct(&self, name: &str) -> Option<&HashMap<String, ValueFlag>> {
        self.struct_env.get(name)
    }
}
