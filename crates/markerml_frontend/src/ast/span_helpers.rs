use super::*;

impl<SpanT> Module<SpanT> {
    pub fn map_span<F, NewSpanT>(self, mut f: F) -> Module<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Module {
            span: f(self.span),
            items: self.items.into_iter().map(|item| item.map_span(&mut f)).collect(),
        }
    }
}

impl<SpanT> ModuleItem<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> ModuleItem<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        match self {
            ModuleItem::Component(component) => ModuleItem::Component(component.map_span(f)),
            ModuleItem::ComponentDefinition(definition) => {
                ModuleItem::ComponentDefinition(definition.map_span(f))
            }
        }
    }
}

impl<SpanT> Component<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Component<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Component {
            span: f(self.span),
            name: self.name.map_span(f),
            properties: self.properties.map(|props| props.map_span(f)),
            children: self.children.map(|children| {
                children.into_iter().map(|child| child.map_span(f)).collect()
            }),
            text: self.text.map(|text| text.map_span(f)),
        }
    }
}

impl<SpanT> Properties<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Properties<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Properties {
            span: f(self.span),
            default: self.default.map(|value| value.map_span(f)),
            properties: self.properties.into_iter().map(|prop| prop.map_span(f)).collect(),
        }
    }
}

impl<SpanT> Property<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Property<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Property {
            span: f(self.span),
            kind: self.kind.map_span(f),
        }
    }
}

impl<SpanT> PropertyKind<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> PropertyKind<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        match self {
            PropertyKind::KeyValue { key, value } => PropertyKind::KeyValue {
                key: key.map_span(f),
                value: value.map_span(f),
            },
            PropertyKind::Flag { key } => PropertyKind::Flag { key: key.map_span(f) },
        }
    }
}

impl<SpanT> Value<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Value<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Value {
            span: f(self.span),
            kind: self.kind.map_span(f),
        }
    }
}

impl<SpanT> ValueKind<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> ValueKind<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        match self {
            ValueKind::String(string_value) => ValueKind::String(string_value.map_span(f)),
            ValueKind::Integer(value) => ValueKind::Integer(value),
            ValueKind::Bool(value) => ValueKind::Bool(value),
            ValueKind::Variable(identifier) => ValueKind::Variable(identifier.map_span(f)),
        }
    }
}

impl<SpanT> ComponentDefinition<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> ComponentDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        ComponentDefinition {
            span: f(self.span),
            name: self.name.map_span(f),
            properties: self.properties.map(|props| props.map_span(f)),
            children: self.children.map(|children| {
                children.into_iter().map(|child| child.map_span(f)).collect()
            }),
        }
    }
}

impl<SpanT> PropertiesDefinition<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> PropertiesDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        PropertiesDefinition {
            span: f(self.span),
            properties: self.properties.into_iter().map(|prop| prop.map_span(f)).collect(),
        }
    }
}

impl<SpanT> PropertyDefinition<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> PropertyDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        PropertyDefinition {
            span: f(self.span),
            kind: self.kind.map_span(f),
        }
    }
}

impl<SpanT> PropertyDefinitionKind<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> PropertyDefinitionKind<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        match self {
            PropertyDefinitionKind::Text(text) => PropertyDefinitionKind::Text(text.map_span(f)),
            PropertyDefinitionKind::Default(named) => PropertyDefinitionKind::Default(named.map_span(f)),
            PropertyDefinitionKind::Named(named) => PropertyDefinitionKind::Named(named.map_span(f)),
        }
    }
}

impl<SpanT> Identifier<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Identifier<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Identifier {
            span: f(self.span),
            name: self.name,
        }
    }
}

impl<SpanT> Text<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Text<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Text {
            span: f(self.span),
            segments: self.segments.into_iter().map(|seg| seg.map_span(f)).collect(),
        }
    }
}

impl<SpanT> TextPropertyDefinition<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> TextPropertyDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        TextPropertyDefinition {
            name: self.name.map_span(f),
        }
    }
}

impl<SpanT> NamedPropertyDefinition<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> NamedPropertyDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        NamedPropertyDefinition {
            name: self.name.map_span(f),
            ty: self.ty.map_span(f),
            default_value: self.default_value.map(|value| value.map_span(f)),
        }
    }
}

impl<SpanT> StringValue<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> StringValue<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        StringValue {
            span: f(self.span),
            segments: self.segments.into_iter().map(|seg| seg.map_span(f)).collect(),
        }
    }
}

impl<SpanT> InterpolationSegment<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> InterpolationSegment<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        InterpolationSegment {
            span: f(self.span),
            kind: self.kind.map_span(f),
        }
    }
}

impl<SpanT> InterpolationSegmentKind<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> InterpolationSegmentKind<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        match self {
            InterpolationSegmentKind::Literal(text) => InterpolationSegmentKind::Literal(text),
            InterpolationSegmentKind::Variable(identifier) => {
                InterpolationSegmentKind::Variable(identifier.map_span(f))
            }
        }
    }
}

impl<SpanT> Type<SpanT> {
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Type<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Type {
            span: f(self.span),
            kind: self.kind,
        }
    }
}
