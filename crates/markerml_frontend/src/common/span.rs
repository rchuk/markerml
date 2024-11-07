use std::ops::{BitOr, BitOrAssign, Range};

/// Represents span in the source code
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position
}

/// Represents line and column position in the source code
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
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

impl BitOr<Span> for Span {
    type Output = Span;

    fn bitor(self, rhs: Span) -> Self::Output {
        let Span { start: start_lhs, end: end_lhs } = self;
        let Span { start: start_rhs, end: end_rhs } = rhs;

        Span {
            start: start_lhs.min(start_rhs),
            end: end_lhs.max(end_rhs)
        }
    }
}

impl BitOrAssign<Span> for Span {
    fn bitor_assign(&mut self, rhs: Span) {
        self.start = self.start.clone().min(rhs.start);
        self.end = self.end.clone().max(rhs.end);
    }
}
