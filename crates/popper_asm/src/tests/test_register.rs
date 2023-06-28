use crate::register::Register;
use popper_sbc::instr::Bytecode;

#[test]
fn test_register_bytecode() {
    let register = Register::Inc(1, Box::new(Register::R1));
    let bytecode = register.to_bytecode();
    let register_expected = Register::from_bytecode(bytecode);
    assert_eq!(register, register_expected);
}