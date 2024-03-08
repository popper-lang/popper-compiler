use crate::{ColorConfig, Error};
use ariadne::Fmt;
use popper_ast::Span;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("can't mutate a constant")]
pub struct CantMut {
    pub span: Span,
}

impl CantMut {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl Error for CantMut {
    fn report(&self, color: ColorConfig, source: &str, file: &str) {
        let variable = color.get("variable").expect("variable color not found");

        ariadne::Report::build(
            ariadne::ReportKind::Error,
            file,
            self.span.find_line(source),
        )
        .with_code(24)
        .with_label(
            ariadne::Label::new((file, self.span.into())).with_message(
                "can't mutate a constant".fg(*variable),
            ),
        )
        .with_message("can't mutate a constant".fg(*variable))
        .finish()
        .print((file, ariadne::Source::from(source)))
        .unwrap();
    }
}
