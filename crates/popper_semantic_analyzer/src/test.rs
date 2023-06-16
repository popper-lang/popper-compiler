

#[cfg(test)]
mod tests {
    use ariadne::Source;
    use ast::Span;
    use popper_common::error::Error;
    use popper_common::error::generate_color;
    use crate::errors::TypeMismatch;

    #[test]
    fn test_error() {
        let err = TypeMismatch {
            expected: (Span::new(0, 5), "integer".to_string()),
            found: (Span::new(7, 12), "string".to_string()),
        };

        err.report(generate_color(), &Source::from("hello; world;"), "<main>");

    }
}