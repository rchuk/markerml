mod span_helpers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<SpanT> {
    pub span: SpanT,
    pub items: Vec<ModuleItem<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem<SpanT> {
    Component(Component<SpanT>),
    ComponentDefinition(ComponentDefinition<SpanT>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<SpanT> {
    pub span: SpanT,
    pub name: Identifier<SpanT>,
    pub properties: Option<Properties<SpanT>>,
    pub children: Option<Vec<Component<SpanT>>>,
    pub text: Option<Text<SpanT>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties<SpanT> {
    pub span: SpanT,
    pub default: Option<Value<SpanT>>,
    pub properties: Vec<Property<SpanT>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property<SpanT> {
    pub span: SpanT,
    pub kind: PropertyKind<SpanT>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyKind<SpanT> {
    KeyValue { key: Identifier<SpanT>, value: Value<SpanT> },
    Flag { key: Identifier<SpanT> }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value<SpanT> {
    pub span: SpanT,
    pub kind: ValueKind<SpanT>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind<SpanT> {
    String(StringValue<SpanT>),
    Integer(i64),
    Bool(bool),
    Variable(Identifier<SpanT>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDefinition<SpanT> {
    pub span: SpanT,
    pub name: Identifier<SpanT>,
    pub properties: Option<PropertiesDefinition<SpanT>>,
    pub children: Option<Vec<Component<SpanT>>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertiesDefinition<SpanT> {
    pub span: SpanT,
    pub properties: Vec<PropertyDefinition<SpanT>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDefinition<SpanT> {
    pub span: SpanT,
    pub kind: PropertyDefinitionKind<SpanT>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyDefinitionKind<SpanT> {
    Text(TextPropertyDefinition<SpanT>),
    Default(NamedPropertyDefinition<SpanT>),
    Named(NamedPropertyDefinition<SpanT>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextPropertyDefinition<SpanT> {
    pub name: Identifier<SpanT>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedPropertyDefinition<SpanT> {
    pub name: Identifier<SpanT>,
    pub ty: Type<SpanT>,
    pub default_value: Option<Value<SpanT>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringValue<SpanT> {
    pub span: SpanT,
    pub segments: Vec<InterpolationSegment<SpanT>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<SpanT> {
    pub span: SpanT,
    pub segments: Vec<InterpolationSegment<SpanT>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterpolationSegment<SpanT> {
    pub span: SpanT,
    pub kind: InterpolationSegmentKind<SpanT>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpolationSegmentKind<SpanT> {
    Literal(String),
    Variable(Identifier<SpanT>)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Identifier<SpanT> {
    pub span: SpanT,
    pub name: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type<SpanT> {
    pub span: SpanT,
    pub kind: TypeKind
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    String,
    Integer,
    Bool,
    Slot,
    SlotList
}


impl<SpanT: Default> Identifier<SpanT> {
    pub fn from_literal(name: &str) -> Self {
        Identifier {
            span: Default::default(),
            name: name.to_owned()
        }
    }
}

impl<SpanT: Default> Text<SpanT> {
    pub fn from_literal(string: &str) -> Self {
        Text {
            span: Default::default(),
            segments: vec![
                InterpolationSegmentKind::Literal(string.to_owned())
                    .spanned(Default::default())
            ]
        }
    }
}

impl<SpanT: Default> StringValue<SpanT> {
    pub fn from_literal(string: &str) -> Self {
        StringValue {
            span: Default::default(),
            segments: vec![
                InterpolationSegmentKind::Literal(string.to_owned())
                    .spanned(Default::default())
            ]
        }
    }
}

impl<SpanT> Into<ModuleItem<SpanT>> for Component<SpanT> {
    fn into(self) -> ModuleItem<SpanT> {
        ModuleItem::Component(self)
    }
}

impl<SpanT> Into<ModuleItem<SpanT>> for ComponentDefinition<SpanT> {
    fn into(self) -> ModuleItem<SpanT> {
        ModuleItem::ComponentDefinition(self)
    }
}

impl<SpanT> Into<ValueKind<SpanT>> for StringValue<SpanT> {
    fn into(self) -> ValueKind<SpanT> {
        ValueKind::String(self)
    }
}

impl<SpanT: Default> Into<Property<SpanT>> for PropertyKind<SpanT> {
    fn into(self) -> Property<SpanT> {
        self.spanned(Default::default())
    }
}

impl<SpanT: Default> Into<Value<SpanT>> for ValueKind<SpanT> {
    fn into(self) -> Value<SpanT> {
        self.spanned(Default::default())
    }
}

impl<SpanT> ValueKind<SpanT> {
    pub fn spanned(self, span: SpanT) -> Value<SpanT> {
        Value { span, kind: self }
    }
}

impl<SpanT> InterpolationSegmentKind<SpanT> {
    pub fn spanned(self, span: SpanT) -> InterpolationSegment<SpanT> {
        InterpolationSegment { span, kind: self }
    }
}

impl<SpanT> PropertyKind<SpanT> {
    pub fn spanned(self, span: SpanT) -> Property<SpanT> {
        Property { span, kind: self }
    }
}

impl<SpanT> PropertyDefinitionKind<SpanT> {
    pub fn spanned(self, span: SpanT) -> PropertyDefinition<SpanT> {
        PropertyDefinition { span, kind: self }
    }
}

impl TypeKind {
    pub fn spanned<SpanT>(self, span: SpanT) -> Type<SpanT> {
        Type { span, kind: self }
    }
}
