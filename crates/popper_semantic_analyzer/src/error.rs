use ast::Span;
use thiserror::Error;
use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use popper_common::error::{ColorConfig, Error as PopperError};
use popper_common::error::source_to_string;



#[derive(Error, Debug)]
#[error("type mismatch")]
pub struct TypeMismatch {
    pub expected: Span,
    pub found: Span,
}

impl PopperError for TypeMismatch {
    fn report(&self,
              color: ColorConfig,
              source: &Source,
              file: &str)  {
        let type_color = color.get("type").expect("type color not found");

        let mut report = Report::build(ReportKind::Error,
                                       file,
                                       self.expected.find_line(
                                           source_to_string(source).as_str()
                                       )
        )
            .with_code(21)
            .with_message(format!("Incompatible types"))
            .with_label(
                Label::new((file, self.expected.into()))
                    .with_message(
                        format!("expected type `{}`",
                                self.expected.extract_from_str(
                                    source_to_string(source).as_str()
                                ).fg(
                                    type_color.clone()
                                )
                        )
                    )
            )
            .with_label(
                Label::new((file, self.found.into()))
                    .with_message(
                        format!("found type `{}`",
                                self.found.extract_from_str(
                                    source_to_string(source).as_str()
                                ).fg(
                                    type_color.clone()
                                )
                        )
                    )
            );

        report.finish().print((file, Source::from(
            source_to_string(source).as_str()
        ))).unwrap();
    }
}