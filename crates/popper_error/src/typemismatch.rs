use popper_ast::Span;
use thiserror::Error;
use ariadne::{Fmt, Label, Report, ReportKind, Source};
use crate::{ColorConfig, Error as PopperError};




#[derive(Error, Debug)]
#[error("type mismatch")]
/// this error is throw when there is a type mismatch
pub struct TypeMismatch {
    pub expected: (Span, String),
    pub found: (Span, String),

}

impl TypeMismatch {
    pub fn new(expected: (Span, String), found: (Span, String)) -> Self {
        Self { expected, found}
    }
}

impl PopperError for TypeMismatch {
    fn report(&self,
              color: ColorConfig,
              source: &str,
              file: &str)  {

        let type_color = color.get("type").expect("type color not found");

        let report = Report::build(ReportKind::Error,
                                       file,
                                       self.expected.0.find_line(
                                           source
                                       )
        )
            .with_code(21)
            .with_message("Incompatible types".to_string())
            .with_label(
                Label::new((file, self.expected.0.into()))
                    .with_message(
                        format!("expected type `{}`",
                                self.expected.1.clone().fg(
                                    *type_color
                                )
                        )
                    )
            );
        report.with_label(
                Label::new((file, self.found.0.into()))
                    .with_message(
                        format!("found type `{}`",
                                self.found.1.clone().fg(
                                    *type_color
                                )
                        )
                    )
            )
            .finish().print((file, Source::from(
            source
        ))).unwrap();
    }
}