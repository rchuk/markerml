use std::fmt::{Display, Formatter};
use enum_display_derive::Display;

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq)]
pub enum TokenKind {
    Invalid,
    Punctuator(Punctuator),
    Keyword(Keyword),
    Type(Type),
    Identifier(Identifier),
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    StringLiteral(StringLiteral),
    Text(Text)
}

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq)]
pub enum Punctuator {
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Comma,
    Colon,
    Equals,
    Dollar,
    At,
    Hash
}

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    String,
    Integer,
    Bool,
    Slot
}

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq)]
pub enum Keyword {
    Component,
    Default,
    Text
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BooleanLiteral(pub bool);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IntegerLiteral(pub i64);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StringLiteral {
    pub content: String,
    pub is_closed: bool
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Text {
    pub content: String,
    pub is_closed: bool
}


impl Into<TokenKind> for Punctuator {
    fn into(self) -> TokenKind {
        TokenKind::Punctuator(self)
    }
}

impl Into<TokenKind> for Keyword {
    fn into(self) -> TokenKind {
        TokenKind::Keyword(self)
    }
}

impl Into<TokenKind> for Type {
    fn into(self) -> TokenKind {
        TokenKind::Type(self)
    }
}

impl Into<TokenKind> for Identifier {
    fn into(self) -> TokenKind {
        TokenKind::Identifier(self)
    }
}

impl Into<TokenKind> for BooleanLiteral {
    fn into(self) -> TokenKind {
        TokenKind::BooleanLiteral(self)
    }
}

impl Into<TokenKind> for IntegerLiteral {
    fn into(self) -> TokenKind {
        TokenKind::IntegerLiteral(self)
    }
}

impl Into<TokenKind> for StringLiteral {
    fn into(self) -> TokenKind {
        TokenKind::StringLiteral(self)
    }
}

impl Into<TokenKind> for Text {
    fn into(self) -> TokenKind {
        TokenKind::Text(self)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier({})", self.0)
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BooleanLiteral({})", self.0)
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerLiteral({})", self.0)
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let closed_string = if self.is_closed { "" } else { "<unclosed>" };
        write!(f, "StringLiteral{}({})", closed_string, self.content)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let closed_string = if self.is_closed { "" } else { "<unclosed>" };
        write!(f, "Text{}({})", closed_string, self.content)
    }
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier(name.to_owned())
    }
}

