use std::ops::Range;

use popper_ast::Span;
use thiserror::Error;
use ariadne::{Label, Report, ReportKind, Source};
use crate::{ColorConfig, Error as PopperError};

#[derive(Debug, Error)]
#[error("Type not specified")]
pub struct TypeNotSpecified {
    pub span: Span,
    ty: String
}

impl TypeNotSpecified {
    pub fn new(span: Span, ty: String) -> Self {
        Self {
            span,
            ty
        }
    }
}

impl PopperError for TypeNotSpecified {
    fn report(&self,
        color: ColorConfig,
        source: &str,
        file: &str) {

            let ty = color.get("type").expect("Failed to get color for type");

            let report = Report::<(&str, Range<usize>)>::build(ReportKind::Error,
                                           file,
                                           self.span.find_line(
                                               source
                                           )
            )
            .with_code(25)
            .with_message("Type not specified")
            .with_label(
                Label::new(
                    (file, self.span.into())
                )
            )
            .with_note(
                format!("Type not specified for {}", ty.paint(self.ty.clone()))
            );

            report
                .finish()
                .print(
                    (file, Source::from(
                            source
                    )))
                .unwrap();
        }
}
