use ariadne::Source;
use ariadne::Fmt;
use thiserror::Error;
use popper_ast::visitor::ExprVisitor;
use popper_common::error::{ColorConfig, Error, source_to_string};
use popper_ast::Span;
use crate::tool::name_similarity::find_similar_name;

#[derive(Error, Debug)]
#[error("name not found")]
pub struct NameNotFound {
    name: (Span, String),
    pub name_more_closed: Option<String>
}

impl NameNotFound {
    pub fn new(name: (Span, String), name_more_closed: Option<String>) -> Self {
        Self { name, name_more_closed }
    }
}

impl Error for NameNotFound {
    fn report(&self,
              color: ColorConfig,
              source: &str,
              file: &str) {

        let variable = color.get("variable").expect("variable color not found");

        let mut report = ariadne::Report::build(ariadne::ReportKind::Error,
                                                file,
                                                self.name.0.find_line(
                                                    source
                                                )
        );

        report = report.with_code(22)
            .with_message(format!("Variable `{}` not found", self.name.1))
            .with_label(
                ariadne::Label::new((file, self.name.0.into()))
                    .with_message(
                        format!("`{}` not found", self.name.1.clone().fg(*variable))
                    )
            )
            ;

        if let Some(name_more_closed) = &self.name_more_closed {
            report = report.with_note(
                format!("Did you mean `{}`?", name_more_closed.clone().fg(*variable))
            );
        }

        report.finish().print((file, Source::from(
            source
        ))).unwrap();
    }
}
