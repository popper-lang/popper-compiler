use crate::ast::Expr;
use crate::vm::Vm;
use crate::vm::Evaluateur;
use crate::errors::*;
use crate::value::Value;
use crate::ast::IOpType;

#[derive(Clone)]
pub struct IOp {
    pub op: IOpType,
    pub name: String,
    pub value: Box<Expr>
}

impl Evaluateur for IOp {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let v = self.value.eval(vm)?;
        match self.op {
            IOpType::IAdd => vm.iadd(self.name.clone(), v),
            IOpType::ISub => vm.isub(self.name.clone(), v),
            IOpType::IMul => vm.imul(self.name.clone(), v),
            IOpType::IDiv => vm.idiv(self.name.clone(), v)
        }
    }
}