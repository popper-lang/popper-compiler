use ariadne::{Label, ReportKind, Source};
use ariadne::Report;
use thiserror::Error;

use crate::{ColorConfig, Error};
use popper_common::plurialize::plurialize;
use popper_ast::Span;

#[derive(Error, Debug)]
#[error("different length argument")]
pub struct DiffLengthOfArgument {
    expected: usize,
    got: usize,
    function_span: Span
}

impl DiffLengthOfArgument {
    pub fn new(expected: usize, got: usize, function_span: Span) -> Self {
        DiffLengthOfArgument {
            expected,
            got,
            function_span
        }
    }
}

impl Error for DiffLengthOfArgument {
    fn report(&self, _color: ColorConfig, source: &str, file: &str) {
        let report = Report::build(ReportKind::Error, file, self.function_span.find_line(source));

        report.with_code(24)
            .with_message(
                          format!(
                              "Expected {} {} got {} {} ",
                              self.expected,
                              plurialize("argument", self.expected),
                              self.got,
                              plurialize("argument", self.got)
                          ))
            .with_label(
                Label::new((file, self.function_span.into()))
                    .with_message(format!("this function takes {} {}", self.expected, plurialize("argument", self.expected)))
            )
            .finish()
            .print((file, Source::from(
                source
            ))).unwrap();
    }
}