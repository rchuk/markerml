
#[derive(Debug)]
pub struct Module {
    pub items: Vec<ModuleItem>,
}

#[derive(Debug)]
pub enum ModuleItem {
    Component(Component),
    ComponentDefinition(ComponentDefinition)
}

#[derive(Debug)]
pub struct Component {
    pub name: Identifier,
    pub properties: Option<Properties>,
    pub children: Option<Vec<Component>>,
    pub text: Option<Text>
}

#[derive(Debug, Default)]
pub struct Properties {
    pub default: Option<Value>,
    pub properties: Vec<Property>
}

#[derive(Debug)]
pub enum Property {
    KeyValue { key: Identifier, value: Value },
    Flag { key: Identifier }
}

#[derive(Debug)]
pub enum Value {
    String(StringValue),
    Integer(i64),
    Bool(bool),
    Variable(Identifier)
}

#[derive(Debug)]
pub struct ComponentDefinition {
    pub name: Identifier,
    pub properties: Option<PropertiesDefinition>,
    pub children: Option<Vec<Component>>
}

#[derive(Debug)]
pub struct PropertiesDefinition {
    pub properties: Vec<PropertyDefinition>
}

#[derive(Debug)]
pub enum PropertyDefinition {
    Text(TextPropertyDefinition),
    Default(NamedPropertyDefinition),
    Named(NamedPropertyDefinition)
}

#[derive(Debug)]
pub struct TextPropertyDefinition {
    pub name: Identifier
}

#[derive(Debug)]
pub struct NamedPropertyDefinition {
    pub name: Identifier,
    pub ty: Type,
    pub default_value: Option<Value>
}

#[derive(Debug)]
pub struct StringValue {
    pub segments: Vec<InterpolationSegment>
}

#[derive(Debug)]
pub struct Text {
    pub segments: Vec<InterpolationSegment>
}

#[derive(Debug)]
pub enum InterpolationSegment {
    Literal(String),
    Variable(Identifier)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Identifier(pub String);


#[derive(Debug, Clone)]
pub enum Type {
    String,
    Integer,
    Bool,
    Slot,
    SlotList
}
