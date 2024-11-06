use std::iter::Peekable;
use std::str::{Chars, FromStr};
use unicode_xid::UnicodeXID;
use crate::common::span::{Position, Span};
use crate::token::*;

/// Lexer responsible for producing list of tokens
/// from the source code
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: u32,
    column: u32,
    interpolation_mode: Vec<InterpolationMode>
}

impl<'a> Lexer<'a> {
    /// Creates lexer with the given source code as input
    pub fn new(input: &'a impl AsRef<str>) -> Self {
        Lexer {
            chars: input.as_ref().chars().peekable(),
            line: 1,
            column: 1,
            interpolation_mode: Vec::new()
        }
    }

    /// Produces list of tokens and EOF span from the input source code
    ///
    /// # Example
    /// ```
    /// use markerml_frontend::lexer::Lexer;
    ///
    /// let lexer = Lexer::new(&"box[vertical] {}");
    /// let (tokens, _) = lexer.lex();
    /// println!("{tokens:#?}");
    /// ```
    pub fn lex(mut self) -> (Vec<(TokenKind, Span)>, Span) {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            let start_position = self.position();
            let add_token = |lexer: &Lexer, token: TokenKind| {
                tokens.push((token, lexer.span(start_position.clone())));
            };

            match self.interpolation_mode.last().cloned() {
                Some(InterpolationMode::Start(kind)) => self.lex_interpolation_start(kind, ch, add_token),
                Some(InterpolationMode::Interpolation(kind)) => self.lex_interpolation(kind, ch, add_token),
                Some(InterpolationMode::Literal(kind)) => self.lex_interpolation_literal(kind, ch, add_token),
                None => self.lex_default(ch, add_token)
            }
        }

