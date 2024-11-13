use markerml_frontend::parser::Span;
use miette::Diagnostic;
use thiserror::Error;

/// Represents IR generation error, which
/// include simple semantic errors
#[derive(Debug, Error, Diagnostic)]
pub enum IrGeneratorError {
    /// Property name is defined multiple times
    #[error(transparent)]
    #[diagnostic(transparent)]
    DuplicatedProperty(#[from] DuplicatedPropertyError),
    /// Component has children and text at the same time
    #[error(transparent)]
    #[diagnostic(transparent)]
    TextComponentWithChildren(#[from] TextComponentWithChildrenError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    MultipleTextProperties(#[from] MultipleTextPropertiesError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    MultipleDefaultProperties(#[from] MultipleDefaultPropertiesError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    CircularDefinition(#[from] CircularDefinitionError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    DefaultPropertyWithValue(#[from] DefaultPropertyWithValueError),
}

#[derive(Debug, Error, Diagnostic)]
#[error("Property named '{name}' is duplicated")]
#[diagnostic(help("Rename one of the properties"))]
pub struct DuplicatedPropertyError {
    /// Name of the property
    pub name: String,
    /// Place where the property was first defined
    #[label("First defined here")]
    pub first: Span,
    /// Place where the property was defined again
    #[label("Then used here")]
    pub second: Span,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Text component can't have children")]
#[diagnostic(help("Either remove text or children from the component"))]
pub struct TextComponentWithChildrenError {
    /// Span with component name
    #[label("Component")]
    pub component_name: Span,
    /// Span with component children
    #[label("Children")]
    pub children: Span,
    /// Span with component text
    #[label("Text")]
    pub text: Span,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Component defines multiple `text` properties")]
#[diagnostic(help("Remove one of the `text` properties"))]
pub struct MultipleTextPropertiesError {
    /// Span with component name
    #[label("Component")]
    pub component_name: Span,
    /// Place where the property was first defined
    #[label("First defined here")]
    pub first: Span,
    /// Place where the property was defined again
    #[label("Then defined here")]
    pub second: Span,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Component defines multiple `default` properties")]
#[diagnostic(help("Remove one of the `default` properties"))]
pub struct MultipleDefaultPropertiesError {
    /// Span with component name
    #[label("Component")]
    pub component_name: Span,
    /// Place where the property was first defined
    #[label("First defined here")]
    pub first: Span,
    /// Place where the property was defined again
    #[label("Then defined here")]
    pub second: Span,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Component definition contains reference to itself")]
#[diagnostic(help("Remove component name from it's own children list"))]
pub struct CircularDefinitionError {
    /// Span with component name
    #[label("Component")]
    pub component_name: Span,
    /// Place where the same name was used
    #[label("Circular definition")]
    pub circular: Span,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Default property has default value")]
#[diagnostic(help("Remove default value from the default property"))]
pub struct DefaultPropertyWithValueError {
    /// Span with component name
    #[label("Component")]
    pub component_name: Span,
    /// Place where the same name was used
    #[label("Property")]
    pub property: Span,
    #[label("Default value")]
    pub default_value: Span,
}
