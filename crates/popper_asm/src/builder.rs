use crate::register::Register;
use crate::asm_value::AsmValue;

#[derive(Clone, Debug)]
pub enum Assembly<'a> {
    Mov(Register, Box<AsmValue>),
    Add(Register, Box<AsmValue>, Register),
    Sub(Register, Box<AsmValue>, Register),
    Mul(Register, Box<AsmValue>, Register),
    Div(Register, Box<AsmValue>, Register),
    IAdd(Register, Box<AsmValue>),
    ISub(Register, Box<AsmValue>),
    IMul(Register, Box<AsmValue>),
    IDiv(Register, Box<AsmValue>),

    Call(&'a str),
    Label(&'a str, Vec<Assembly<'a>>),
    Ret
}


pub type Program<'a> = Vec<Assembly<'a>>;


#[derive(Clone)]
pub struct Builder<'a> {
    program: Program<'a>
}

impl<'a> Builder<'a> {

    pub fn new() -> Self {
        Self {
            program: vec![]
        }
    }

    pub fn build(&self) -> Program<'a> {
        self.program.clone()
    }

    pub fn build_mov(&mut self, dest: Register, src: AsmValue) {
        self.program.push(Assembly::Mov(dest, Box::new(src)));
    }


    pub fn build_add(&mut self, dest: Register, src: AsmValue, dest2: Register) {
        self.program.push(Assembly::Add(dest, Box::new(src), dest2));
    }

    pub fn build_sub(&mut self, dest: Register, src: AsmValue, dest2: Register) {
        self.program.push(Assembly::Sub(dest, Box::new(src), dest2));
    }

    pub fn build_mul(&mut self, dest: Register, src: AsmValue, dest2: Register) {
        self.program.push(Assembly::Mul(dest, Box::new(src), dest2));
    }

    pub fn build_div(&mut self, dest: Register, src: AsmValue, dest2: Register) {
        self.program.push(Assembly::Div(dest, Box::new(src), dest2));
    }

    pub fn build_iadd(&mut self, dest: Register, src: AsmValue) {
        self.program.push(Assembly::IAdd(dest, Box::new(src)));
    }

    pub fn build_isub(&mut self, dest: Register, src: AsmValue) {
        self.program.push(Assembly::ISub(dest, Box::new(src)));
    }

    pub fn build_imul(&mut self, dest: Register, src: AsmValue) {
        self.program.push(Assembly::IMul(dest, Box::new(src)));
    }

    pub fn build_idiv(&mut self, dest: Register, src: AsmValue) {
        self.program.push(Assembly::IDiv(dest, Box::new(src)));
    }

    pub fn build_call(&mut self, label: &'a str) {
        self.program.push(Assembly::Call(label));
    }

    pub fn build_label(&mut self, label: &'a str, body: Vec<Assembly<'a>>) {
        self.program.push(Assembly::Label(label, body));
    }

    pub fn build_ret(&mut self) {
        self.program.push(Assembly::Ret);
    }


}