use std::ops::Range;

/// Represents span in the source code
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position
}

/// Represents line and column position in the source code
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub line: u32,
    pub column: u32
}

impl chumsky::Span for Span {
    type Context = ();
    type Offset = Position;

    fn new(_context: Self::Context, range: Range<Self::Offset>) -> Self {
        Span {
            start: range.start,
            end: range.end
        }
    }

    fn context(&self) -> Self::Context {
        ()
    }

    fn start(&self) -> Self::Offset {
        self.start.clone()
    }

    fn end(&self) -> Self::Offset {
        self.end.clone()
    }
}
