use crate::token::*;

pub trait TokenHumanDisplay {
    fn to_human_string(&self) -> String;
}

// TODO: Use crate to derive this
impl TokenHumanDisplay for TokenKind {
    fn to_human_string(&self) -> String {
        match self {
            TokenKind::Invalid => "invalid token".to_owned(),
            TokenKind::Punctuator(punctuator) => punctuator.to_human_string(),
            TokenKind::Keyword(keyword) => keyword.to_human_string(),
            TokenKind::Type(ty) => ty.to_human_string(),
            TokenKind::Identifier(identifier) => identifier.to_human_string(),
            TokenKind::BooleanLiteral(boolean) => boolean.to_human_string(),
            TokenKind::IntegerLiteral(integer) => integer.to_human_string(),
            TokenKind::StringLiteral(string) => string.to_human_string(),
            TokenKind::Text(text) => text.to_human_string()
        }
    }
}

impl TokenHumanDisplay for Punctuator {
    fn to_human_string(&self) -> String {
        let string = match self {
            Punctuator::LeftSquareBracket => "[",
            Punctuator::RightSquareBracket => "]",
            Punctuator::LeftCurlyBracket => "{",
            Punctuator::RightCurlyBracket => "}",
            Punctuator::Comma => ",",
            Punctuator::Colon => ":",
            Punctuator::Equals => "=",
            Punctuator::Dollar => "$",
            Punctuator::At => "@",
            Punctuator::Hash => "#"
        };

        format!("'{string}'")
    }
}

impl TokenHumanDisplay for Type {
    fn to_human_string(&self) -> String {
        "type".to_owned()
    }
}

impl TokenHumanDisplay for Keyword {
    fn to_human_string(&self) -> String {
        let keyword = match self {
            Keyword::Component => "component",
            Keyword::Default => "default",
            Keyword::Text => "text"
        };

        format!("'{keyword}' keyword")
    }
}

impl TokenHumanDisplay for BooleanLiteral {
    fn to_human_string(&self) -> String {
        "boolean".to_owned()
    }
}

impl TokenHumanDisplay for Identifier {
    fn to_human_string(&self) -> String {
        "identifier".to_owned()
    }
}

impl TokenHumanDisplay for IntegerLiteral {
    fn to_human_string(&self) -> String {
        "integer".to_owned()
    }
}

impl TokenHumanDisplay for StringLiteral {
    fn to_human_string(&self) -> String {
        "\"string literal\"".to_owned()
    }
}

impl TokenHumanDisplay for Text {
    fn to_human_string(&self) -> String {
        "(text)".to_owned()
    }
}
