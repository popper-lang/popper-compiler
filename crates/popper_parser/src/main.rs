use popper_parser_new::cursor::Cursor;
use popper_parser_new::parse::Parser;
use popper_parser_new::util::ignore::Ignore;
use popper_parser_new::lexer::Space;

fn main() {
    let mut cursor = Cursor::new("  > ".chars());
    let res = Ignore::<Space, _>::parse(&mut cursor);

    assert_eq!(cursor.collect(), vec!['>']);

}