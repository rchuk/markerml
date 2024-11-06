
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub items: Vec<ModuleItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem {
    Component(Component),
    ComponentDefinition(ComponentDefinition)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    pub name: Identifier,
    pub properties: Option<Properties>,
    pub children: Option<Vec<Component>>,
    pub text: Option<Text>
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Properties {
    pub default: Option<Value>,
    pub properties: Vec<Property>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Property {
    KeyValue { key: Identifier, value: Value },
    Flag { key: Identifier }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(StringValue),
    Integer(i64),
    Bool(bool),
    Variable(Identifier)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDefinition {
    pub name: Identifier,
    pub properties: Option<PropertiesDefinition>,
    pub children: Option<Vec<Component>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertiesDefinition {
    pub properties: Vec<PropertyDefinition>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyDefinition {
    Text(TextPropertyDefinition),
    Default(NamedPropertyDefinition),
    Named(NamedPropertyDefinition)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextPropertyDefinition {
    pub name: Identifier
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedPropertyDefinition {
    pub name: Identifier,
    pub ty: Type,
    pub default_value: Option<Value>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringValue {
    pub segments: Vec<InterpolationSegment>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub segments: Vec<InterpolationSegment>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpolationSegment {
    Literal(String),
    Variable(Identifier)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Identifier(pub String);


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    String,
    Integer,
    Bool,
    Slot,
    SlotList
}


impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier(name.to_owned())
    }
}

impl Text {
    pub fn from_literal(string: &str) -> Self {
        Text { segments: vec![InterpolationSegment::Literal(string.to_owned())] }
    }
}

impl StringValue {
    pub fn from_literal(string: &str) -> Self {
        StringValue { segments: vec![InterpolationSegment::Literal(string.to_owned())] }
    }
}

impl Into<ModuleItem> for Component {
    fn into(self) -> ModuleItem {
        ModuleItem::Component(self)
    }
}

impl Into<ModuleItem> for ComponentDefinition {
    fn into(self) -> ModuleItem {
        ModuleItem::ComponentDefinition(self)
    }
}

impl Into<Value> for StringValue {
    fn into(self) -> Value {
        Value::String(self)
    }
}
