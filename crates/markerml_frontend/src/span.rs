use std::ops::Range;

/// Represents span in the source code
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

/// Represents byte position in the source code
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position(pub usize);

impl From<Span> for miette::SourceSpan {
    fn from(span: Span) -> Self {
        miette::SourceSpan::from(Range {
            start: span.start.0,
            end: span.end.0,
        })
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(span: pest::Span<'_>) -> Self {
        Span {
            start: Position(span.start()),
            end: Position(span.end()),
        }
    }
}
