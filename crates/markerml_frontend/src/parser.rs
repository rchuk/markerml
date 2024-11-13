use crate::ast::*;
use crate::span;
use pest::error::{Error, ErrorVariant};
use pest::{iterators::Pair, Parser, Position};
use pest_derive::Parser;

/// Source code span. Used for error reporting
pub type Span = span::Span;
/// Parser error
pub type ParserError = Error<Rule>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MarkermlParser;

type Result<T> = std::result::Result<T, Box<ParserError>>;

/// Parses given code into an AST
pub fn parse(code: &str) -> Result<Module<Span>> {
    let pairs = MarkermlParser::parse(Rule::module, code)?;
    let position = Position::from_start(pairs.as_str());
    let pair = pairs.into_iter().next().ok_or_else(|| {
        ParserError::new_from_pos(
            ErrorVariant::CustomError {
                message: "Missing module".to_owned(),
            },
            position,
        )
    })?;

    parse_module(pair)
}

fn parse_module(pair: Pair<Rule>) -> Result<Module<Span>> {
    let span = pair.as_span();
    let items = pair
        .into_inner()
        .map(|pair| {
            Ok(match pair.as_rule() {
                Rule::component => Some(ModuleItem::Component(parse_component(pair)?)),
                Rule::component_definition => Some(ModuleItem::ComponentDefinition(
                    parse_component_definition(pair)?,
                )),
                Rule::EOI => None,
                rule => return Err(create_error(format!("Unexpected {rule:?} in module"), span)),
            })
        })
        .filter_map(Result::transpose)
        .collect::<Result<Vec<_>>>()?;

    Ok(Module {
        span: span.into(),
        items,
    })
}

fn parse_component(pair: Pair<Rule>) -> Result<Component<Span>> {
    let span = pair.as_span();
    let mut name = None;
    let mut properties = None;
    let mut children = None;
    let mut text = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::component_name => {
                name = Some(parse_component_name(pair)?);
            }
            Rule::properties => {
                properties = Some(parse_properties(pair)?);
            }
            Rule::children => {
                children = Some(parse_children(pair)?);
            }
            Rule::text => {
                text = Some(parse_text(pair)?);
            }
            _ => {}
        }
    }

    Ok(Component {
        span: span.into(),
        name: name.ok_or_else(|| create_error("Missing component name".to_owned(), span))?,
        properties,
        children,
        text,
    })
}

fn parse_properties(pair: Pair<Rule>) -> Result<Properties<Span>> {
    let span = pair.as_span();
    let mut default = None;
    let mut properties = Vec::new();

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::default_property => {
                default = Some(parse_default_property(pair)?);
            }
            Rule::property => {
                properties.push(parse_property(pair)?);
            }
            rule => {
                return Err(create_error(
                    format!("Unexpected {rule:?} in properties"),
                    span,
                ))
            }
        }
    }

    Ok(Properties {
        span: span.into(),
        default,
        properties,
    })
}

fn parse_default_property(pair: Pair<Rule>) -> Result<Value<Span>> {
    let span = pair.as_span();
    let pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| create_error("Missing value in default property".to_owned(), span))?;

    parse_value(pair)
}

fn parse_property(pair: Pair<Rule>) -> Result<Property<Span>> {
    let span = pair.as_span();
    let pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| create_error("Missing named or flag property".to_owned(), span))?;

    let kind = match pair.as_rule() {
        Rule::named_property => parse_named_property(pair)?,
        Rule::flag_property => parse_flag_property(pair)?,
        rule => {
            return Err(create_error(
                format!("Unexpected {rule:?} in property"),
                span,
            ))
        }
    };

    Ok(kind.spanned(span.into()))
}

fn parse_named_property(pair: Pair<Rule>) -> Result<PropertyKind<Span>> {
    let span = pair.as_span();
    let mut key = None;
    let mut value = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                key = Some(parse_identifier(pair)?);
            }
            Rule::value => {
                value = Some(parse_value(pair)?);
            }
            _ => {}
        }
    }

    Ok(PropertyKind::KeyValue {
        key: key.ok_or_else(|| create_error("Missing key in named property".to_owned(), span))?,
        value: value
            .ok_or_else(|| create_error("Missing value in named property".to_owned(), span))?,
    })
}

fn parse_flag_property(pair: Pair<Rule>) -> Result<PropertyKind<Span>> {
    let span = pair.as_span();
    let pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| create_error("Missing identifier in flag property".to_owned(), span))?;

    match pair.as_rule() {
        Rule::identifier => Ok(PropertyKind::Flag {
            key: parse_identifier(pair)?,
        }),
        rule => Err(create_error(
            format!("Unexpected {rule:?} in flag property"),
            span,
        )),
    }
}

