use inkwell::values::BasicValueEnum;

#[derive(Debug, Clone)]
pub struct LLVMValue<'a> {
    value: BasicValueEnum<'a>,
    flags: Vec<Flag>,
}

impl<'a> LLVMValue<'a> {
    pub fn new(value: BasicValueEnum<'a>) -> Self {
        Self {
            value,
            flags: Vec::new(),
        }
    }

    pub fn set_value(&mut self, value: BasicValueEnum<'a>) {
        self.value = value;
    }

    pub fn add_flag(&mut self, flag: Flag) {
        self.flags.push(flag);
    }

    pub fn get_flag(&self) -> Vec<Flag> {
        self.flags.clone()
    }

    pub fn can_load(&self) -> bool {
        !self.flags.contains(&Flag::CantLoad)
    }

    pub fn basic_value_enum(&self) -> BasicValueEnum<'a> {
        self.value
    }
}

impl<'val> From<LLVMValue<'val>> for BasicValueEnum<'val> {
    fn from(value: LLVMValue<'val>) -> BasicValueEnum<'val> {
        value.basic_value_enum()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    CantLoad,
    None,
}
