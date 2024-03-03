use ariadne::Fmt;
use ariadne::Source;
use thiserror::Error;

use crate::{ColorConfig, Error};
use popper_ast::Span;

#[derive(Error, Debug)]
#[error("already exist")]
pub struct AlreadyExist {
    created_at: Span,
    recreated_at: (String, Span),
}

impl AlreadyExist {
    pub fn new(created_at: Span, recreated_at: (String, Span)) -> Self {
        AlreadyExist {
            created_at,
            recreated_at,
        }
    }
}

impl Error for AlreadyExist {
    fn report(&self, color: ColorConfig, source: &str, file: &str) {
        let variable = color.get("variable").expect("variable color not found");

        let report = ariadne::Report::build(
            ariadne::ReportKind::Error,
            file,
            self.recreated_at.1.find_line(source),
        );

        report
            .with_code(23)
            .with_message(format!(
                "Variable `{}` already exist at `{}`",
                self.recreated_at.0, self.created_at
            ))
            .with_label(
                ariadne::Label::new((file, self.recreated_at.1.into())).with_message(format!(
                    "`{}` already exist",
                    self.recreated_at.0.clone().fg(*variable)
                )),
            )
            .with_label(
                ariadne::Label::new((file, self.created_at.into())).with_message("created here"),
            )
            .finish()
            .print((file, Source::from(source)))
            .unwrap();
    }
}
