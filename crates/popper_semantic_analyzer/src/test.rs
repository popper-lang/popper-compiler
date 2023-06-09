

#[cfg(test)]
mod tests {
    use ariadne::Source;
    use ast::Span;
    use popper_common::error::Error;
    use popper_common::error::generate_color;
    use crate::error::TypeMismatch;

    #[test]
    fn test_error() {
        let err = TypeMismatch {
            expected: Span::new(0, 5),
            found: Span::new(7, 12),
        };

        err.report(generate_color(), &Source::from("hello; world;"), "<main>");

    }
}