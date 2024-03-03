use crate::{ColorConfig, Error};
use ariadne::Fmt;
use popper_ast::Span;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("field not found")]
pub struct FieldNotFound {
    pub name: String,
    pub span: Span,
    pub similar: Option<String>,
}

impl FieldNotFound {
    pub fn new(name: String, span: Span, similar: Option<String>) -> Self {
        Self {
            name,
            span,
            similar,
        }
    }
}

impl Error for FieldNotFound {
    fn report(&self, color: ColorConfig, source: &str, file: &str) {
        let variable = color.get("variable").expect("variable color not found");

        let mut report = ariadne::Report::build(
            ariadne::ReportKind::Error,
            file,
            self.span.find_line(source),
        )
        .with_code(24)
        .with_label(
            ariadne::Label::new((file, self.span.into())).with_message(format!(
                "field `{}` not found",
                self.name.clone().fg(*variable)
            )),
        )
        .with_message(format!(
            "field `{}` not found",
            self.name.clone().fg(*variable)
        ));
        if let Some(similar) = &self.similar {
            report =
                report
                    .with_label(ariadne::Label::new((file, self.span.into())).with_message(
                        format!("did you mean `{}`?", similar.clone().fg(*variable)),
                    ));
        }

        report
            .finish()
            .print((file, ariadne::Source::from(source)))
            .unwrap();
    }
}