fn parse_component_name(pair: Pair<Rule>) -> Result<Identifier<Span>> {
    let span = pair.as_span();
    match pair.as_str() {
        name @ ("@" | "#") => Ok(Identifier {
            span: span.into(),
            name: name.to_owned(),
        }),
        _ => {
            let pair = pair.into_inner().next().ok_or_else(|| {
                create_error("Missing identifier in component name".to_owned(), span)
            })?;

            parse_identifier(pair)
        }
    }
}

fn parse_children(pair: Pair<Rule>) -> Result<ComponentChildren<Span>> {
    let span = pair.as_span();
    let children = pair
        .into_inner()
        .map(|pair| {
            Ok(match pair.as_rule() {
                Rule::component => Some(parse_component(pair)?),
                _ => None,
            })
        })
        .filter_map(Result::transpose)
        .collect::<Result<Vec<_>>>()?;

    Ok(ComponentChildren {
        span: span.into(),
        children,
    })
}

fn parse_component_definition(pair: Pair<Rule>) -> Result<ComponentDefinition<Span>> {
    let span = pair.as_span();
    let mut name = None;
    let mut properties = None;
    let mut children = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                name = Some(parse_identifier(pair)?);
            }
            Rule::properties_definition => {
                properties = Some(parse_properties_definition(pair)?);
            }
            Rule::children => {
                children = Some(parse_children(pair)?);
            }
            _ => {}
        }
    }

    Ok(ComponentDefinition {
        span: span.into(),
        name: name
            .ok_or_else(|| create_error("Missing name in component definition".to_owned(), span))?,
        properties,
        children,
    })
}

fn parse_properties_definition(pair: Pair<Rule>) -> Result<PropertiesDefinition<Span>> {
    let span = pair.as_span();
    let properties = pair
        .into_inner()
        .map(|pair| {
            Ok(match pair.as_rule() {
                Rule::property_definition => Some(parse_property_definition(pair)?),
                _ => None,
            })
        })
        .filter_map(Result::transpose)
        .collect::<Result<Vec<_>>>()?;

    Ok(PropertiesDefinition {
        span: span.into(),
        properties,
    })
}

fn parse_property_definition(pair: Pair<Rule>) -> Result<PropertyDefinition<Span>> {
    let span = pair.as_span();
    let pair = pair.into_inner().next().ok_or_else(|| {
        create_error(
            "Missing text, default or named property definition".to_owned(),
            span,
        )
    })?;

    let kind = match pair.as_rule() {
        Rule::text_property_definition => {
            let ident = pair.into_inner().next().ok_or_else(|| {
                create_error(
                    "Missing identifier in text property definition".to_owned(),
                    span,
                )
            })?;
            let name = parse_identifier(ident)?;
            PropertyDefinitionKind::Text(TextPropertyDefinition { name })
        }
        Rule::default_property_definition => {
            let property = parse_named_property_definition(pair)?;
            PropertyDefinitionKind::Default(property)
        }
        Rule::named_property_definition => {
            let property = parse_named_property_definition(pair)?;
            PropertyDefinitionKind::Named(property)
        }
        rule => {
            return Err(create_error(
                format!("Unexpected {rule:?} in property definition"),
                span,
            ))
        }
    };

    Ok(PropertyDefinition {
        span: span.into(),
        kind,
    })
}

fn parse_named_property_definition(pair: Pair<Rule>) -> Result<NamedPropertyDefinition<Span>> {
    let span = pair.as_span();
    let mut name = None;
    let mut ty = None;
    let mut default_value = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                name = Some(parse_identifier(pair)?);
            }
            Rule::ty => {
                ty = Some(parse_type(pair)?);
            }
            Rule::value => {
                default_value = Some(parse_value(pair)?);
            }
            _ => {}
        }
    }

    Ok(NamedPropertyDefinition {
        name: name.ok_or_else(|| {
            create_error("Missing name in named property definition".to_owned(), span)
        })?,
        ty: ty.ok_or_else(|| {
            create_error("Missing type in named property definition".to_owned(), span)
        })?,
        default_value,
    })
}

