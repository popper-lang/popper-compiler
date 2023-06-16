use popper_parser::parser::popper::FileParser;

pub fn get_ast(input: &str) -> FileParser {
    FileParser::new().parse(input).unwrap()
}