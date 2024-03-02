
use thiserror::Error;
use crate::Error as PopperError;
use popper_ast::Span;

#[derive(Error, Debug)]
#[error("Return is not allowed outside function")]
pub struct ReturnNotAllowed {
    span: Span,
}

impl ReturnNotAllowed {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl PopperError for ReturnNotAllowed {
    fn report(&self,
              _color: crate::ColorConfig,
              source: &str,
              file: &str) {

        let report = ariadne::Report::build(ariadne::ReportKind::Error,
                                                file,
                                                self.span.find_line(
                                                    source
                                                )
        );

        report.with_code(23)
            .with_message("Return is not allowed outside function".to_string())
            .with_label(
                ariadne::Label::new((file, self.span.into()))
                    .with_message(
                        "Return is not allowed outside function".to_string()
                    )
            )
            .finish().print((file, ariadne::Source::from(
                source
            ))).unwrap();
    }
}