        (tokens, self.span(self.position()))
    }

    fn lex_interpolation_start(&mut self, kind: InterpolationKind, _ch: char, mut add_token: impl FnMut(&Self, TokenKind)) {
        add_token(self, Punctuator::Dollar.into());
        self.skip();
        add_token(self, Punctuator::LeftCurlyBracket.into());

        self.interpolation_mode.pop();
        self.interpolation_mode.push(InterpolationMode::Interpolation(kind));
    }

    fn lex_interpolation(&mut self, kind: InterpolationKind, ch: char, mut add_token: impl FnMut(&Self, TokenKind)) {
        if ch == '}' {
            self.skip();
            add_token(self, Punctuator::RightCurlyBracket.into());

            self.interpolation_mode.pop();
            self.interpolation_mode.push(InterpolationMode::Literal(kind));
        } else {
            self.lex_default(ch, add_token);
        }
    }

    fn lex_interpolation_literal(&mut self, kind: InterpolationKind, _ch: char, mut add_token: impl FnMut(&Self, TokenKind)) {
        match kind {
            InterpolationKind::String => {
                let token = self.consume_string_literal().into();
                add_token(self, token);
            },
            InterpolationKind::Text => {
                let token = self.consume_text().into();
                add_token(self, token);
            }
        }
    }

    fn lex_default(&mut self, ch: char, mut add_token: impl FnMut(&Self, TokenKind)) {
        if self.try_consume_whitespace(ch) || self.try_consume_line_ending(ch) {

        } else if let Some(res) = self.try_consume_comment(ch) {
            if let Some(token) = res {
                add_token(self, token);
            }
        } else if ch == '(' {
            self.skip();
            let token = self.consume_text().into();
            add_token(self, token);
        } else if ch == '"' {
            self.skip();
            let token = self.consume_string_literal().into();
            add_token(self, token);
        } else if let Some(token) = self.try_consume_integer_literal() {
            add_token(self, token);
        } else if let Some(punctuator) = self.try_consume_punctuator() {
            add_token(self, punctuator.into())
        } else if let Some(token) = self.try_consume_identifier() {
            add_token(self, token);
        } else {
            self.skip();
            add_token(self, TokenKind::Invalid);
        }
    }

    fn try_consume_integer_literal(&mut self) -> Option<TokenKind> {
        let Some(ch) = self.peek() else { return None };
        if !Self::is_integer_head(ch) {
            return None;
        }
        self.skip();

        let mut content = String::from(ch);
        while let Some(ch) = self.peek() {
            if !Self::is_integer_tail(ch) {
                break;
            }

            content.push(ch);
            self.skip();
        }

        Some(match i64::from_str(&content) {
            Ok(number) => IntegerLiteral(number).into(),
            Err(_) => TokenKind::Invalid
        })
    }

    fn try_consume_identifier(&mut self) -> Option<TokenKind> {
        let Some(string) = self.try_consume_identifier_string() else { return None };

        Self::get_type(&string)
            .map(Into::into)
            .or_else(|| Self::get_keyword(&string).map(Into::into))
            .or_else(|| Self::get_boolean_literal(&string).map(Into::into))
            .or_else(|| Some(Identifier(string).into()))
    }

    fn try_consume_identifier_string(&mut self) -> Option<String> {
        let Some(ch) = self.peek() else { return None };
        if !Self::is_identifier_head(ch) {
            return None;
        }
        self.skip();

        let mut content = String::from(ch);
        while let Some(ch) = self.peek() {
            if !Self::is_identifier_tail(ch) {
                break;
            }

            content.push(ch);
            self.skip();
        }

        Some(content)
    }

    fn try_consume_punctuator(&mut self) -> Option<Punctuator> {
        let Some(ch) = self.peek() else { return None };

        let punctuator = match ch {
            '[' => Punctuator::LeftSquareBracket,
            ']' => Punctuator::RightSquareBracket,
            '{' => Punctuator::LeftCurlyBracket,
            '}' => Punctuator::RightCurlyBracket,
            ',' => Punctuator::Comma,
            ':' => Punctuator::Colon,
            '=' => Punctuator::Equals,
            '$' => Punctuator::Dollar,
            '@' => Punctuator::At,
            '#' => Punctuator::Hash,
            _ => return None
        };
        self.skip();

        Some(punctuator)
    }

    fn consume_string_literal(&mut self) -> StringLiteral {
        let (content, is_closed) = self.consume_text_until('"', InterpolationKind::String);
        StringLiteral {
            content,
            is_closed
        }
    }

    fn consume_text(&mut self) -> Text {
        let (content, is_closed) = self.consume_text_until(')', InterpolationKind::Text);
        Text {
            content,
            is_closed
        }
    }

    fn consume_text_until(&mut self, delimiter: char, interpolation_kind: InterpolationKind) -> (String, bool) {
        let mut text = String::new();
        while let Some(ch) = self.next() {
            match ch {
                ch if ch == delimiter => {
                    self.interpolation_mode.pop();
                    return (text, true);
                },
                ch if self.try_consume_line_ending(ch) => {
                    while let Some(ch) = self.peek() {
                        if !self.try_consume_whitespace(ch) {
                            break;
                        }
                    }

                    match self.peek() {
                        Some(ch) if ch != delimiter && !Self::is_newline(ch) => text.push(' '),
                        _ => {}
                    }
                },
                '\\' => {
                    let ch = self.consume_if(|ch| ch == '$' || ch == delimiter)
                        .unwrap_or(ch);
                    text.push(ch);
                },
                '$' => {
                    if let Some('{') = self.peek() {
                        self.interpolation_mode.push(InterpolationMode::Start(interpolation_kind));
                        return (text, true);
                    } else {
                        text.push(ch);
                    };
                },
                _ => {
                    text.push(ch);
                }
            }
        }

        (text, false)
    }

    fn try_consume_comment(&mut self, ch: char) -> Option<Option<TokenKind>> {
        if ch != '/' {
            return None;
        }

        let delimiter = match self.interpolation_mode.last() {
            Some(InterpolationMode::Interpolation(_)) => Some('}'),
            _ => None
        };

        self.skip();
        if self.consume_if_eq('/').is_some() {
            while let value@Some(ch) = self.peek() {
                if Self::is_newline(ch) || value == delimiter {
                    break;
                }

                self.skip();
            }

            Some(None)
        } else {
            Some(Some(TokenKind::Invalid))
        }
    }

    fn try_consume_whitespace(&mut self, ch: char) -> bool {
        if Self::is_whitespace_except_newline(ch) {
            self.skip();
            true
        } else {
            false
        }
    }

    fn try_consume_line_ending(&mut self, ch: char) -> bool {
        match ch {
            '\r' => {
                self.skip();
                self.consume_if_eq('\n');
                self.advance_line();
                true
            },
            '\n' => {
                self.skip();
                self.advance_line();
                true
            },
            _ => false
        }
    }

    fn consume_if_eq(&mut self, ch: char) -> Option<char> {
        self.consume_if(|x| x == ch)
    }

    fn consume_if(&mut self, pred: impl FnOnce(char) -> bool) -> Option<char> {
        match self.peek() {
            Some(ch) if pred(ch) => {
                self.skip();
                Some(ch)
            },
            _ => None
        }
    }

    fn advance_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    fn span(&self, start: Position) -> Span {
        let end = self.position();

        Span {
            start,
            end
        }
    }

    fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column
        }
    }

    fn skip(&mut self) {
        _ = self.next();
    }

    fn next(&mut self) -> Option<char> {
        let Some(ch) = self.chars.next() else { return None };
        self.column += 1;

        Some(ch)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    fn get_boolean_literal(identifier: &str) -> Option<BooleanLiteral> {
        Some(match identifier {
            "true" => BooleanLiteral(true),
            "false" => BooleanLiteral(false),
            _ => return None
        })
    }

    fn get_type(identifier: &str) -> Option<Type> {
        Some(match identifier {
            "string" => Type::String,
            "integer" => Type::Integer,
            "bool" => Type::Bool,
            "slot" => Type::Slot,
            _ => return None
        })
    }

    fn get_keyword(identifier: &str) -> Option<Keyword> {
        Some(match identifier {
            "component" => Keyword::Component,
            "default" => Keyword::Default,
            "text" => Keyword::Text,
            _ => return None
        })
    }

    fn is_identifier_tail(ch: char) -> bool {
        UnicodeXID::is_xid_continue(ch) || ch == '_'
    }

    fn is_identifier_head(ch: char) -> bool {
        UnicodeXID::is_xid_start(ch) || ch == '_'
    }

    fn is_integer_tail(ch: char) -> bool {
        matches!(ch, '0'..='9')
    }

    fn is_integer_head(ch: char) -> bool {
        matches!(ch, '-' | '0'..='9')
    }

    fn is_newline(ch: char) -> bool {
        ch == '\n' || ch == '\r'
    }

    fn is_whitespace_except_newline(ch: char) -> bool {
        ch == ' ' || ch == '\t'
    }
}

