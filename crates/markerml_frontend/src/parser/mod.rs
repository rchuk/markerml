use chumsky::prelude::*;
use chumsky::primitive::Just;
use crate::ast;
use crate::common::span::Span;
use crate::token::{self, TokenKind};

pub type ParserError = Simple<TokenKind, Span>;

/// Returns implementation of chumsky parser for parsing program modules
pub fn parser() -> impl Parser<TokenKind, ast::Module<Span>, Error = ParserError> {
    module()
}

fn module() -> impl Parser<TokenKind, ast::Module<Span>, Error = ParserError> {
    component()
        .map(ast::ModuleItem::Component)
        .or(component_definition().map(ast::ModuleItem::ComponentDefinition))
        .repeated()
        .then_ignore(end())
        .map_with_span(|items, span| ast::Module { span, items })
}

fn component_definition() -> impl Parser<TokenKind, ast::ComponentDefinition<Span>, Error = ParserError> {
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
        .map_with_span(|((name, properties), children), span| {
            ast::ComponentDefinition { span, name, properties, children }
        })
}

fn properties_definition() -> impl Parser<TokenKind, ast::PropertiesDefinition<Span>, Error = ParserError> {
    let named_property_definition = || identifier()
        .then_ignore(punctuator(token::Punctuator::Colon))
        .then(ty())
        .then(
            punctuator(token::Punctuator::Equals)
                .ignore_then(value())
                .or_not()
        )
        .map_with_span(|((name, ty), default_value), span| {
            (ast::NamedPropertyDefinition { name, ty, default_value }, span)
        });

    let default_property_definition = keyword(token::Keyword::Default)
        .ignore_then(named_property_definition())
        .map_with_span(|(def, _), span| {
            ast::PropertyDefinitionKind::Default(def).spanned(span)
        });

    let text_property_definition = keyword(token::Keyword::Text)
        .ignore_then(identifier())
        .map_with_span(|name, span| {
            ast::PropertyDefinitionKind::Text(ast::TextPropertyDefinition { name }).spanned(span)
        });

    let property_definition = choice((
        default_property_definition,
        text_property_definition,
        named_property_definition()
            .map(|(def, span)| ast::PropertyDefinitionKind::Named(def).spanned(span))
    ));

    property_definition
        .separated_by(punctuator(token::Punctuator::Comma))
        .allow_trailing()
        .delimited_by(
            punctuator(token::Punctuator::LeftSquareBracket),
            punctuator(token::Punctuator::RightSquareBracket)
        )
        .map_with_span(|properties, span| {
            ast::PropertiesDefinition { span, properties }
        })
}

fn ty() -> impl Parser<TokenKind, ast::Type<Span>, Error = ParserError> {
    choice((
        just(TokenKind::Type(token::Type::String))
            .map_with_span(|_, span| ast::TypeKind::String.spanned(span)),
        just(TokenKind::Type(token::Type::Integer))
            .map_with_span(|_, span| ast::TypeKind::Integer.spanned(span)),
        just(TokenKind::Type(token::Type::Bool))
            .map_with_span(|_, span| ast::TypeKind::Bool.spanned(span)),
        just(TokenKind::Type(token::Type::Slot))
            .then_ignore(punctuator(token::Punctuator::LeftSquareBracket))
            .then_ignore(punctuator(token::Punctuator::RightSquareBracket))
            .map_with_span(|_, span| {
                ast::TypeKind::SlotList.spanned(span)
            }),
        just(TokenKind::Type(token::Type::Slot))
            .map_with_span(|_, span| ast::TypeKind::Slot.spanned(span))
    ))
}

fn component() -> impl Parser<TokenKind, ast::Component<Span>, Error = ParserError> {
    let component_name = choice((
        identifier(),
        punctuator(token::Punctuator::At)
            .map_with_span(|_, span| ast::Identifier { span, name: "@".to_owned() }),
        punctuator(token::Punctuator::Hash)
            .map_with_span(|_, span| ast::Identifier { span, name: "#".to_owned() })
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
            .map_with_span(|(((name, properties), text), children), span| {
                ast::Component { span, name, properties, children, text }
            })
    })
}

