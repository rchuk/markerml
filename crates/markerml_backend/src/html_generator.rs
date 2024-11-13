use crate::error::*;
use build_html::{Html, HtmlChild, HtmlContainer, HtmlElement, HtmlPage, HtmlTag};
use itertools::{Either, Itertools};
use markerml_middleend::{ir, Span};
use std::collections::HashSet;

/// HTML string generator
pub struct HtmlGenerator {
    ir: Option<ir::Module<Span>>,
    definitions: HashSet<ir::ComponentDefinition<Span>>,
}

impl HtmlGenerator {
    /// Creates new instance from the given IR
    pub fn new(ir: ir::Module<Span>) -> Self {
        HtmlGenerator {
            ir: Some(ir),
            definitions: HashSet::new(),
        }
    }

    /// Generates HTML from the stored IR
    pub fn generate(mut self) -> Result<String, BackendError> {
        let module = self.ir.take().unwrap();
        let root = self.emit_module(module)?;

        Ok(root.to_html_string())
    }

    fn emit_module(&mut self, module: ir::Module<Span>) -> Result<HtmlPage, BackendError> {
        let mut items = Vec::new();

        let (components, definitions): (Vec<_>, HashSet<_>) =
            module.items.into_iter().partition_map(|item| match item {
                ir::ModuleItem::Component(component) => Either::Left(component),
                ir::ModuleItem::ComponentDefinition(def) => Either::Right(def),
            });
        self.definitions = definitions;

        for component in components {
            items.push(self.emit_component(&component, None)?);
        }

        let mut page = HtmlPage::new();
        let mut main = HtmlElement::new(HtmlTag::Main);
        main.children = items;
        page.add_raw(main.to_html_string());

        Ok(page)
    }

    fn emit_component(
        &self,
        component: &ir::Component<Span>,
        ctx: Option<&ir::Component<Span>>,
    ) -> Result<HtmlChild, BackendError> {
        if let Some(component) = self.try_emit_builtin_component(component, ctx)? {
            Ok(component)
        } else {
            Err(BackendError::Unimplemented)
        }
    }

    fn try_emit_builtin_component(
        &self,
        component: &ir::Component<Span>,
        ctx: Option<&ir::Component<Span>>,
    ) -> Result<Option<HtmlChild>, BackendError> {
        Ok(Some(match component.name.as_str() {
            "box" => {
                let is_vertical = match (
                    Self::get_flag_property(component, "vertical"),
                    Self::get_flag_property(component, "horizontal"),
                ) {
                    (true, true) => return Err(BackendError::Todo), // TODO
                    (true, false) | (false, false) => true,
                    (false, true) => false,
                };
                let flex_direction = if is_vertical { "column" } else { "row" };
                let x_align = Self::try_get_named_property(component, "x_align")
                    .map(Self::cast_to_string)
                    .transpose()?;
                let y_align = Self::try_get_named_property(component, "y_align")
                    .map(Self::cast_to_string)
                    .transpose()?;
                x_align
                    .as_ref()
                    .map(|value| Self::check_align_allowed(value))
                    .transpose()?;
                y_align
                    .as_ref()
                    .map(|value| Self::check_align_allowed(value))
                    .transpose()?;

                let justify_content = if is_vertical { &y_align } else { &x_align };
                let align_items = if is_vertical { &x_align } else { &y_align };

                let mut style = format!("display: flex; flex-direction: {flex_direction}");
                if let Some(justify_content) = justify_content {
                    style.push_str(&format!("; justify-content: {justify_content}"));
                }
                if let Some(align_items) = align_items {
                    style.push_str(&format!("; align-items: {align_items}"));
                }

                let mut element = HtmlElement::new(HtmlTag::Div).with_attribute("style", style);

                element.children = component
                    .children
                    .iter()
                    .map(|child| self.emit_component(child, ctx))
                    .collect::<Result<_, _>>()?;

                HtmlChild::Element(element)
            }
            "@" => {
                let text = Self::get_text(component)?;

                HtmlChild::Raw(format!("<span>{text}</span>"))
            }
            "#" => {
                let href =
                    Self::cast_to_string(Self::get_default_or_named_property(component, "url")?)?;
                let text = Self::get_text(component)?;

                HtmlChild::Element(
                    HtmlElement::new(HtmlTag::Link)
                        .with_attribute("href", href)
                        .with_child(text.into()),
                )
            }
            "paragraph" => {
                let text = Self::get_text(component)?;

                HtmlChild::Element(HtmlElement::new(HtmlTag::ParagraphText).with_child(text.into()))
            }
            "header" => {
                let text = Self::get_text(component)?;
                let level = Self::try_get_default_or_named_property(component, "level")
                    .map(Self::cast_to_int)
                    .transpose()?
                    .unwrap_or(1);

                let tag = match level {
                    1 => HtmlTag::Heading1,
                    2 => HtmlTag::Heading2,
                    3 => HtmlTag::Heading3,
                    4 => HtmlTag::Heading4,
                    5 => HtmlTag::Heading5,
                    6 => HtmlTag::Heading6,
                    _ => return Err(BackendError::Todo), // TODO
                };

                HtmlChild::Element(HtmlElement::new(tag).with_child(text.into()))
            }
            "image" => {
                let src =
                    Self::cast_to_string(Self::get_default_or_named_property(component, "src")?)?;

                HtmlChild::Element(HtmlElement::new(HtmlTag::Image).with_attribute("src", src))
            }
            "list" => {
                let is_unordered = match (
                    Self::get_flag_property(component, "unordered"),
                    Self::get_flag_property(component, "ordered"),
                ) {
                    (true, true) => return Err(BackendError::Todo), // TODO
                    (true, false) | (false, false) => true,
                    (false, true) => false,
                };
                let tag = if is_unordered {
                    HtmlTag::UnorderedList
                } else {
                    HtmlTag::OrderedList
                };

                let mut element = HtmlElement::new(tag);
                element.children = component
                    .children
                    .iter()
                    .map(|child| {
                        Ok::<HtmlChild, BackendError>(
                            HtmlElement::new(HtmlTag::ListElement)
                                .with_child(self.emit_component(child, ctx)?)
                                .into(),
                        )
                    })
                    .collect::<Result<_, _>>()?;

                HtmlChild::Element(element)
            }
            _ => return Ok(None),
        }))
    }

