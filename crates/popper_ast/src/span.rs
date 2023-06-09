

#[derive(Debug, Clone, Copy)]
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

    /// generate a `^` marker pointing at the start of the span until the end of the span
    pub fn generate_marker(&self, start: usize,  end: usize) -> String {
        let mut marker = String::new();
        for _ in start..self.start {
            marker.push(' ');
        }
        for _ in self.start..self.end {
            marker.push('^');
        }

        for _ in self.end..end {
            marker.push(' ');
        }

        marker
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