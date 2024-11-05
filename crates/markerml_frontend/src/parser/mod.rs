use chumsky::prelude::*;
use chumsky::primitive::Just;
use crate::ast;
use crate::common::span::Span;
use crate::token::{self, TokenKind};

pub type ParserError = Simple<TokenKind, Span>;

/// Returns implementation of chumsky parser for parsing program modules
pub fn parser() -> impl Parser<TokenKind, ast::Module, Error = ParserError> {
    module()
}

fn module() -> impl Parser<TokenKind, ast::Module, Error = ParserError> {
    component().map(ast::ModuleItem::Component)
        .or(component_definition().map(ast::ModuleItem::ComponentDefinition))
        .repeated()
        .then_ignore(end())
        .map(|items| ast::Module { items })
}

fn component_definition() -> impl Parser<TokenKind, ast::ComponentDefinition, Error = ParserError> {
    keyword(token::Keyword::Component)
        .ignore_then(identifier())
        .then(properties_definition().or_not())
        .then(
            component()
                .repeated()
                .delimited_by(
                    punctuator(token::Punctuator::LeftCurlyBracket),
                    punctuator(token::Punctuator::RightCurlyBracket)
                )
                .or_not()
        )
        .map(|((name, properties), children)| ast::ComponentDefinition { name, properties, children })
}

fn properties_definition() -> impl Parser<TokenKind, ast::PropertiesDefinition, Error = ParserError> {
    let named_property_definition = || identifier()
        .then_ignore(punctuator(token::Punctuator::Colon))
        .then(ty())
        .then(
            punctuator(token::Punctuator::Equals)
                .then(value())
                .or_not()
        )
        .map(|((name, ty), value)| ast::NamedPropertyDefinition { name, ty, default_value: value.map(|(_, x)| x) });

    let default_property_definition = keyword(token::Keyword::Default)
        .ignore_then(named_property_definition())
        .map(|def| ast::PropertyDefinition::Default(def));

    let text_property_definition = keyword(token::Keyword::Text)
        .ignore_then(identifier())
        .map(|name| ast::PropertyDefinition::Text(ast::TextPropertyDefinition { name }));

    let property_definition = choice((
        default_property_definition,
        text_property_definition,
        named_property_definition().map(|def| ast::PropertyDefinition::Named(def))
    ));

    property_definition
        .separated_by(punctuator(token::Punctuator::Comma))
        .allow_trailing()
        .delimited_by(
            punctuator(token::Punctuator::LeftSquareBracket),
            punctuator(token::Punctuator::RightSquareBracket)
        )
        .map(|properties| ast::PropertiesDefinition { properties })
}

fn ty() -> impl Parser<TokenKind, ast::Type, Error = ParserError> {
    choice((
        just(TokenKind::Type(token::Type::String)).to(ast::Type::String),
        just(TokenKind::Type(token::Type::Integer)).to(ast::Type::Integer),
        just(TokenKind::Type(token::Type::Bool)).to(ast::Type::Bool),
        just(TokenKind::Type(token::Type::Slot))
            .then_ignore(punctuator(token::Punctuator::LeftSquareBracket))
            .then_ignore(punctuator(token::Punctuator::RightSquareBracket))
            .to(ast::Type::SlotList),
        just(TokenKind::Type(token::Type::Slot)).to(ast::Type::Slot)
    ))
}

fn component() -> impl Parser<TokenKind, ast::Component, Error = ParserError> {
    let component_name = choice((
        identifier(),
        punctuator(token::Punctuator::At).to(ast::Identifier("@".to_owned())),
        punctuator(token::Punctuator::Hash).to(ast::Identifier("#".to_owned()))
    ));

    recursive(|component| {
        component_name
            .then(properties().or_not())
            .then(text().or_not())
            .then(
                component
                    .repeated()
                    .delimited_by(
                        punctuator(token::Punctuator::LeftCurlyBracket),
                        punctuator(token::Punctuator::RightCurlyBracket)
                    )
                    .or_not()
            )
            .map(|(((name, properties), text), children)| {
                ast::Component { name, properties, children, text }
            })
    })
}

