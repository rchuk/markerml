use crate::error::*;
use crate::{ir, IrGeneratorError};
use markerml_frontend::ast;
use markerml_frontend::parser::Span;
use std::collections::{HashMap, HashSet};

/// Intermediate Representation generator
pub struct IrGenerator {
    ast: Option<ast::Module<Span>>,
}

impl IrGenerator {
    /// Creates new instance from the given AST
    pub fn new(ast: ast::Module<Span>) -> Self {
        IrGenerator { ast: Some(ast) }
    }

    /// Generates Intermediate Representation from the stored AST
    pub fn generate(mut self) -> Result<ir::Module<Span>, IrGeneratorError> {
        let ast = self.ast.take().unwrap();

        self.generate_module(ast)
    }

    fn generate_module(
        &mut self,
        module: ast::Module<Span>,
    ) -> Result<ir::Module<Span>, IrGeneratorError> {
        Ok(ir::Module {
            span: module.span,
            items: module
                .items
                .into_iter()
                .map(|item| self.generate_module_item(item))
                .collect::<Result<_, _>>()?,
        })
    }

    fn generate_module_item(
        &mut self,
        item: ast::ModuleItem<Span>,
    ) -> Result<ir::ModuleItem<Span>, IrGeneratorError> {
        Ok(match item {
            ast::ModuleItem::Component(component) => {
                ir::ModuleItem::Component(self.generate_component(component)?)
            }
            ast::ModuleItem::ComponentDefinition(def) => {
                ir::ModuleItem::ComponentDefinition(self.generate_component_definition(def)?)
            }
        })
    }

    fn generate_component(
        &mut self,
        component: ast::Component<Span>,
    ) -> Result<ir::Component<Span>, IrGeneratorError> {
        let name_span = component.name.span.clone();
        let name = self.generate_identifier(component.name)?;
        let properties = component
            .properties
            .map(|properties| self.generate_properties(properties))
            .unwrap_or_else(|| {
                Ok(ir::Properties {
                    default: None,
                    named_properties: HashSet::new(),
                    flag_properties: HashSet::new(),
                })
            })?;

        if let (Some(children), Some(text)) = (&component.children, &component.text) {
            return Err(TextComponentWithChildrenError {
                component_name: name_span,
                children: children.span.clone(),
                text: text.span.clone(),
            }
            .into());
        }

        let children = component
            .children
            .map(|children| self.generate_children(children))
            .unwrap_or_else(|| Ok(Vec::new()))?;
        let text = component
            .text
            .map(|text| self.generate_text(text))
            .transpose()?;

        Ok(ir::Component {
            span: component.span,
            name,
            properties,
            children,
            text,
        })
    }

    fn generate_properties(
        &mut self,
        properties: ast::Properties<Span>,
    ) -> Result<ir::Properties<Span>, IrGeneratorError> {
        let default = properties
            .default
            .map(|value| self.generate_value(value))
            .transpose()?;
        let mut names: HashMap<String, Span> = HashMap::new();
        let mut named_properties = HashSet::new();
        let mut flag_properties = HashSet::new();

        for property in properties.properties {
            match property.kind {
                ast::PropertyKind::KeyValue { key, value } => {
                    let key = self.generate_identifier(key)?;
                    if let Some(span) = names.get(key.as_str()) {
                        return Err(DuplicatedPropertyError {
                            name: key.clone().into(),
                            first: span.clone(),
                            second: key.span.clone(),
                        }
                        .into());
                    }

                    names.insert(key.as_str().to_owned(), key.span.clone());
                    named_properties.insert(ir::Property {
                        span: property.span.clone(),
                        key,
                        value: self.generate_value(value)?,
                    });
                }
                ast::PropertyKind::Flag { key } => {
                    let key = self.generate_identifier(key)?;
                    if let Some(span) = names.get(key.as_str()) {
                        return Err(DuplicatedPropertyError {
                            name: key.clone().into(),
                            first: span.clone(),
                            second: key.span.clone(),
                        }
                        .into());
                    }

                    names.insert(key.as_str().to_owned(), key.span.clone());
                    flag_properties.insert(key);
                }
            }
        }

        Ok(ir::Properties {
            default,
            named_properties,
            flag_properties,
        })
    }

    fn generate_component_definition(
        &mut self,
        def: ast::ComponentDefinition<Span>,
    ) -> Result<ir::ComponentDefinition<Span>, IrGeneratorError> {
        let name = self.generate_identifier(def.name.clone())?;
        let children = def
            .children
            .map(|children| self.generate_children(children))
            .transpose()?
            .unwrap_or_else(Vec::new);

        if let Some(child) = children
            .iter()
            .find(|child| child.name.as_str() == name.as_str())
        {
            return Err(CircularDefinitionError {
                component_name: name.span,
                circular: child.span.clone(),
            }
            .into());
        }

        Ok(ir::ComponentDefinition {
            span: def.span.clone(),
            name: self.generate_identifier(def.name.clone())?,
            properties: def
                .properties
                .map(|props| self.generate_properties_definition(props))
                .transpose()?
                .unwrap_or_else(|| ir::PropertiesDefinition {
                    span: def.name.span,
                    text_property: None,
                    default_property: None,
                    properties: HashSet::new(),
                }),
            children,
        })
    }

