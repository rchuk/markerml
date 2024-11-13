/// Represents top level module
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<SpanT> {
    pub span: SpanT,
    pub items: Vec<ModuleItem<SpanT>>,
}

/// Represents module item: component or component definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem<SpanT> {
    Component(Component<SpanT>),
    ComponentDefinition(ComponentDefinition<SpanT>),
}

/// Represents component. It has name
/// and also might contain properties, children and text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<SpanT> {
    pub span: SpanT,
    pub name: Identifier<SpanT>,
    pub properties: Option<Properties<SpanT>>,
    pub children: Option<ComponentChildren<SpanT>>,
    pub text: Option<Text<SpanT>>,
}

/// Represents component properties.
/// Might contain single default property and list
/// of named or flag properties
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties<SpanT> {
    pub span: SpanT,
    pub default: Option<Value<SpanT>>,
    pub properties: Vec<Property<SpanT>>,
}

/// Represents key-value or flag property along with a span
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property<SpanT> {
    pub span: SpanT,
    pub kind: PropertyKind<SpanT>,
}

/// Represents key-value or flag property
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyKind<SpanT> {
    KeyValue {
        key: Identifier<SpanT>,
        value: Value<SpanT>,
    },
    Flag {
        key: Identifier<SpanT>,
    },
}

/// Represents list of component children
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentChildren<SpanT> {
    pub span: SpanT,
    pub children: Vec<Component<SpanT>>,
}

/// Represents component definition.
/// Consists of name, optional properties and children
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDefinition<SpanT> {
    pub span: SpanT,
    pub name: Identifier<SpanT>,
    pub properties: Option<PropertiesDefinition<SpanT>>,
    pub children: Option<ComponentChildren<SpanT>>,
}

/// Represents list of property definitions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertiesDefinition<SpanT> {
    pub span: SpanT,
    pub properties: Vec<PropertyDefinition<SpanT>>,
}

/// Represents property definition along with a span
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDefinition<SpanT> {
    pub span: SpanT,
    pub kind: PropertyDefinitionKind<SpanT>,
}

/// Represents property definition, which can be text, default, or named
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyDefinitionKind<SpanT> {
    Text(TextPropertyDefinition<SpanT>),
    Default(NamedPropertyDefinition<SpanT>),
    Named(NamedPropertyDefinition<SpanT>),
}

/// Represents text property definition (which always has string type)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextPropertyDefinition<SpanT> {
    pub name: Identifier<SpanT>,
}

/// Represents named property definition, consisting of name, type
/// and optional default value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedPropertyDefinition<SpanT> {
    pub name: Identifier<SpanT>,
    pub ty: Type<SpanT>,
    pub default_value: Option<Value<SpanT>>,
}

/// Represents value along with a span
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value<SpanT> {
    pub span: SpanT,
    pub kind: ValueKind<SpanT>,
}

/// Represents value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind<SpanT> {
    String(StringValue<SpanT>),
    Integer(i64),
    Bool(bool),
    Variable(Identifier<SpanT>),
}

/// Represents string value, consisting of multiple interpolation segments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringValue<SpanT> {
    pub span: SpanT,
    pub segments: Vec<InterpolationSegment<SpanT>>,
}

/// Represents text value, consisting of multiple interpolation segments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<SpanT> {
    pub span: SpanT,
    pub segments: Vec<InterpolationSegment<SpanT>>,
}

/// Represents interpolation segment along with a span
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterpolationSegment<SpanT> {
    pub span: SpanT,
    pub kind: InterpolationSegmentKind<SpanT>,
}

/// Represents interpolation segment: literal string or variable interpolation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpolationSegmentKind<SpanT> {
    Literal(String),
    Variable(Identifier<SpanT>),
}

/// Represents identifier
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier<SpanT> {
    pub span: SpanT,
    pub name: String,
}

/// Represents type along with a span
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type<SpanT> {
    pub span: SpanT,
    pub kind: TypeKind,
}

