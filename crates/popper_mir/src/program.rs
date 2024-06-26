use crate::consts::TypeId;
use crate::function::Function;
use crate::pretty::Pretty;
use crate::types::Types;

#[derive(Debug, Clone)]
pub struct Program {
    pub programs: Vec<ProgramSection>
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            programs: Vec::new()
        }
    }

    pub fn add_program(&mut self, program: ProgramSection) {
        self.programs.push(program);
    }

    pub fn add_function(&mut self, function: Function) {
        self.programs.push(ProgramSection::Function(function));
    }
    
    pub fn add_type_decl(&mut self, ident: TypeId, ty: Types) {
        self.programs.push(ProgramSection::TypeDecl(ident, ty));
    }
    
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        for program in &self.programs {
            if let ProgramSection::Function(function) = program {
                if function.name == name {
                    return Some(function);
                }
            }
        }
        None
    }

    pub fn add_external_function(&mut self, external_function: ExternalFunction) {
        self.programs.push(ProgramSection::ExternalFunction(external_function));
    }
    
    pub fn print_to_string(&self) -> String {
        let mut pretty = Pretty::new(self.clone());
        pretty.pretty_program();
        pretty.print_to_string()
    }
    
    pub fn print_to_file(&self, path: &str) {
        let mut pretty = Pretty::new(self.clone());
        pretty.pretty_program();
        pretty.print_to_file(path);
    }
    
    pub fn print_to_stdout(&self) {
        let mut pretty = Pretty::new(self.clone());
        pretty.pretty_program();
        pretty.print_to_stdout();
    }
}

#[derive(Debug, Clone)]
pub enum ProgramSection {
    Function(Function),
    ExternalFunction(ExternalFunction),
    TypeDecl(TypeId, Types)
}

#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub name: String,
    pub args: Vec<Types>,
    pub ret: Types,
    pub is_var_arg: bool
}

impl ExternalFunction {
    pub fn new(name: String, args: Vec<Types>, ret: Types, is_var_arg: bool) -> Self {
        Self {
            name,
            args,
            ret,
            is_var_arg
        }
    }
}

impl From<ExternalFunction> for Function {
    fn from(external_function: ExternalFunction) -> Self {
        Self::new(external_function.name, external_function.args, external_function.ret)
    }
}