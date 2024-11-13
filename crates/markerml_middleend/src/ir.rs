use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<SpanT: Eq> {
    pub span: SpanT,
    pub items: Vec<ModuleItem<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem<SpanT: Eq> {
    Component(Component<SpanT>),
    ComponentDefinition(ComponentDefinition<SpanT>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component<SpanT: Eq> {
    pub span: SpanT,
    pub name: Identifier<SpanT>,
    pub properties: Properties<SpanT>,
    pub children: Vec<Component<SpanT>>,
    pub text: Option<Text<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties<SpanT: Eq> {
    pub default: Option<Value<SpanT>>,
    pub flag_properties: HashSet<Identifier<SpanT>>,
    pub named_properties: HashSet<Property<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property<SpanT: Eq> {
    pub span: SpanT,
    pub key: Identifier<SpanT>,
    pub value: Value<SpanT>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDefinition<SpanT: Eq> {
    pub span: SpanT,
    pub name: Identifier<SpanT>,
    pub properties: PropertiesDefinition<SpanT>,
    pub children: Vec<Component<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertiesDefinition<SpanT: Eq> {
    pub span: SpanT,
    pub text_property: Option<Identifier<SpanT>>,
    pub default_property: Option<PropertyDefinition<SpanT>>,
    pub properties: HashSet<PropertyDefinition<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDefinition<SpanT: Eq> {
    pub span: SpanT,
    pub name: Identifier<SpanT>,
    pub ty: Type<SpanT>,
    pub default_value: Option<Value<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value<SpanT: Eq> {
    pub span: SpanT,
    pub kind: ValueKind<SpanT>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind<SpanT: Eq> {
    String(StringValue<SpanT>),
    Integer(i64),
    Bool(bool),
    Variable(Identifier<SpanT>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringValue<SpanT: Eq> {
    pub span: SpanT,
    pub segments: Vec<InterpolationSegment<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<SpanT: Eq> {
    pub span: SpanT,
    pub segments: Vec<InterpolationSegment<SpanT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterpolationSegment<SpanT: Eq> {
    pub span: SpanT,
    pub kind: InterpolationSegmentKind<SpanT>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpolationSegmentKind<SpanT: Eq> {
    Literal(String),
    Variable(Identifier<SpanT>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier<SpanT: Eq> {
    pub span: SpanT,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type<SpanT: Eq> {
    pub span: SpanT,
    pub kind: TypeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    String,
    Integer,
    Bool,
    Slot,
    SlotList,
}

impl<SpanT: Eq> Identifier<SpanT> {
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl<SpanT: Default + Eq> Identifier<SpanT> {
    pub fn from_literal(name: &str) -> Self {
        Identifier {
            span: Default::default(),
            name: name.to_owned(),
        }
    }
}

impl<SpanT: Default + Eq> Text<SpanT> {
    pub fn from_literal(string: &str) -> Self {
        Text {
            span: Default::default(),
            segments: vec![
                InterpolationSegmentKind::Literal(string.to_owned()).spanned(Default::default())
            ],
        }
    }
}

impl<SpanT: Default + Eq> StringValue<SpanT> {
    pub fn from_literal(string: &str) -> Self {
        StringValue {
            span: Default::default(),
            segments: vec![
                InterpolationSegmentKind::Literal(string.to_owned()).spanned(Default::default())
            ],
        }
    }
}

impl<SpanT: Eq> From<Identifier<SpanT>> for String {
    fn from(identifier: Identifier<SpanT>) -> String {
        identifier.name
    }
}

impl<SpanT: Eq> From<Component<SpanT>> for ModuleItem<SpanT> {
    fn from(component: Component<SpanT>) -> Self {
        ModuleItem::Component(component)
    }
}

impl<SpanT: Eq> From<ComponentDefinition<SpanT>> for ModuleItem<SpanT> {
    fn from(def: ComponentDefinition<SpanT>) -> Self {
        ModuleItem::ComponentDefinition(def)
    }
}

impl<SpanT: Eq> From<StringValue<SpanT>> for ValueKind<SpanT> {
    fn from(value: StringValue<SpanT>) -> Self {
        ValueKind::String(value)
    }
}

impl<SpanT: Default + Eq> From<ValueKind<SpanT>> for Value<SpanT> {
    fn from(value: ValueKind<SpanT>) -> Self {
        value.spanned(Default::default())
    }
}

impl<SpanT: Eq> ValueKind<SpanT> {
    pub fn spanned(self, span: SpanT) -> Value<SpanT> {
        Value { span, kind: self }
    }
}

impl<SpanT: Eq> InterpolationSegmentKind<SpanT> {
    pub fn spanned(self, span: SpanT) -> InterpolationSegment<SpanT> {
        InterpolationSegment { span, kind: self }
    }
}

impl TypeKind {
    pub fn spanned<SpanT: Eq>(self, span: SpanT) -> Type<SpanT> {
        Type { span, kind: self }
    }
}

impl<SpanT: Eq> Hash for Property<SpanT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<SpanT: Eq> Hash for PropertyDefinition<SpanT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<SpanT: Eq> Hash for ComponentDefinition<SpanT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<SpanT: Eq> Borrow<Identifier<SpanT>> for Property<SpanT> {
    fn borrow(&self) -> &Identifier<SpanT> {
        &self.key
    }
}

impl<SpanT: Eq> Borrow<Identifier<SpanT>> for PropertyDefinition<SpanT> {
    fn borrow(&self) -> &Identifier<SpanT> {
        &self.name
    }
}

impl<SpanT: Eq> Borrow<Identifier<SpanT>> for ComponentDefinition<SpanT> {
    fn borrow(&self) -> &Identifier<SpanT> {
        &self.name
    }
}

impl<SpanT: Eq> Borrow<str> for Property<SpanT> {
    fn borrow(&self) -> &str {
        self.key.borrow()
    }
}

impl<SpanT: Eq> Borrow<str> for PropertyDefinition<SpanT> {
    fn borrow(&self) -> &str {
        self.name.borrow()
    }
}

impl<SpanT: Eq> Borrow<str> for Identifier<SpanT> {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl<SpanT: Eq> Hash for Identifier<SpanT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