/// Represents type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    String,
    Integer,
    Bool,
    Slot,
    SlotList,
}

impl<SpanT> Identifier<SpanT> {
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl<SpanT: Default> Identifier<SpanT> {
    /// Creates identifier. Useful for testing
    pub fn from_literal(name: &str) -> Self {
        Identifier {
            span: Default::default(),
            name: name.to_owned(),
        }
    }
}

impl<SpanT: Default> Text<SpanT> {
    /// Creates text from single literal span. Useful for testing.
    pub fn from_literal(string: &str) -> Self {
        Text {
            span: Default::default(),
            segments: vec![
                InterpolationSegmentKind::Literal(string.to_owned()).spanned(Default::default())
            ],
        }
    }
}

impl<SpanT: Default> StringValue<SpanT> {
    /// Creates string from single literal span. Useful for testing.
    pub fn from_literal(string: &str) -> Self {
        StringValue {
            span: Default::default(),
            segments: vec![
                InterpolationSegmentKind::Literal(string.to_owned()).spanned(Default::default())
            ],
        }
    }
}

impl<SpanT> From<Identifier<SpanT>> for String {
    fn from(identifier: Identifier<SpanT>) -> Self {
        identifier.name
    }
}

impl<SpanT> From<Component<SpanT>> for ModuleItem<SpanT> {
    fn from(component: Component<SpanT>) -> Self {
        ModuleItem::Component(component)
    }
}

impl<SpanT> From<ComponentDefinition<SpanT>> for ModuleItem<SpanT> {
    fn from(def: ComponentDefinition<SpanT>) -> Self {
        ModuleItem::ComponentDefinition(def)
    }
}

impl<SpanT> From<StringValue<SpanT>> for ValueKind<SpanT> {
    fn from(value: StringValue<SpanT>) -> Self {
        ValueKind::String(value)
    }
}

impl<SpanT: Default> From<StringValue<SpanT>> for Value<SpanT> {
    fn from(value: StringValue<SpanT>) -> Self {
        ValueKind::String(value).into()
    }
}

impl<SpanT: Default> From<PropertyKind<SpanT>> for Property<SpanT> {
    fn from(value: PropertyKind<SpanT>) -> Self {
        value.spanned(Default::default())
    }
}

impl<SpanT: Default> From<ValueKind<SpanT>> for Value<SpanT> {
    fn from(value: ValueKind<SpanT>) -> Self {
        value.spanned(Default::default())
    }
}

impl<SpanT: Default> From<TypeKind> for Type<SpanT> {
    fn from(value: TypeKind) -> Self {
        value.spanned(Default::default())
    }
}

impl<SpanT: Default> From<PropertyDefinitionKind<SpanT>> for PropertyDefinition<SpanT> {
    fn from(value: PropertyDefinitionKind<SpanT>) -> Self {
        value.spanned(Default::default())
    }
}

impl<SpanT> PropertyKind<SpanT> {
    /// Creates property from kind and span
    pub fn spanned(self, span: SpanT) -> Property<SpanT> {
        Property { span, kind: self }
    }
}

impl<SpanT> PropertyDefinitionKind<SpanT> {
    /// Creates property definition from kind and span
    pub fn spanned(self, span: SpanT) -> PropertyDefinition<SpanT> {
        PropertyDefinition { span, kind: self }
    }
}

impl<SpanT> ValueKind<SpanT> {
    /// Creates vale from kind and span
    pub fn spanned(self, span: SpanT) -> Value<SpanT> {
        Value { span, kind: self }
    }
}

impl<SpanT> InterpolationSegmentKind<SpanT> {
    /// Creates interpolation segment from kind and span
    pub fn spanned(self, span: SpanT) -> InterpolationSegment<SpanT> {
        InterpolationSegment { span, kind: self }
    }
}

impl TypeKind {
    /// Creates type from kind and span
    pub fn spanned<SpanT>(self, span: SpanT) -> Type<SpanT> {
        Type { span, kind: self }
    }
}
