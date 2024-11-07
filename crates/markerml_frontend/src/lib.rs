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
///     let (module, errors) = markerml_frontend::parse(&code);
///     println!("Module: {:#?}", module);
///     for err in errors {
///         println!("Parse error: {}. At line {} column {}", err, err.span().start.line, err.span().start.column);
///     }
/// ```
pub fn parse(code: impl AsRef<str>) -> (Option<ast::Module<common::span::Span>>, Vec<parser::ParserError>) {
    use chumsky::{Stream, Parser};

    let (tokens, eof) = lexer::Lexer::new(&code).lex();
    let parser = parser::parser();
    let stream = Stream::from_iter(eof, tokens.into_iter());
    parser.parse_recovery(stream)

    // TODO: Check some language invariants in additional pass
    //  for example: single text property, single default property, no children in text component, etc.
    //
    // TODO: Implement better error handling. Use miette crate
}

#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::common::span::{Position, Span};

    fn parse(code: &str) -> Option<Module<Span>> {
        super::parse(&code).0
    }

    fn parse_no_spans(code: &str) -> Option<Module<()>> {
        parse(&code)
            .map(|module| module.map_span(|_| ()))
    }

    #[test]
    fn component_simple() {
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

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn component_with_flag_property() {
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

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn component_with_children() {
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
                    children: Some(vec![
                        paragraph.clone(),
                        paragraph.clone(),
                        paragraph
                    ]),
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn component_with_text() {
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

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn component_with_named_properties() {
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
                    children: Some(Vec::new()),
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn component_with_nested_children() {
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
                    children: Some(vec![
                        Component {
                            name: Identifier::from_literal("box"),
                            properties: None,
                            children: Some(vec![
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
                                    children: Some(Vec::new()),
                                    text: None,
                                    span: Default::default()
                                }
                            ]),
                            text: None,
                            span: Default::default()
                        },
                        Component {
                            name: Identifier::from_literal("box"),
                            properties: None,
                            children: Some(Vec::new()),
                            text: None,
                            span: Default::default()
                        },
                    ]),
                    text: None,
                    span: Default::default()
                }.into()
            ],
            span: Default::default()
        };

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn builtin_link_component() {
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

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn builtin_text_component() {
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

        assert_eq!(parse_no_spans(code), Some(res));
    }

    #[test]
    fn span_component_simple() {
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
                    children: Some(Vec::new()),
                    text: None
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res))
    }
}

// TODO: Add more test cases
