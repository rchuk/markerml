pub mod validator_error;

use std::collections::HashMap;
use std::sync::Arc;
use itertools::Itertools;
use miette::SourceSpan;
use crate::ast::*;
use crate::common::span::Span;
use crate::validator::validator_error::*;

pub struct Validator<'a> {
    code: Arc<str>,
    ast: &'a Module<Span>
}

impl<'a> Validator<'a> {
    pub fn new(code: Arc<str>, ast: &'a Module<Span>) -> Self {
        Validator { code, ast }
    }

    pub fn validate(self) -> Result<(), ValidatorError> {
        self.validate_module(self.ast)
    }

    fn validate_module(&self, module: &Module<Span>) -> Result<(), ValidatorError> {
        for item in &module.items {
            self.validate_module_item(item)?;
        }

        Ok(())
    }

    fn validate_module_item(&self, item: &ModuleItem<Span>) -> Result<(), ValidatorError> {
        match item {
            ModuleItem::Component(component) => self.validate_component(component),
            ModuleItem::ComponentDefinition(component_def) => self.validate_component_definition(component_def)
        }
    }

    fn validate_component(&self, component: &Component<Span>) -> Result<(), ValidatorError> {
        if let (Some(text), Some(children)) = (&component.text, &component.children) {
            return Err(TextComponentWithChildrenError {
                component_name: component.name.span.to_miette_span(&self.code),
                text: text.span.to_miette_span(&self.code),
                children: children.span.to_miette_span(&self.code)
            }.into());
        }

        if let Some(properties) = &component.properties {
            self.validate_component_properties(properties, component)?;
        }

        Ok(())
    }

    fn validate_component_properties(&self, properties: &Properties<Span>, component: &Component<Span>) -> Result<(), ValidatorError> {
        let mut keys = HashMap::<String, SourceSpan>::new();

        for property in &properties.properties {
            match &property.kind {
                PropertyKind::Flag { key} | PropertyKind::KeyValue { key, .. } => {
                    if let Some(prev) = keys.get(&key.name) {
                        return Err(DuplicatedPropertyNameError {
                            component_name: component.name.span.to_miette_span(&self.code),
                            first: prev.clone(),
                            second: key.span.to_miette_span(&self.code),
                            name: key.name.clone()
                        }.into());
                    }
                    keys.insert(key.name.clone(), key.span.to_miette_span(&self.code));
                }
            };
        }

        Ok(())
    }

    fn validate_component_definition(&self, component_def: &ComponentDefinition<Span>) -> Result<(), ValidatorError> {
        if let Some(properties) = &component_def.properties {
            self.validate_component_definition_properties(properties, component_def)?;
        }

        Ok(())
    }

    fn validate_component_definition_properties(&self, properties: &PropertiesDefinition<Span>, component: &ComponentDefinition<Span>) -> Result<(), ValidatorError> {
        let PropertiesDefinition { properties, .. } = properties;

        if let Some((a, b)) = properties.iter()
            .filter(|prop| matches!(prop.kind, PropertyDefinitionKind::Text(_)))
            .next_tuple()
        {
            return Err(MultipleTextPropsError {
                component_name: component.name.span.to_miette_span(&self.code),
                first: a.span.to_miette_span(&self.code),
                second: b.span.to_miette_span(&self.code)
            }.into());
        }

        if let Some((a, b)) = properties.iter()
            .filter(|prop| matches!(prop.kind, PropertyDefinitionKind::Default(_)))
            .next_tuple()
        {
            return Err(MultipleDefaultPropsError {
                component_name: component.name.span.to_miette_span(&self.code),
                first: a.span.to_miette_span(&self.code),
                second: b.span.to_miette_span(&self.code)
            }.into());
        }

        if let Some((a, b)) = properties.iter()
            .filter(|prop| {
                matches!(prop.kind, PropertyDefinitionKind::Named(
                    NamedPropertyDefinition {
                    ty: Type { kind: TypeKind::Slot, .. },
                    ..
                    }
                ))
            })
            .next_tuple()
        {
            return Err(MultipleSlotPropsError {
                component_name: component.name.span.to_miette_span(&self.code),
                first: a.span.to_miette_span(&self.code),
                second: b.span.to_miette_span(&self.code)
            }.into());
        }

        if let Some((a, b)) = properties.iter()
            .filter(|prop| matches!(prop.kind, PropertyDefinitionKind::Named(
                NamedPropertyDefinition {
                    ty: Type { kind: TypeKind::SlotList, .. },
                    ..
                })
            ))
            .next_tuple()
        {
            return Err(MultipleSlotListPropsError {
                component_name: component.name.span.to_miette_span(&self.code),
                first: a.span.to_miette_span(&self.code),
                second: b.span.to_miette_span(&self.code)
            }.into());
        }

        let slot_prop = properties.iter().find(|prop| {
            matches!(prop.kind, PropertyDefinitionKind::Named(
                    NamedPropertyDefinition {
                    ty: Type { kind: TypeKind::Slot, .. },
                    ..
                    }
                ))
        });
        let slot_list_prop = properties.iter().find(|prop| {
            matches!(prop.kind, PropertyDefinitionKind::Named(
                NamedPropertyDefinition {
                    ty: Type { kind: TypeKind::SlotList, .. },
                    ..
                })
            )
        });

        if let (Some(slot_prop), Some(slot_list_prop)) = (slot_prop, slot_list_prop) {
            return Err(SlotAndSlotListPropsError {
                component_name: component.name.span.to_miette_span(&self.code),
                slot: slot_prop.span.to_miette_span(&self.code),
                slot_list: slot_list_prop.span.to_miette_span(&self.code)
            }.into());
        }

        let mut keys = HashMap::<String, SourceSpan>::new();
        for property in properties {
            match &property.kind {
                PropertyDefinitionKind::Text(TextPropertyDefinition { name }) |
                PropertyDefinitionKind::Named(NamedPropertyDefinition { name, .. }) |
                PropertyDefinitionKind::Default(NamedPropertyDefinition { name, ..}) => {
                    if let Some(prev) = keys.get(&name.name) {
                        return Err(DuplicatedPropertyNameError {
                            component_name: component.name.span.to_miette_span(&self.code),
                            first: prev.clone(),
                            second: name.span.to_miette_span(&self.code),
                            name: name.name.clone()
                        }.into());
                    }
                    keys.insert(name.name.clone(), name.span.to_miette_span(&self.code));
                }
            };
        }

        Ok(())
    }
}





