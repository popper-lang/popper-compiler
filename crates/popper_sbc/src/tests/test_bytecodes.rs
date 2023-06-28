
use crate::instr::Instruction;
use crate::value::StrPtr;
use crate::value::Literal;
use crate::instr::Bytecode;

#[test]
pub fn test_literal_bytecode() {
    let literal = Literal::Integer(1);
    let bytecode = literal.to_bytecode();
    let literal_expected = Literal::from_bytecode(bytecode);
    assert_eq!(literal, literal_expected);

    let literal = Literal::Float(1.0);
    let bytecode = literal.to_bytecode();
    let literal_expected = Literal::from_bytecode(bytecode);
    assert_eq!(literal, literal_expected);

    let literal = StrPtr::from_str("hello");
    let bytecode = literal.to_bytecode();
    let literal_expected = StrPtr::from_bytecode(bytecode);
    unsafe {
        assert_eq!(literal.as_str(), literal_expected.as_str());
    }

    let literal = Literal::Boolean(true);
    let bytecode = literal.to_bytecode();
    let literal_expected = Literal::from_bytecode(bytecode);
    assert_eq!(literal, literal_expected);

    let literal = Literal::Null;
    let bytecode = literal.to_bytecode();
    let literal_expected = Literal::from_bytecode(bytecode);
    assert_eq!(literal, literal_expected);
}

type LiteralTuple = (Literal, Literal);

#[test]
pub fn test_tuple_bytecode() {
    let tuple: LiteralTuple = (Literal::Integer(1), Literal::Integer(2));
    let bytecode = tuple.to_bytecode();
    let tuple_expected = LiteralTuple::from_bytecode(bytecode);
    assert_eq!(tuple, tuple_expected);
}