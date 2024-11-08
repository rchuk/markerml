pub mod parse_error;
pub mod validator;
pub mod parser;
pub mod ast;
pub mod lexer;
pub mod token;
pub mod common;

/// Performs complete parsing of the source code
///
/// # Example
/// ```
/// let code = r#"
///       page[
///         title = "Sample page"
///       ] {
///         header[1](Hello World)
///         paragraph(
///             Content
///         )
///       }
///     "#;
///
///     let module = markerml_frontend::parse(&code)?;
///     println!("Module: {:#?}", module);
/// ```
pub fn parse(code: impl AsRef<str>) -> Result<ast::Module<common::span::Span>, parse_error::ParseError> {
    use std::sync::Arc;

    // TODO: Properly manage code
    let code = Arc::from(code.as_ref());

    let (tokens, eof) = lexer::Lexer::new(&code).lex();
    let ast = parser::parse(&code, tokens, eof)?;
    let validator = validator::Validator::new(code.clone(), &ast);
    validator.validate()?;

    Ok(ast)
}

#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::common::span::{Position, Span};
    use anyhow::Result;

    fn parse(code: &str) -> Result<Module<Span>> {
        let module = super::parse(&code)?;

        Ok(module)
    }

    fn parse_no_spans(code: &str) -> Result<Module<()>> {
        parse(&code)
            .map(|module| module.map_span(|_| ()))
    }

    #[test]
    fn component_simple() -> Result<()> {
        let code = r#"
            box
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("box"),
                    properties: None,
                    children: None,
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_with_flag_property() -> Result<()> {
        let code = r#"
            box[vertical]
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("box"),
                    properties: Some(Properties {
                        default: None,
                        properties: vec![
                            PropertyKind::Flag {
                                key: Identifier::from_literal("vertical")
                            }.into()
                        ],
                        span: Default::default()
                    }),
                    children: None,
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_with_children() -> Result<()> {
        let code = r#"
            box {
                paragraph
                paragraph
                paragraph
            }
        "#;
        let paragraph = Component {
            name: Identifier::from_literal("paragraph"),
            properties: None,
            children: None,
            text: None,
            span: Default::default()
        };
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("box"),
                    properties: None,
                    children: Some(ComponentChildren {
                        children: vec![
                            paragraph.clone(),
                            paragraph.clone(),
                            paragraph
                        ],
                        span: Default::default()
                    }),
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_with_text() -> Result<()> {
        let code = r#"
            paragraph(Hello world!)
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("paragraph"),
                    properties: None,
                    children: None,
                    text: Some(Text::from_literal("Hello world!")),
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_with_named_properties() -> Result<()> {
        let code = r#"
            box[prop_a = "hello", prop_b="world", prop_c=false] {

            }
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("box"),
                    properties: Some(Properties {
                        default: None,
                        properties: vec![
                            PropertyKind::KeyValue {
                                key: Identifier::from_literal("prop_a"),
                                value: ValueKind::String(StringValue::from_literal("hello")).into()
                            }.into(),
                            PropertyKind::KeyValue {
                                key: Identifier::from_literal("prop_b"),
                                value: ValueKind::String(StringValue::from_literal("world")).into()
                            }.into(),
                            PropertyKind::KeyValue {
                                key: Identifier::from_literal("prop_c"),
                                value: ValueKind::Bool(false).into(),
                            }.into()
                        ],
                        span: Default::default()
                    }),
                    children: Some(ComponentChildren {
                        children: Vec::new(),
                        span: Default::default()
                    }),
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_with_nested_children() -> Result<()> {
        let code = r#"
            box {
                box {
                    box[horizontal] {

                    }
                }

                box {

                }
            }
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("box"),
                    properties: None,
                    children: Some(ComponentChildren {
                        children: vec![
                            Component {
                                name: Identifier::from_literal("box"),
                                properties: None,
                                children: Some(ComponentChildren {
                                    children: vec![
                                        Component {
                                            name: Identifier::from_literal("box"),
                                            properties: Some(Properties {
                                                default: None,
                                                properties: vec![
                                                    PropertyKind::Flag {
                                                        key: Identifier::from_literal("horizontal")
                                                    }.into()
                                                ],
                                                span: Default::default()
                                            }),
                                            children: Some(ComponentChildren {
                                                children: Vec::new(),
                                                span: Default::default()
                                            }),
                                            text: None,
                                            span: Default::default()
                                        }
                                    ],
                                    span: Default::default()
                                }),
                                text: None,
                                span: Default::default()
                            },
                            Component {
                                name: Identifier::from_literal("box"),
                                properties: None,
                                children: Some(ComponentChildren {
                                    children: Vec::new(),
                                    span: Default::default()
                                }),
                                text: None,
                                span: Default::default()
                            },
                        ],
                        span: Default::default()
                    }),
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn builtin_link_component() -> Result<()> {
        let code = r#"
            #["google.com"](google)
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("#"),
                    properties: Some(Properties {
                        default: Some(ValueKind::String(StringValue::from_literal("google.com")).into()),
                        properties: Vec::new(),
                        span: Default::default()
                    }),
                    children: None,
                    text: Some(Text::from_literal("google")),
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn builtin_text_component() -> Result<()> {
        let code = r#"
            @[bold](Hello, world!)
            @( wow )
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("@"),
                    properties: Some(Properties {
                        default: None,
                        properties: vec![
                            PropertyKind::Flag {
                                key: Identifier::from_literal("bold")
                            }.into()
                        ],
                        span: Default::default()
                    }),
                    children: None,
                    text: Some(Text::from_literal("Hello, world!")),
                    span: Default::default()
                }.into(),
                Component {
                    name: Identifier::from_literal("@"),
                    properties: None,
                    children: None,
                    text: Some(Text::from_literal(" wow ")),
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn span_component_simple() -> Result<()> {
        let code = r#"
box[vertical] {

}
        "#;
        let res = Module {
            span: Span {
                start: Position { line: 2, column: 1 },
                end: Position { line: 4, column: 2 }
            },
            items: vec![
                Component {
                    span: Span {
                        start: Position { line: 2, column: 1 },
                        end: Position { line: 4, column: 2 }
                    },
                    name: Identifier {
                        span: Span {
                            start: Position { line: 2, column: 1 },
                            end: Position { line: 2, column: 4 },
                        },
                        name: "box".to_owned()
                    },
                    properties: Some(Properties {
                        span: Span {
                            start: Position { line: 2, column: 4 },
                            end: Position { line: 2, column: 14 }
                        },
                        default: None,
                        properties: vec![
                            PropertyKind::Flag {
                                key: Identifier {
                                    span: Span {
                                        start: Position { line: 2, column: 5 },
                                        end: Position { line: 2, column: 13 }
                                    },
                                    name: "vertical".to_owned()
                                }
                            }.spanned(Span {
                                start: Position { line: 2, column: 5 },
                                end: Position { line: 2, column: 13 }
                            })
                        ]
                    }),
                    children: Some(ComponentChildren {
                        children: Vec::new(),
                        span: Span {
                            start: Position { line: 2, column: 15 },
                            end: Position { line: 4, column: 2 }
                        }
                    }),
                    text: None
                }.into()
            ]
        };

        assert_eq!(parse(code)?, res);

        Ok(())
    }
}

// TODO: Add more test cases