    fn generate_properties_definition(
        &mut self,
        def: ast::PropertiesDefinition<Span>,
    ) -> Result<ir::PropertiesDefinition<Span>, IrGeneratorError> {
        let mut default_property: Option<ir::PropertyDefinition<Span>> = None;
        let mut text_property: Option<ir::Identifier<Span>> = None;
        let mut properties = HashSet::new();
        let mut names = HashMap::<String, Span>::new();

        for property in def.properties {
            match property.kind {
                ast::PropertyDefinitionKind::Default(def) => {
                    if let Some(prop_def) = default_property {
                        return Err(MultipleDefaultPropertiesError {
                            component_name: def.name.span,
                            first: prop_def.span,
                            second: property.span,
                        }
                        .into());
                    }
                    if let Some(span) = names.get(def.name.as_str()) {
                        return Err(DuplicatedPropertyError {
                            name: def.name.into(),
                            first: span.clone(),
                            second: property.span,
                        }
                        .into());
                    }
                    if let Some(default) = def.default_value {
                        return Err(DefaultPropertyWithValueError {
                            component_name: def.name.span,
                            property: property.span,
                            default_value: default.span,
                        }
                        .into());
                    }

                    names.insert(def.name.clone().into(), property.span.clone());
                    let def = ir::PropertyDefinition {
                        span: property.span.clone(),
                        name: self.generate_identifier(def.name)?,
                        ty: self.generate_type(def.ty)?,
                        default_value: None,
                    };
                    default_property = Some(def.clone());
                    properties.insert(def);
                }
                ast::PropertyDefinitionKind::Text(def) => {
                    if let Some(prop_def) = text_property {
                        return Err(MultipleTextPropertiesError {
                            component_name: def.name.span,
                            first: prop_def.span,
                            second: property.span,
                        }
                        .into());
                    }
                    if let Some(span) = names.get(def.name.as_str()) {
                        return Err(DuplicatedPropertyError {
                            name: def.name.into(),
                            first: span.clone(),
                            second: property.span,
                        }
                        .into());
                    }

                    names.insert(def.name.clone().into(), property.span.clone());
                    text_property = Some(self.generate_identifier(def.name)?);
                }
                ast::PropertyDefinitionKind::Named(def) => {
                    if let Some(span) = names.get(def.name.as_str()) {
                        return Err(DuplicatedPropertyError {
                            name: def.name.into(),
                            first: span.clone(),
                            second: property.span,
                        }
                        .into());
                    }

                    names.insert(def.name.clone().into(), property.span.clone());
                    properties.insert(ir::PropertyDefinition {
                        span: property.span.clone(),
                        name: self.generate_identifier(def.name)?,
                        ty: self.generate_type(def.ty)?,
                        default_value: def
                            .default_value
                            .map(|value| self.generate_value(value))
                            .transpose()?,
                    });
                }
            }
        }

        Ok(ir::PropertiesDefinition {
            span: def.span,
            default_property,
            text_property,
            properties,
        })
    }

    fn generate_children(
        &mut self,
        children: ast::ComponentChildren<Span>,
    ) -> Result<Vec<ir::Component<Span>>, IrGeneratorError> {
        children
            .children
            .into_iter()
            .map(|component| self.generate_component(component))
            .collect::<Result<_, _>>()
    }

    fn generate_value(
        &mut self,
        value: ast::Value<Span>,
    ) -> Result<ir::Value<Span>, IrGeneratorError> {
        let kind = match value.kind {
            ast::ValueKind::String(value) => {
                ir::ValueKind::String(self.generate_string_value(value)?)
            }
            ast::ValueKind::Variable(identifier) => {
                ir::ValueKind::Variable(self.generate_identifier(identifier)?)
            }
            ast::ValueKind::Integer(value) => ir::ValueKind::Integer(value),
            ast::ValueKind::Bool(value) => ir::ValueKind::Bool(value),
        };

        Ok(kind.spanned(value.span))
    }

    fn generate_text(&mut self, text: ast::Text<Span>) -> Result<ir::Text<Span>, IrGeneratorError> {
        let span = text.span;
        let segments = text
            .segments
            .into_iter()
            .map(|segment| self.generate_interpolation_segment(segment))
            .collect::<Result<_, _>>()?;

        Ok(ir::Text { span, segments })
    }

    fn generate_string_value(
        &mut self,
        value: ast::StringValue<Span>,
    ) -> Result<ir::StringValue<Span>, IrGeneratorError> {
        let span = value.span;
        let segments = value
            .segments
            .into_iter()
            .map(|segment| self.generate_interpolation_segment(segment))
            .collect::<Result<_, _>>()?;

        Ok(ir::StringValue { span, segments })
    }

    fn generate_interpolation_segment(
        &mut self,
        segment: ast::InterpolationSegment<Span>,
    ) -> Result<ir::InterpolationSegment<Span>, IrGeneratorError> {
        let kind = match segment.kind {
            ast::InterpolationSegmentKind::Literal(literal) => {
                ir::InterpolationSegmentKind::Literal(literal)
            }
            ast::InterpolationSegmentKind::Variable(identifier) => {
                ir::InterpolationSegmentKind::Variable(self.generate_identifier(identifier)?)
            }
        };

        Ok(kind.spanned(segment.span))
    }

    fn generate_type(&mut self, ty: ast::Type<Span>) -> Result<ir::Type<Span>, IrGeneratorError> {
        let kind = match ty.kind {
            ast::TypeKind::String => ir::TypeKind::String,
            ast::TypeKind::Integer => ir::TypeKind::Integer,
            ast::TypeKind::Bool => ir::TypeKind::Bool,
            ast::TypeKind::Slot => ir::TypeKind::Slot,
            ast::TypeKind::SlotList => ir::TypeKind::SlotList,
        };

        Ok(kind.spanned(ty.span))
    }

    fn generate_identifier(
        &mut self,
        identifier: ast::Identifier<Span>,
    ) -> Result<ir::Identifier<Span>, IrGeneratorError> {
        Ok(ir::Identifier {
            span: identifier.span,
            name: identifier.name,
        })
    }
}
