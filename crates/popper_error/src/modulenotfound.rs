use ariadne::Source;
use crate::ColorConfig;
use crate::Error;
use popper_ast::Span;
use thiserror::Error;
use ariadne::Fmt;

#[derive(Debug, Error)]
#[error("module not found")]
pub struct ModuleNotFound {
    module: String,
    span: Span
}

impl ModuleNotFound {
    pub fn new(module: String, span: Span) -> Self {
        ModuleNotFound {
            module,
            span
        }
    }
}

impl Error for ModuleNotFound {
    fn report(&self, color: ColorConfig, source: &str, file: &str) {
        let module = color.get("module").expect("module color not found");

        let report = ariadne::Report::build(ariadne::ReportKind::Error,
                                            file,
                                            self.span.find_line(
                                                source
                                            )
        );

        report
            .with_code(23)
            .with_message(
                format!("Module `{}` not found", self.module)
            )
            .with_label(
                ariadne::Label::new((file, self.span.into()))
                    .with_message(
                        format!("`{}` not found", self.module.clone().fg(*module))
                    )
            )
            .finish()
            .print((file, Source::from(
                source
            )))
            .unwrap()
        ;
    }
}