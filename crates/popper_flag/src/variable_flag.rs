
use crate::scope_flag::ScopeFlag;

use popper_ast::Span;
use crate::SymbolFlags;

#[derive(PartialEq, Clone, Debug)]
/// variable flag is used to store Variable information
pub struct VariableFlag {
    pub name: String,
    pub value: SymbolFlags,
    pub used: bool,
    pub used_at: Vec<Span>,
    pub scope: ScopeFlag,
    pub mutable: bool,
    pub span: Span
}


impl VariableFlag {
    pub fn new(name: String,
               value: SymbolFlags,
               scope: ScopeFlag,
               mutable: bool,
               span: Span
               ) -> Self {
        Self {
            name,
            value,
            used: false,
            used_at: Vec::new(),
            scope,
            mutable,
            span
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
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
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
        self.variables.iter().filter(|v| &v.scope == scope).collect()
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

}

