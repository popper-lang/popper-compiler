use std::fmt::{Display, Formatter};

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq, Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "default-trait", derive(Default))]
#[derive(Copy, Clone)]
/// Span is used to save a token / stmt / expr location
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn extract_from_str<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }

    pub fn find_line(&self, source: &str) -> usize {
        let mut line = 1;
        for (i, c) in source.chars().enumerate() {
            if i >= self.start {
                break;
            }
            if c == '\n' {
                line += 1;
            }
        }
        line
    }

}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

impl Into<Span> for std::ops::Range<usize> {
    fn into(self) -> Span {
        Span::new(self.start, self.end)
    }
}