fn properties() -> impl Parser<TokenKind, ast::Properties, Error = ParserError> {
    let default_property = || value();
    let flag_property = || identifier()
        .map(|key| ast::Property::Flag { key });
    let key_value_property = || identifier()
        .then_ignore(punctuator(token::Punctuator::Equals))
        .then(value())
        .map(|(key, value)| ast::Property::KeyValue { key, value });
    let named_property = || key_value_property()
        .or(flag_property());
    let named_properties = || named_property()
        .separated_by(punctuator(token::Punctuator::Comma))
        .at_least(1)
        .allow_trailing();

    named_properties().map(|properties| ast::Properties { default: None, properties })
        .or(
            default_property()
                .then_ignore(punctuator(token::Punctuator::Comma))
                .then(named_properties())
                .map(|(default, properties)| ast::Properties { default: Some(default), properties })
        )
        .or(default_property().map(|default| ast::Properties { default: Some(default), properties: Vec::new() }))
        .delimited_by(
            punctuator(token::Punctuator::LeftSquareBracket),
            punctuator(token::Punctuator::RightSquareBracket)
        )
}

fn value() -> impl Parser<TokenKind, ast::Value, Error = ParserError> {
    choice((
        integer(),
        boolean(),
        string(),
        identifier().map(|ident| ast::Value::Variable(ident))
    ))
}

fn text() -> impl Parser<TokenKind, ast::Text, Error = ParserError> {
    let text_segment = variable_interpolation().or(text_literal());

    text_literal().map(|segment| ast::Text { segments: vec![segment] })
        .then(text_segment.repeated())
        .foldl(|mut acc, x| {
            match x {
                ast::InterpolationSegment::Literal(s) if s.is_empty() => {},
                _ => acc.segments.push(x)
            };
            acc
        })
}

fn string() -> impl Parser<TokenKind, ast::Value, Error = ParserError> {
    let string_segment = variable_interpolation()
        .or(string_literal())
        .map(|a| a);

    string_literal().map(|segment| ast::StringValue { segments: vec![segment] })
        .then(string_segment.repeated())
        .foldl(|mut acc, x| {
            match x {
                ast::InterpolationSegment::Literal(s) if s.is_empty() => {},
                _ => acc.segments.push(x)
            };
            acc
        })
        .map(|value| ast::Value::String(value))
}

fn variable_interpolation() -> impl Parser<TokenKind, ast::InterpolationSegment, Error = ParserError> {
    punctuator(token::Punctuator::Dollar)
        .ignore_then(
            identifier()
                .delimited_by(
                    punctuator(token::Punctuator::LeftCurlyBracket),
                    punctuator(token::Punctuator::RightCurlyBracket)
                )
        )
        .map(|ident| ast::InterpolationSegment::Variable(ident))
}

fn text_literal() -> impl Parser<TokenKind, ast::InterpolationSegment, Error = ParserError> {
    select! {
        TokenKind::Text(token::Text { content, ..}) => ast::InterpolationSegment::Literal(content)
    }
}

fn string_literal() -> impl Parser<TokenKind, ast::InterpolationSegment, Error = ParserError> {
    select! {
        TokenKind::StringLiteral(token::StringLiteral { content, .. }) => ast::InterpolationSegment::Literal(content)
    }
}

fn integer() -> impl Parser<TokenKind, ast::Value, Error = ParserError> {
    select! {
        TokenKind::IntegerLiteral(token::IntegerLiteral(value)) => ast::Value::Integer(value)
    }
}

fn boolean() -> impl Parser<TokenKind, ast::Value, Error = ParserError> {
    select! {
        TokenKind::BooleanLiteral(token::BooleanLiteral(value)) => ast::Value::Bool(value)
    }
}

fn identifier() -> impl Parser<TokenKind, ast::Identifier, Error = ParserError> {
    select! {
        TokenKind::Identifier(token::Identifier(ident)) => ast::Identifier(ident)
    }
}

fn keyword(keyword: token::Keyword) -> Just<TokenKind, TokenKind, ParserError> {
    just(TokenKind::Keyword(keyword))
}

fn punctuator(punctuator: token::Punctuator) -> Just<TokenKind, TokenKind, ParserError> {
    just(TokenKind::Punctuator(punctuator))
}