    fn cast_to_string(value: ir::Value<Span>) -> Result<String, BackendError> {
        match value.kind {
            ir::ValueKind::String(string_value) => Self::build_string(string_value),
            kind => Err(TypeMismatchError {
                span: value.span,
                expected: "string",
                got: Self::get_value_kind_name(kind),
            }
            .into()),
        }
    }

    fn cast_to_int(value: ir::Value<Span>) -> Result<i64, BackendError> {
        match value.kind {
            ir::ValueKind::Integer(value) => Ok(value),
            kind => Err(TypeMismatchError {
                span: value.span,
                expected: "int",
                got: Self::get_value_kind_name(kind),
            }
            .into()),
        }
    }

    fn build_string(string: ir::StringValue<Span>) -> Result<String, BackendError> {
        Self::interpolate_string(string.segments)
    }

    fn build_text(text: ir::Text<Span>) -> Result<String, BackendError> {
        Self::interpolate_string(text.segments)
    }

    // TODO: Pass context
    fn interpolate_string(
        segments: Vec<ir::InterpolationSegment<Span>>,
    ) -> Result<String, BackendError> {
        Ok(segments
            .into_iter()
            .flat_map(|segment| match segment.kind {
                ir::InterpolationSegmentKind::Literal(string) => Some(string),
                ir::InterpolationSegmentKind::Variable(_) => None,
            })
            .join(""))
    }

    fn get_default_or_named_property(
        component: &ir::Component<Span>,
        name: &str,
    ) -> Result<ir::Value<Span>, BackendError> {
        Self::try_get_default_or_named_property(component, name).ok_or_else(|| {
            RequiredDefaultPropertyMissingError {
                span: component.span.clone(),
                name: name.to_owned(),
            }
            .into()
        })
    }

    fn try_get_default_or_named_property(
        component: &ir::Component<Span>,
        name: &str,
    ) -> Option<ir::Value<Span>> {
        component
            .properties
            .default
            .clone()
            .or_else(|| Self::try_get_named_property(component, name))
    }

    fn try_get_named_property(
        component: &ir::Component<Span>,
        name: &str,
    ) -> Option<ir::Value<Span>> {
        component
            .properties
            .named_properties
            .get(name)
            .map(|prop| prop.value.clone())
    }

    fn get_flag_property(component: &ir::Component<Span>, name: &str) -> bool {
        component.properties.flag_properties.contains(name)
    }

    fn get_text(component: &ir::Component<Span>) -> Result<String, BackendError> {
        let text = component.text.clone().ok_or_else(|| TextMissingError {
            span: component.span.clone(),
        })?;

        Self::build_text(text)
    }

    fn get_value_kind_name(kind: ir::ValueKind<Span>) -> &'static str {
        match kind {
            ir::ValueKind::String(_) => "string",
            ir::ValueKind::Integer(_) => "int",
            ir::ValueKind::Variable(_) => "variable",
            ir::ValueKind::Bool(_) => "bool",
        }
    }

    fn check_align_allowed(align: &str) -> Result<(), BackendError> {
        match align {
            "start" | "center" | "end" => Ok(()),
            _ => Err(BackendError::Todo),
        }
    }
}
