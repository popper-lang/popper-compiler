use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub popper); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(popper::FileParser::new().parse("22").is_ok());
}