#[cfg(test)]
mod test {
    use anyhow::Result;
    use markerml_frontend::ast_span_helpers::MapSpan;
    use markerml_frontend::parser::Span;
    use markerml_frontend::{self, ast::*};

    fn parse(code: &str) -> Result<Module<Span>> {
        let module = markerml_frontend::parser::parse(&code)?;

        Ok(module)
    }

    fn parse_no_spans(code: &str) -> Result<Module<()>> {
        parse(&code).map(|module| module.map_span(&mut |_| ()))
    }

    #[test]
    fn whitespace() -> Result<()> {
        let code = r#"   box [ a,  b  ,  c] {   } "#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: Some(Properties {
                    default: None,
                    properties: vec![
                        PropertyKind::Flag {
                            key: Identifier::from_literal("a"),
                        }
                        .into(),
                        PropertyKind::Flag {
                            key: Identifier::from_literal("b"),
                        }
                        .into(),
                        PropertyKind::Flag {
                            key: Identifier::from_literal("c"),
                        }
                        .into(),
                    ],
                    span: (),
                }),
                children: Some(ComponentChildren {
                    children: vec![],
                    span: (),
                }),
                text: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn integer() -> Result<()> {
        let code = r#"box[a = 24, b = -143, c = 0]"#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: Some(Properties {
                    default: None,
                    properties: vec![
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("a"),
                            value: ValueKind::Integer(24).into(),
                        }
                        .into(),
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("b"),
                            value: ValueKind::Integer(-143).into(),
                        }
                        .into(),
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("c"),
                            value: ValueKind::Integer(0).into(),
                        }
                        .into(),
                    ],
                    span: (),
                }),
                children: None,
                text: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn boolean() -> Result<()> {
        let code = r#"box[a = true, b = false]"#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: Some(Properties {
                    default: None,
                    properties: vec![
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("a"),
                            value: ValueKind::Bool(true).into(),
                        }
                        .into(),
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("b"),
                            value: ValueKind::Bool(false).into(),
                        }
                        .into(),
                    ],
                    span: (),
                }),
                children: None,
                text: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn identifier() -> Result<()> {
        let code = r#"box_a _box __box box1_ box22"#;
        let res = Module {
            items: vec![
                Component {
                    name: Identifier::from_literal("box_a"),
                    properties: None,
                    children: None,
                    text: None,
                    span: (),
                }
                .into(),
                Component {
                    name: Identifier::from_literal("_box"),
                    properties: None,
                    children: None,
                    text: None,
                    span: (),
                }
                .into(),
                Component {
                    name: Identifier::from_literal("__box"),
                    properties: None,
                    children: None,
                    text: None,
                    span: (),
                }
                .into(),
                Component {
                    name: Identifier::from_literal("box1_"),
                    properties: None,
                    children: None,
                    text: None,
                    span: (),
                }
                .into(),
                Component {
                    name: Identifier::from_literal("box22"),
                    properties: None,
                    children: None,
                    text: None,
                    span: (),
                }
                .into(),
            ],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[should_panic]
    #[test]
    fn invalid_identifier() {
        let code = r#"1box"#;
        parse_no_spans(code).unwrap();
    }

    #[test]
    fn types() -> Result<()> {
        let code = r#"
            component custom[
                a: int,
                b: string = "abc",
                c: bool = false,
                d: slot,
                e: slot[]
            ]
        "#;
        let res = Module {
            items: vec![ComponentDefinition {
                name: Identifier::from_literal("custom"),
                properties: Some(PropertiesDefinition {
                    properties: vec![
                        PropertyDefinitionKind::Named(NamedPropertyDefinition {
                            name: Identifier::from_literal("a"),
                            ty: TypeKind::Integer.into(),
                            default_value: None,
                        })
                        .into(),
                        PropertyDefinitionKind::Named(NamedPropertyDefinition {
                            name: Identifier::from_literal("b"),
                            ty: TypeKind::String.into(),
                            default_value: Some(StringValue::from_literal("abc").into()),
                        })
                        .into(),
                        PropertyDefinitionKind::Named(NamedPropertyDefinition {
                            name: Identifier::from_literal("c"),
                            ty: TypeKind::Bool.into(),
                            default_value: Some(ValueKind::Bool(false).into()),
                        })
                        .into(),
                        PropertyDefinitionKind::Named(NamedPropertyDefinition {
                            name: Identifier::from_literal("d"),
                            ty: TypeKind::Slot.into(),
                            default_value: None,
                        })
                        .into(),
                        PropertyDefinitionKind::Named(NamedPropertyDefinition {
                            name: Identifier::from_literal("e"),
                            ty: TypeKind::SlotList.into(),
                            default_value: None,
                        })
                        .into(),
                    ],
                    span: (),
                }),
                children: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_simple() -> Result<()> {
        let code = r#"
            box
        "#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: None,
                children: None,
                text: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_empty_props() -> Result<()> {
        let code = r#"
            box[]
        "#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: Some(Properties {
                    default: None,
                    properties: vec![],
                    span: (),
                }),
                children: None,
                text: None,
                span: (),
            }
            .into()],
            span: (),
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
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: Some(Properties {
                    default: None,
                    properties: vec![PropertyKind::Flag {
                        key: Identifier::from_literal("vertical"),
                    }
                    .into()],
                    span: (),
                }),
                children: None,
                text: None,
                span: (),
            }
            .into()],
            span: (),
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
            span: (),
        };
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: None,
                children: Some(ComponentChildren {
                    children: vec![paragraph.clone(), paragraph.clone(), paragraph],
                    span: (),
                }),
                text: None,
                span: (),
            }
            .into()],
            span: (),
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
            items: vec![Component {
                name: Identifier::from_literal("paragraph"),
                properties: None,
                children: None,
                text: Some(Text::from_literal("Hello world!")),
                span: (),
            }
            .into()],
            span: (),
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
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: Some(Properties {
                    default: None,
                    properties: vec![
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("prop_a"),
                            value: ValueKind::String(StringValue::from_literal("hello")).into(),
                        }
                        .into(),
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("prop_b"),
                            value: ValueKind::String(StringValue::from_literal("world")).into(),
                        }
                        .into(),
                        PropertyKind::KeyValue {
                            key: Identifier::from_literal("prop_c"),
                            value: ValueKind::Bool(false).into(),
                        }
                        .into(),
                    ],
                    span: (),
                }),
                children: Some(ComponentChildren {
                    children: Vec::new(),
                    span: (),
                }),
                text: None,
                span: (),
            }
            .into()],
            span: (),
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
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: None,
                children: Some(ComponentChildren {
                    children: vec![
                        Component {
                            name: Identifier::from_literal("box"),
                            properties: None,
                            children: Some(ComponentChildren {
                                children: vec![Component {
                                    name: Identifier::from_literal("box"),
                                    properties: Some(Properties {
                                        default: None,
                                        properties: vec![PropertyKind::Flag {
                                            key: Identifier::from_literal("horizontal"),
                                        }
                                        .into()],
                                        span: (),
                                    }),
                                    children: Some(ComponentChildren {
                                        children: Vec::new(),
                                        span: (),
                                    }),
                                    text: None,
                                    span: (),
                                }],
                                span: (),
                            }),
                            text: None,
                            span: (),
                        },
                        Component {
                            name: Identifier::from_literal("box"),
                            properties: None,
                            children: Some(ComponentChildren {
                                children: Vec::new(),
                                span: (),
                            }),
                            text: None,
                            span: (),
                        },
                    ],
                    span: (),
                }),
                text: None,
                span: (),
            }
            .into()],
            span: (),
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
            items: vec![Component {
                name: Identifier::from_literal("#"),
                properties: Some(Properties {
                    default: Some(
                        ValueKind::String(StringValue::from_literal("google.com")).into(),
                    ),
                    properties: Vec::new(),
                    span: (),
                }),
                children: None,
                text: Some(Text::from_literal("google")),
                span: (),
            }
            .into()],
            span: (),
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
                        properties: vec![PropertyKind::Flag {
                            key: Identifier::from_literal("bold"),
                        }
                        .into()],
                        span: (),
                    }),
                    children: None,
                    text: Some(Text::from_literal("Hello, world!")),
                    span: (),
                }
                .into(),
                Component {
                    name: Identifier::from_literal("@"),
                    properties: None,
                    children: None,
                    text: Some(Text::from_literal(" wow ")),
                    span: (),
                }
                .into(),
            ],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn comment() -> Result<()> {
        let code = r#"
            box {
                // comment
                box { // another comment

                // }
                }
            }
        "#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: None,
                children: Some(ComponentChildren {
                    children: vec![Component {
                        name: Identifier::from_literal("box"),
                        properties: None,
                        children: Some(ComponentChildren {
                            children: vec![],
                            span: (),
                        }),
                        text: None,
                        span: (),
                    }],
                    span: (),
                }),
                text: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[should_panic]
    #[test]
    fn invalid_token() {
        parse_no_spans(
            r#"
            box ^ {

            }
        "#,
        )
        .unwrap();
    }

    #[should_panic]
    #[test]
    fn unclosed_string() {
        parse_no_spans(
            r#"
            image[url = "]
            @(Wow)
        "#,
        )
        .unwrap();
    }

    #[should_panic]
    #[test]
    fn unclosed_text() {
        parse_no_spans(
            r#"
            box {
                paragraph(Some long text
                box[] {

                }
            }
        "#,
        )
        .unwrap();
    }

    #[should_panic]
    #[test]
    fn unclosed_properties() {
        parse_no_spans(
            r#"
            box[ {
                box[] {

                }
            }
        "#,
        )
        .unwrap();
    }

    #[should_panic]
    #[test]
    fn unclosed_children() {
        parse_no_spans(
            r#"
            box {
                box {

            }
        "#,
        )
        .unwrap();
    }

    #[test]
    fn text_multiline() -> Result<()> {
        let code = r#"
            paragraph(
                text
                a
                b
                c
            )
        "#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("paragraph"),
                properties: None,
                children: None,
                text: Some(Text {
                    segments: vec![
                        InterpolationSegmentKind::Literal(" ".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal("text".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal(" ".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal("a".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal(" ".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal("b".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal(" ".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal("c".to_owned()).spanned(()),
                        InterpolationSegmentKind::Literal(" ".to_owned()).spanned(()),
                    ],
                    span: (),
                }),
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn string_interpolation() -> Result<()> {
        let code = r#"box[a = "Hello, ${variable}"]"#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("box"),
                properties: Some(Properties {
                    default: None,
                    properties: vec![PropertyKind::KeyValue {
                        key: Identifier::from_literal("a"),
                        value: StringValue {
                            segments: vec![
                                InterpolationSegmentKind::Literal("Hello, ".to_owned()).spanned(()),
                                InterpolationSegmentKind::Variable(Identifier::from_literal(
                                    "variable",
                                ))
                                .spanned(()),
                            ],
                            span: (),
                        }
                        .into(),
                    }
                    .into()],
                    span: (),
                }),
                children: None,
                text: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn text_interpolation() -> Result<()> {
        let code = r#"paragraph(Hello, ${variable}!)"#;
        let res = Module {
            items: vec![Component {
                name: Identifier::from_literal("paragraph"),
                properties: None,
                children: None,
                text: Some(Text {
                    segments: vec![
                        InterpolationSegmentKind::Literal("Hello, ".to_owned()).spanned(()),
                        InterpolationSegmentKind::Variable(Identifier::from_literal("variable"))
                            .spanned(()),
                        InterpolationSegmentKind::Literal("!".to_owned()).spanned(()),
                    ],
                    span: (),
                }),
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_definition() -> Result<()> {
        let code = r#"component custom {}"#;
        let res = Module {
            items: vec![ComponentDefinition {
                name: Identifier::from_literal("custom"),
                properties: None,
                children: Some(ComponentChildren {
                    children: vec![],
                    span: (),
                }),
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_definition_default_property() -> Result<()> {
        let code = r#"component custom[
            default prop: string
        ]"#;
        let res = Module {
            items: vec![ComponentDefinition {
                name: Identifier::from_literal("custom"),
                properties: Some(PropertiesDefinition {
                    properties: vec![PropertyDefinitionKind::Default(NamedPropertyDefinition {
                        name: Identifier::from_literal("prop"),
                        ty: TypeKind::String.into(),
                        default_value: None,
                    })
                    .into()],
                    span: (),
                }),
                children: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_definition_text_property() -> Result<()> {
        let code = r#"component custom[
            text prop
        ]"#;
        let res = Module {
            items: vec![ComponentDefinition {
                name: Identifier::from_literal("custom"),
                properties: Some(PropertiesDefinition {
                    properties: vec![PropertyDefinitionKind::Text(TextPropertyDefinition {
                        name: Identifier::from_literal("prop"),
                    })
                    .into()],
                    span: (),
                }),
                children: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }

    #[test]
    fn component_definition_named_property() -> Result<()> {
        let code = r#"component custom_component[
            something: int = 24,
            else: string
        ]"#;
        let res = Module {
            items: vec![ComponentDefinition {
                name: Identifier::from_literal("custom_component"),
                properties: Some(PropertiesDefinition {
                    properties: vec![
                        PropertyDefinitionKind::Named(NamedPropertyDefinition {
                            name: Identifier::from_literal("something"),
                            ty: TypeKind::Integer.into(),
                            default_value: Some(ValueKind::Integer(24).into()),
                        })
                        .into(),
                        PropertyDefinitionKind::Named(NamedPropertyDefinition {
                            name: Identifier::from_literal("else"),
                            ty: TypeKind::String.into(),
                            default_value: None,
                        })
                        .into(),
                    ],
                    span: (),
                }),
                children: None,
                span: (),
            }
            .into()],
            span: (),
        };

        assert_eq!(parse_no_spans(code)?, res);

        Ok(())
    }
}
