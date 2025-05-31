use ariadne::{Label, Report, Source};
use popper_ast::{
    ast::{LineInfo, Span},
    file::FileId,
};

pub trait Diagnostics {
    fn message(&self) -> String;
    fn code(&self) -> u32;
    fn label(&self) -> String;
    fn span(&self) -> Span;

    fn note(&self) -> Option<String> {
        None
    }
}

pub struct ErrorInfo {
    line_info: LineInfo,
    file: FileId,
}

impl ErrorInfo {
    pub fn new(line_info: LineInfo, file: FileId) -> ErrorInfo {
        ErrorInfo { line_info, file }
    }

    pub fn line_info(&self) -> &LineInfo {
        &self.line_info
    }

    pub fn file(&self) -> FileId {
        self.file
    }
}

pub struct Error {
    info: ErrorInfo,
    diagnostics: Box<dyn Diagnostics>,
}

impl Error {
    pub fn new<T: Diagnostics + 'static>(info: ErrorInfo, diagnostics: T) -> Error {
        Error {
            info,
            diagnostics: Box::new(diagnostics),
        }
    }

    pub fn report<'a>(
        &self,
        filename: &'a str,
    ) -> Option<Report<(&'a str, std::ops::Range<usize>)>> {
        let span = self.diagnostics.span();
        let span = (filename, span.lo..span.hi);
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
            if let Some(report) = self.report(file.info().absolute_path()) {
                report
                    .eprint((
                        file.info().absolute_path(),
                        Source::from(file.info().source()),
                    ))
                    .map_err(|x| x.to_string())?;
                Ok(())
            } else {
                Err("report not found".to_string())
            }
        } else {
            Err("file not found".to_string())
        }
    }
}

pub struct ErrorTable {
    errors: Vec<Error>,
}

impl ErrorTable {
    pub fn new() -> ErrorTable {
        ErrorTable { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn print(&self, context: popper_context::Context) -> Result<(), String> {
        for error in &self.errors {
            error.print(context.clone())?;
        }
        Ok(())
    }
}