#[derive(Clone)]
enum InterpolationMode {
    Start(InterpolationKind),
    Interpolation(InterpolationKind),
    Literal(InterpolationKind)
}

#[derive(Copy, Clone)]
enum InterpolationKind {
    Text,
    String
}


#[cfg(test)]
mod tests {
    use super::*;

    fn lex_with_spans(code: &str) -> Vec<(TokenKind, Span)> {
        let (tokens, _ ) = Lexer::new(&code).lex();

        tokens
    }

    fn lex(code: &str) -> Vec<TokenKind> {
        lex_with_spans(code).into_iter()
            .map(|(token, _)| token)
            .collect()
    }

    fn to_tokens<T: Into<TokenKind>>(items: Vec<T>) -> Vec<TokenKind> {
        items.into_iter().map(Into::into).collect()
    }

    #[test]
    fn empty() {
        let code = "";
        let res = vec![] ;

        assert_eq!(lex(code), res);
    }

    #[test]
    fn keywords() {
        let code = "component default text";
        let res = vec![Keyword::Component, Keyword::Default, Keyword::Text];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn types() {
        let code = "string integer bool slot";
        let res = vec![Type::String, Type::Integer, Type::Bool, Type::Slot];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn punctuators() {
        let code = "[]{},:=$@#";
        let res = vec![
            Punctuator::LeftSquareBracket,
            Punctuator::RightSquareBracket,
            Punctuator::LeftCurlyBracket,
            Punctuator::RightCurlyBracket,
            Punctuator::Comma,
            Punctuator::Colon,
            Punctuator::Equals,
            Punctuator::Dollar,
            Punctuator::At,
            Punctuator::Hash
        ];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn boolean_literals() {
        let code = "true false";
        let res = vec![BooleanLiteral(true), BooleanLiteral(false)];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn integer_literal_zero() {
        let code = "0";
        let res = vec![IntegerLiteral(0)];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn integer_literal_negative_zero() {
        let code = "-0";
        let res = vec![IntegerLiteral(0)];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn integer_literal_positive() {
        let code = "123 42 90021 967";
        let res = vec![IntegerLiteral(123), IntegerLiteral(42), IntegerLiteral(90021), IntegerLiteral(967)];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn integer_literal_negative() {
        let code = "-123 -42 -666";
        let res = vec![IntegerLiteral(-123), IntegerLiteral(-42), IntegerLiteral(-666)];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn integer_literal_leading_zeros() {
        let code = "000014";
        let res = vec![IntegerLiteral(14)];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn two_integer_literals() {
        let code = "42-666";
        let res = vec![IntegerLiteral(42), IntegerLiteral(-666)];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn identifier() {
        let code = "something";
        let res = vec![Identifier::new("something")];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn identifier_with_underscore() {
        let code = "some_component other___component component_";
        let res = vec![
            Identifier::new("some_component"),
            Identifier::new("other___component"),
            Identifier::new("component_"),
        ];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn identifier_leading_underscore() {
        let code = "_component_name";
        let res = vec![Identifier::new("_component_name")];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn identifier_with_digits() {
        let code = "component1424 other42component 53box";
        let res = vec![
            Identifier::new("component1424").into(),
            Identifier::new("other42component").into(),
            IntegerLiteral(53).into(),
            Identifier::new("box").into()
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn invalid_token() {
        let code = "component_name % ^ wow *default";
        let res = vec![
            Identifier::new("component_name").into(),
            TokenKind::Invalid,
            TokenKind::Invalid,
            Identifier::new("wow").into(),
            TokenKind::Invalid,
            Keyword::Default.into()
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn string_literal() {
        let code = r#""text in quotes" " other string  ""#;
        let res = vec![
            StringLiteral {
                content: "text in quotes".to_owned(),
                is_closed: true,
            },
            StringLiteral {
                content: " other string  ".to_owned(),
                is_closed: true
            }
        ];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn string_literal_interpolated() {
        let code = r#""User age is: ${user_age}""#;
        let res = vec![
            StringLiteral {
                content:  "User age is: ".to_owned(),
                is_closed: true,
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_age").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into()
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn string_literal_not_closed() {
        let code = r#""this string literal is not closed..."#;
        let res = vec![
            StringLiteral {
                content: "this string literal is not closed...".to_owned(),
                is_closed: false
            }
        ];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn string_literal_interpolated_multiple() {
        let code = r#""User age is: ${user_age} and their height is ${user_height}""#;
        let res = vec![
            StringLiteral {
                content:  "User age is: ".to_owned(),
                is_closed: true,
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_age").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: " and their height is ".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_height").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into(),
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn string_literal_interpolation_not_closed() {
        let code = r#""User age is: ${user_age wait""#;
        let res = vec![
            StringLiteral {
                content:  "User age is: ".to_owned(),
                is_closed: true,
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_age").into(),
            Identifier::new("wait").into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: false
            }.into(),
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn string_literal_interpolation_nested() {
        let code = r#""This ${is "some ${cursed_stuff}"}""#;
        let res = vec![
            StringLiteral {
                content:  "This ".to_owned(),
                is_closed: true,
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("is").into(),
            StringLiteral {
                content: "some ".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("cursed_stuff").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into(),
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn string_literal_multiline() {
        let code = r#""this
            is
            multiline
            string
            literal
        ""#;
        let res = vec![
            StringLiteral {
                content: "this is multiline string literal".to_owned(),
                is_closed: true
            }
        ];

        assert_eq!(lex(code), to_tokens(res));
    }

    #[test]
    fn string_interpolation_multiline() {
        let code = r#""this
            is ${


            a
            }

            multiline
            ${
            interpolation}
        ""#;
        let res = vec![
            StringLiteral {
                content: "this is ".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("a").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: " multiline ".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("interpolation").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into()
        ];

        assert_eq!(lex(code), res);
    }


    #[test]
    fn code_spans_simple() {
        let code = "123456789 hello  component";
        let res = vec![
            (IntegerLiteral(123456789).into(), Span {
                start: Position { line: 1, column: 1 },
                end: Position { line: 1, column: 10 }
            }),
            (Identifier::new("hello").into(), Span {
                start: Position { line: 1, column: 11 },
                end: Position { line: 1, column: 16 }
            }),
            (Keyword::Component.into(), Span {
                start: Position { line: 1, column: 18 },
                end: Position { line: 1, column: 27 }
            })
        ];

        assert_eq!(lex_with_spans(code), res);
    }

    #[test]
    fn code_spans_multiline() {
        let code = r#"
123
hello world $

 42
        "#;

        let res = vec![
            (IntegerLiteral(123).into(), Span {
                start: Position { line: 2, column: 1 },
                end: Position { line: 2, column: 4 }
            }),
            (Identifier::new("hello").into(), Span {
                start: Position { line: 3, column: 1 },
                end: Position { line: 3, column: 6 }
            }),
            (Identifier::new("world").into(), Span {
                start: Position { line: 3, column: 7 },
                end: Position { line: 3, column: 12 }
            }),
            (Punctuator::Dollar.into(), Span {
                start: Position { line: 3, column: 13 },
                end: Position { line: 3, column: 14 }
            }),
            (IntegerLiteral(42).into(), Span {
                start: Position { line: 5, column: 2 },
                end: Position { line: 5, column: 4 }
            }),
        ];

        assert_eq!(lex_with_spans(code), res);
    }

    #[test]
    fn comments() {
        let code = r#"
            // Comment
            123 //
            // Some comment
            // blah blah
            456
            //
            // Other comment
            // 666
            789 // Cool number!
        "#;
        let res = vec![
            IntegerLiteral(123),
            IntegerLiteral(456),
            IntegerLiteral(789)
        ];

        assert_eq!(lex(code), to_tokens(res))
    }

    #[test]
    fn code_spans_string_interpolation() {
        let code = r#"
paragraph(${
    user
} is)"
        "#;
        let res = vec![
            (Identifier::new("paragraph").into(), Span {
                start: Position { line: 2, column: 1 },
                end: Position { line: 2, column: 10 }
            }),
            (
                Text {
                    content: "".to_owned(),
                    is_closed: true
                }.into(),
                Span {
                    start: Position { line: 2, column: 10 },
                    end: Position { line: 2, column: 11 }
                }
            ),
            (Punctuator::Dollar.into(), Span {
                start: Position { line: 2, column: 11 },
                end: Position { line: 2, column: 12 }
            }),
            (Punctuator::LeftCurlyBracket.into(), Span {
                start: Position { line: 2, column: 12 },
                end: Position { line: 2, column: 13 }
            }),
            (Identifier::new("user").into(), Span {
                start: Position { line: 3, column: 5 },
                end: Position { line: 3, column: 9 }
            }),
            (Punctuator::RightCurlyBracket.into(), Span {
                start: Position { line: 4, column: 1 },
                end: Position { line: 4, column: 2 }
            }),
            (
                Text {
                    content: " is".to_owned(),
                    is_closed: true
                }.into(),
                Span {
                    start: Position { line: 4, column: 2 },
                    end: Position { line: 4, column: 6 }
                }
            ),
        ];

        assert_eq!(lex_with_spans(code), res);
    }

    #[test]
    fn comments_in_string_interpolation() {
        let code = r#"
            // Comment
            "${user_age //cool variable}" //comment
            // comment
        "#;
        let res = vec![
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_age").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into()
        ];

        assert_eq!(lex(code), res)
    }

    #[test]
    fn comments_in_string_interpolation_multiline() {
        let code = r#"
            // Comment
            "${user_age //cool variable

            // comment
            something
            }" //comment
            // comment
        "#;
        let res = vec![
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_age").into(),
            Identifier::new("something").into(),
            Punctuator::RightCurlyBracket.into(),
            StringLiteral {
                content: "".to_owned(),
                is_closed: true
            }.into()
        ];

        assert_eq!(lex(code), res)
    }

    #[test]
    fn text() {
        let code = r#"
            box(Hello world  )
        "#;
        let res = vec![
            Identifier::new("box").into(),
            Text {
                content: "Hello world  ".to_owned(),
                is_closed: true,
            }.into()
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn text_multiline() {
        let code = r#"
            paragraph(Hello

            world
            )
        "#;
        let res = vec![
            Identifier::new("paragraph").into(),
            Text {
                content: "Hello world".to_owned(),
                is_closed: true,
            }.into()
        ];

        assert_eq!(lex(code), res);
    }

    #[test]
    fn text_interpolated_multiline() {
        let code = r#"
            paragraph(User first name is ${user_first_name}
                and the last name is ${user_last_name}
            )
        "#;
        let res = vec![
            Identifier::new("paragraph").into(),
            Text {
                content: "User first name is ".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_first_name").into(),
            Punctuator::RightCurlyBracket.into(),
            Text {
                content: " and the last name is ".to_owned(),
                is_closed: true
            }.into(),
            Punctuator::Dollar.into(),
            Punctuator::LeftCurlyBracket.into(),
            Identifier::new("user_last_name").into(),
            Punctuator::RightCurlyBracket.into(),
            Text {
                content: "".to_owned(),
                is_closed: true
            }.into()
        ];

        assert_eq!(lex(code), res);
    }
}

// TODO: Fix a couple of failing tests related to text
// TODO: Test escapes for '$', ')' and '"'