fn properties() -> impl Parser<TokenKind, ast::Properties<Span>, Error = ParserError> {
    let default_property = || value();
    let flag_property = || identifier()
        .map_with_span(|key, span| ast::PropertyKind::Flag { key }.spanned(span));
    let key_value_property = || identifier()
        .then_ignore(punctuator(token::Punctuator::Equals))
        .then(value())
        .map_with_span(|(key, value), span| {
            ast::PropertyKind::KeyValue { key, value }.spanned(span)
        });

    let named_property = || key_value_property().or(flag_property());
    let named_properties = || named_property()
        .separated_by(punctuator(token::Punctuator::Comma))
        .at_least(1)
        .allow_trailing();

    choice((
        named_properties().map(|properties| (None, properties)),
        default_property()
            .then_ignore(punctuator(token::Punctuator::Comma))
            .then(named_properties())
            .map(|(default, properties)| (Some(default), properties)),
        default_property().map(|default| (Some(default), Vec::new()))
    ))
    .delimited_by(
        punctuator(token::Punctuator::LeftSquareBracket),
        punctuator(token::Punctuator::RightSquareBracket)
    )
    .map_with_span(|(default, properties), span| {
        ast::Properties { span, default, properties }
    })
}

fn value() -> impl Parser<TokenKind, ast::Value<Span>, Error = ParserError> {
    choice((
        integer(),
        boolean(),
        string(),
        identifier()
            .map_with_span(|ident, span| ast::ValueKind::Variable(ident).spanned(span))
    ))
}

fn text() -> impl Parser<TokenKind, ast::Text<Span>, Error = ParserError> {
    let text_segment = variable_interpolation().or(text_literal());

    text_literal()
        .map(|segment| ast::Text { span: segment.span.clone(), segments: vec![segment] })
        .then(text_segment.repeated())
        .foldl(|mut acc, x| {
            match x.kind {
                ast::InterpolationSegmentKind::Literal(s) if s.is_empty() => {},
                _ => {
                    acc.span |= x.span.clone();
                    acc.segments.push(x)
                }
            };
            acc
        })
}

fn string() -> impl Parser<TokenKind, ast::Value<Span>, Error = ParserError> {
    let string_segment = variable_interpolation().or(string_literal());

    string_literal()
        .map(|segment| ast::StringValue { span: segment.span.clone(), segments: vec![segment] })
        .then(string_segment.repeated())
        .foldl(|mut acc, x| {
            match x.kind {
                ast::InterpolationSegmentKind::Literal(s) if s.is_empty() => {},
                _ => {
                    acc.span |= x.span.clone();
                    acc.segments.push(x);
                }
            };
            acc.into()
        })
        .map(|value| {
            let span = value.span.clone();

            ast::ValueKind::String(value).spanned(span)
        })
}

fn variable_interpolation() -> impl Parser<TokenKind, ast::InterpolationSegment<Span>, Error = ParserError> {
    punctuator(token::Punctuator::Dollar)
        .ignore_then(
            identifier()
                .delimited_by(
                    punctuator(token::Punctuator::LeftCurlyBracket),
                    punctuator(token::Punctuator::RightCurlyBracket)
                )
        )
        .map_with_span(|ident, span| {
            ast::InterpolationSegmentKind::Variable(ident).spanned(span)
        })
}

fn text_literal() -> impl Parser<TokenKind, ast::InterpolationSegment<Span>, Error = ParserError> {
    select! { |span|
        TokenKind::Text(token::Text { content, .. }) => ast::InterpolationSegmentKind::Literal(content).spanned(span)
    }
}

fn string_literal() -> impl Parser<TokenKind, ast::InterpolationSegment<Span>, Error = ParserError> {
    select! { |span|
        TokenKind::StringLiteral(token::StringLiteral { content, .. }) => ast::InterpolationSegmentKind::Literal(content).spanned(span)
    }
}

fn integer() -> impl Parser<TokenKind, ast::Value<Span>, Error = ParserError> {
    select! { |span|
        TokenKind::IntegerLiteral(token::IntegerLiteral(value)) => ast::ValueKind::Integer(value).spanned(span)
    }
}

fn boolean() -> impl Parser<TokenKind, ast::Value<Span>, Error = ParserError> {
    select! { |span|
        TokenKind::BooleanLiteral(token::BooleanLiteral(value)) => ast::ValueKind::Bool(value).spanned(span)
    }
}

fn identifier() -> impl Parser<TokenKind, ast::Identifier<Span>, Error = ParserError> {
    select! { |span|
        TokenKind::Identifier(token::Identifier(name)) => ast::Identifier {
            span,
            name
        }
    }
}

fn keyword(keyword: token::Keyword) -> Just<TokenKind, TokenKind, ParserError> {
    just(TokenKind::Keyword(keyword))
}

fn punctuator(punctuator: token::Punctuator) -> Just<TokenKind, TokenKind, ParserError> {
    just(TokenKind::Punctuator(punctuator))
}
