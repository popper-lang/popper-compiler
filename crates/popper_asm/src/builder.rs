use crate::register::Register;
use crate::asm_value::AsmValue;

#[derive(Clone, Debug, PartialEq)]
pub enum Assembly<'a> {
    Mov(Register, Box<AsmValue>),
    Add(Register, Box<AsmValue>, Register),
    Sub(Register, Box<AsmValue>, Register),
    Mul(Register),
    Div(Register, Box<AsmValue>, Register),
    IAdd(Register, Box<AsmValue>),
    ISub(Register, Box<AsmValue>),
    IDiv(Register, Box<AsmValue>),
    Cmp(Box<AsmValue>, Box<AsmValue>),
    Je(String),
    Jne(String),
    Jmp(String),
    Call(&'a str),
    Ret,
    Nop
}


pub type Program<'a> = Vec<Assembly<'a>>;


#[derive(Clone)]
pub struct Builder<'a> {
    pub program: Program<'a>,
    pub labels: Vec<(String, Program<'a>)>
}

impl<'a> Builder<'a> {

    pub fn new() -> Self {
        Self {
            program: vec![],
            labels: vec![]
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

    pub fn build_mul(&mut self, dest: Register) {
        self.program.push(Assembly::Mul(dest));
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

    pub fn build_idiv(&mut self, dest: Register, src: AsmValue) {
        self.program.push(Assembly::IDiv(dest, Box::new(src)));
    }

    pub fn build_call(&mut self, label: &'a str) {
        self.program.push(Assembly::Call(label));
    }

    pub fn build_label(&mut self, label: String, body: Vec<Assembly<'a>>) {
        self.labels.push((label, body));
    }

    pub fn build_ret(&mut self) {
        self.program.push(Assembly::Ret);
    }

    pub fn build_cmp(&mut self, lhs: AsmValue, rhs: AsmValue) {
        self.program.push(
            Assembly::Cmp(
                Box::new(lhs),
                Box::new(rhs)
            )
        )
    }

    pub fn build_je(&mut self, label: String) {
        self.program.push(Assembly::Je(label));
    }

    pub fn build_jne(&mut self, label: String) {
        self.program.push(Assembly::Jne(label));
    }
    pub fn build_jmp(&mut self, label: String) {
        self.program.push(Assembly::Jmp(label));
    }
    pub fn build_nop(&mut self) {
        self.program.push(Assembly::Nop);
    }

    pub fn push(&mut self, assembly: Assembly<'a>) {
        self.program.push(assembly);
    }
}