fn parse_value(pair: Pair<Rule>) -> Result<Value<Span>> {
    let span = pair.as_span();
    let pair = pair.into_inner().next().ok_or_else(|| {
        create_error(
            "Missing string, integer, boolean or identifier".to_owned(),
            span,
        )
    })?;

    let kind = match pair.as_rule() {
        Rule::string => {
            let string_value = parse_string(pair)?;
            ValueKind::String(string_value)
        }
        Rule::integer => {
            let int_value: i64 = pair.as_str().parse().unwrap();
            ValueKind::Integer(int_value)
        }
        Rule::bool => {
            let bool_value: bool = pair.as_str().parse().unwrap();
            ValueKind::Bool(bool_value)
        }
        Rule::identifier => {
            let identifier = parse_identifier(pair)?;
            ValueKind::Variable(identifier)
        }
        rule => return Err(create_error(format!("Unexpected {rule:?} in value"), span)),
    };

    Ok(kind.spanned(span.into()))
}

fn parse_string(pair: Pair<Rule>) -> Result<StringValue<Span>> {
    let span = pair.as_span();
    let segments = pair
        .into_inner()
        .map(parse_string_interpolation_segment)
        .collect::<Result<Vec<_>>>()?;

    Ok(StringValue {
        span: span.into(),
        segments,
    })
}

fn parse_text(pair: Pair<Rule>) -> Result<Text<Span>> {
    let span = pair.as_span();
    let segments = pair
        .into_inner()
        .map(parse_text_interpolation_segment)
        .collect::<Result<Vec<_>>>()?;

    Ok(Text {
        span: span.into(),
        segments,
    })
}

fn parse_string_interpolation_segment(pair: Pair<Rule>) -> Result<InterpolationSegment<Span>> {
    let span = pair.as_span();
    let pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| create_error("Missing segment in string".to_owned(), span))?;

    let kind = match pair.as_rule() {
        Rule::string_literal_segment => InterpolationSegmentKind::Literal(pair.as_str().to_owned()),
        Rule::variable_interpolation => {
            let ident = pair.into_inner().next().ok_or_else(|| {
                create_error(
                    "Missing identifier in string interpolation".to_owned(),
                    span,
                )
            })?;
            InterpolationSegmentKind::Variable(parse_identifier(ident)?)
        }
        Rule::literal_newline => InterpolationSegmentKind::Literal(" ".to_owned()),
        rule => {
            return Err(create_error(
                format!("Unexpected {rule:?} in string interpolation segment"),
                span,
            ))
        }
    };

    Ok(InterpolationSegment {
        span: span.into(),
        kind,
    })
}

fn parse_text_interpolation_segment(pair: Pair<Rule>) -> Result<InterpolationSegment<Span>> {
    let span = pair.as_span();
    let pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| create_error("Missing segment in text".to_owned(), span))?;

    let kind = match pair.as_rule() {
        Rule::text_literal_segment => InterpolationSegmentKind::Literal(pair.as_str().to_owned()),
        Rule::variable_interpolation => {
            let ident = pair.into_inner().next().ok_or_else(|| {
                create_error("Missing identifier in text interpolation".to_owned(), span)
            })?;

            InterpolationSegmentKind::Variable(parse_identifier(ident)?)
        }
        Rule::literal_newline => InterpolationSegmentKind::Literal(" ".to_owned()),
        rule => {
            return Err(create_error(
                format!("Unexpected {rule:?} in text interpolation segment"),
                span,
            ))
        }
    };

    Ok(InterpolationSegment {
        span: span.into(),
        kind,
    })
}

fn parse_type(pair: Pair<Rule>) -> Result<Type<Span>> {
    let span = pair.as_span();
    let kind = match pair.as_str() {
        "string" => TypeKind::String,
        "int" => TypeKind::Integer,
        "bool" => TypeKind::Bool,
        "slot" => TypeKind::Slot,
        "slot[]" => TypeKind::SlotList,
        value => return Err(create_error(format!("Unexpected `{value}` in type"), span)),
    };

    Ok(Type {
        span: span.into(),
        kind,
    })
}

fn parse_identifier(pair: Pair<Rule>) -> Result<Identifier<Span>> {
    let span = pair.as_span();
    match pair.as_rule() {
        Rule::identifier => Ok(Identifier {
            span: span.into(),
            name: pair.as_str().to_owned(),
        }),
        rule => Err(create_error(
            format!("Unexpected {rule:?} in identifier"),
            span,
        )),
    }
}

fn create_error(message: String, span: pest::Span) -> Box<ParserError> {
    Box::new(ParserError::new_from_span(
        ErrorVariant::CustomError { message },
        span,
    ))
}
