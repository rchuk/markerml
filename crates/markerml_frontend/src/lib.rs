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
pub fn parse(code: impl AsRef<str>) -> (Option<ast::Module>, Vec<parser::ParserError>) {
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

    fn parse(code: &str) -> Option<Module> {
        super::parse(&code).0
    }

    #[test]
    fn component_simple() {
        let code = r#"
            box
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::new("box"),
                    properties: None,
                    children: None,
                    text: None
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
    }

    #[test]
    fn component_with_flag_property() {
        let code = r#"
            box[vertical]
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::new("box"),
                    properties: Some(Properties {
                        default: None,
                        properties: vec![
                            Property::Flag { key: Identifier::new("vertical") }
                        ]
                    }),
                    children: None,
                    text: None
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
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
            name: Identifier::new("paragraph"),
            properties: None,
            children: None,
            text: None
        };
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::new("box"),
                    properties: None,
                    children: Some(vec![
                        paragraph.clone(),
                        paragraph.clone(),
                        paragraph
                    ]),
                    text: None
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
    }

    #[test]
    fn component_with_text() {
        let code = r#"
            paragraph(Hello world!)
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::new("paragraph"),
                    properties: None,
                    children: None,
                    text: Some(Text::from_literal("Hello world!"))
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
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
                    name: Identifier::new("box"),
                    properties: Some(Properties {
                        default: None,
                        properties: vec![
                            Property::KeyValue {
                                key: Identifier::new("prop_a"),
                                value: StringValue::from_literal("hello").into()
                            },
                            Property::KeyValue {
                                key: Identifier::new("prop_b"),
                                value: StringValue::from_literal("world").into()
                            },
                            Property::KeyValue {
                                key: Identifier::new("prop_c"),
                                value: Value::Bool(false)
                            }
                        ]
                    }),
                    children: Some(Vec::new()),
                    text: None
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
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
                    name: Identifier::new("box"),
                    properties: None,
                    children: Some(vec![
                        Component {
                            name: Identifier::new("box"),
                            properties: None,
                            children: Some(vec![
                                Component {
                                    name: Identifier::new("box"),
                                    properties: Some(Properties {
                                        default: None,
                                        properties: vec![
                                            Property::Flag { key: Identifier::new("horizontal") }
                                        ]
                                    }),
                                    children: Some(Vec::new()),
                                    text: None
                                }
                            ]),
                            text: None
                        },
                        Component {
                            name: Identifier::new("box"),
                            properties: None,
                            children: Some(Vec::new()),
                            text: None
                        },
                    ]),
                    text: None
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
    }

    #[test]
    fn builtin_link_component() {
        let code = r#"
            #["google.com"](google)
        "#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::new("#"),
                    properties: Some(Properties {
                        default: Some(StringValue::from_literal("google.com").into()),
                        properties: Vec::new()
                    }),
                    children: None,
                    text: Some(Text::from_literal("google"))
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
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
                    name: Identifier::new("@"),
                    properties: Some(Properties {
                        default: None,
                        properties: vec![
                            Property::Flag { key: Identifier::new("bold") }
                        ]
                    }),
                    children: None,
                    text: Some(Text::from_literal("Hello, world!"))
                }.into(),
                Component {
                    name: Identifier::new("@"),
                    properties: None,
                    children: None,
                    text: Some(Text::from_literal(" wow "))
                }.into()
            ]
        };

        assert_eq!(parse(code), Some(res));
    }
}

// TODO: Add more test cases
