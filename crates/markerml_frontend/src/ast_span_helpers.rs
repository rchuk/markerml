use crate::ast::*;

/// Helper trait to change span type.
///
/// Useful for testing, where it can be
/// used to replace spans with unit type.
pub trait MapSpan<A> {
    /// Generic type of Self
    type Item<T>;

    /// Converts AST spans to different type
    fn map_span<F, B>(self, f: &mut F) -> Self::Item<B>
    where
        F: FnMut(A) -> B;
}

impl<SpanT> MapSpan<SpanT> for Module<SpanT> {
    type Item<T> = Module<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Module<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Module {
            span: f(self.span),
            items: self
                .items
                .into_iter()
                .map(|item| item.map_span(f))
                .collect(),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for ModuleItem<SpanT> {
    type Item<T> = ModuleItem<T>;
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

impl<SpanT> MapSpan<SpanT> for Component<SpanT> {
    type Item<T> = Component<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Component<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Component {
            span: f(self.span),
            name: self.name.map_span(f),
            properties: self.properties.map(|props| props.map_span(f)),
            children: self.children.map(|children| children.map_span(f)),
            text: self.text.map(|text| text.map_span(f)),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for Properties<SpanT> {
    type Item<T> = Properties<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Properties<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Properties {
            span: f(self.span),
            default: self.default.map(|value| value.map_span(f)),
            properties: self
                .properties
                .into_iter()
                .map(|prop| prop.map_span(f))
                .collect(),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for Property<SpanT> {
    type Item<T> = Property<T>;
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

impl<SpanT> MapSpan<SpanT> for PropertyKind<SpanT> {
    type Item<T> = PropertyKind<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> PropertyKind<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        match self {
            PropertyKind::KeyValue { key, value } => PropertyKind::KeyValue {
                key: key.map_span(f),
                value: value.map_span(f),
            },
            PropertyKind::Flag { key } => PropertyKind::Flag {
                key: key.map_span(f),
            },
        }
    }
}

impl<SpanT> MapSpan<SpanT> for Value<SpanT> {
    type Item<T> = Value<T>;
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

impl<SpanT> MapSpan<SpanT> for ValueKind<SpanT> {
    type Item<T> = ValueKind<T>;
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

impl<SpanT> MapSpan<SpanT> for ComponentDefinition<SpanT> {
    type Item<T> = ComponentDefinition<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> ComponentDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        ComponentDefinition {
            span: f(self.span),
            name: self.name.map_span(f),
            properties: self.properties.map(|props| props.map_span(f)),
            children: self.children.map(|children| children.map_span(f)),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for ComponentChildren<SpanT> {
    type Item<T> = ComponentChildren<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> ComponentChildren<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        ComponentChildren {
            span: f(self.span),
            children: self
                .children
                .into_iter()
                .map(|child| child.map_span(f))
                .collect(),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for PropertiesDefinition<SpanT> {
    type Item<T> = PropertiesDefinition<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> PropertiesDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        PropertiesDefinition {
            span: f(self.span),
            properties: self
                .properties
                .into_iter()
                .map(|prop| prop.map_span(f))
                .collect(),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for PropertyDefinition<SpanT> {
    type Item<T> = PropertyDefinition<T>;
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

impl<SpanT> MapSpan<SpanT> for PropertyDefinitionKind<SpanT> {
    type Item<T> = PropertyDefinitionKind<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> PropertyDefinitionKind<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        match self {
            PropertyDefinitionKind::Text(text) => PropertyDefinitionKind::Text(text.map_span(f)),
            PropertyDefinitionKind::Default(named) => {
                PropertyDefinitionKind::Default(named.map_span(f))
            }
            PropertyDefinitionKind::Named(named) => {
                PropertyDefinitionKind::Named(named.map_span(f))
            }
        }
    }
}

impl<SpanT> MapSpan<SpanT> for Identifier<SpanT> {
    type Item<T> = Identifier<T>;
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

impl<SpanT> MapSpan<SpanT> for Text<SpanT> {
    type Item<T> = Text<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> Text<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        Text {
            span: f(self.span),
            segments: self
                .segments
                .into_iter()
                .map(|seg| seg.map_span(f))
                .collect(),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for TextPropertyDefinition<SpanT> {
    type Item<T> = TextPropertyDefinition<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> TextPropertyDefinition<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        TextPropertyDefinition {
            name: self.name.map_span(f),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for NamedPropertyDefinition<SpanT> {
    type Item<T> = NamedPropertyDefinition<T>;
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

impl<SpanT> MapSpan<SpanT> for StringValue<SpanT> {
    type Item<T> = StringValue<T>;
    fn map_span<F, NewSpanT>(self, f: &mut F) -> StringValue<NewSpanT>
    where
        F: FnMut(SpanT) -> NewSpanT,
    {
        StringValue {
            span: f(self.span),
            segments: self
                .segments
                .into_iter()
                .map(|seg| seg.map_span(f))
                .collect(),
        }
    }
}

impl<SpanT> MapSpan<SpanT> for InterpolationSegment<SpanT> {
    type Item<T> = InterpolationSegment<T>;
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

impl<SpanT> MapSpan<SpanT> for InterpolationSegmentKind<SpanT> {
    type Item<T> = InterpolationSegmentKind<T>;
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

impl<SpanT> MapSpan<SpanT> for Type<SpanT> {
    type Item<T> = Type<T>;
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
