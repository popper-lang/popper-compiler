use ariadne::{Label, Report, Source};
use popper_ast::{ast::{LineInfo, Span}, file::FileId};
use popper_error_macro::Diagnostics;

pub trait Diagnostics {
    fn message(&self) -> &str;
    fn code(&self) -> u32;
    fn label(&self) -> &str;
    fn span(&self) -> Span;

    fn note(&self) -> Option<&str> {
        None
    }
}

pub struct ErrorInfo {
    line_info: LineInfo,
    file: FileId,
}




pub struct Error {
    info: ErrorInfo,
    diagnostics: Box<dyn Diagnostics>,
}

impl<T: Diagnostics> Error {
    pub fn new(info: ErrorInfo, diagnostics: T) -> Error {
        Error { info, diagnostics: Box::new(diagnostics) }
    }


    pub fn report(&self) -> Option<Report> {
        let span = self.diagnostics.span();
        let span = span.hi..span.lo;
        let message = self.diagnostics.message();
        let label = self.diagnostics.label();
        let code = self.diagnostics.code();
        let report = Report::build(ariadne::ReportKind::Error, span.clone())
            .with_code(code)
            .with_message(message)
            .with_label(Label::new(span).with_message(label))
            .finish();
        Some(report)
    }

    pub fn print(&self, context: popper_context::Context) -> Result<(), String> {
        let file = context.get_file(self.info.file);
        if let Some(file) = file {
            if let Some(report) = self.report() {
                report.eprint(
                    (file.info().absolute_path(), Source::from(&file.info().source())),
                ).map_err(|x| x.to_string())?;
                Ok(())
            } else {
                Err("report not found".to_string())
            }
        } else {
            Err("file not found".to_string())
        }

    }
}
