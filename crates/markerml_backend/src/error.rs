use markerml_middleend::Span;
use miette::Diagnostic;
use thiserror::Error;

/// Represents HTML generation error, which often include semantic errors
#[derive(Debug, Error, Diagnostic)]
pub enum BackendError {
    /// Required default (or named) property is missing
    #[error(transparent)]
    #[diagnostic(transparent)]
    RequiredDefaultPropertyMissing(#[from] RequiredDefaultPropertyMissingError),
    /// Component is missing text
    #[error(transparent)]
    #[diagnostic(transparent)]
    TextMissing(#[from] TextMissingError),
    /// Unexpected type is used
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypeMismatch(#[from] TypeMismatchError),
    #[error("Unimplemented")]
    Unimplemented,
    #[error("TODO")]
    Todo,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Required default property, also known as '{name}' is missing")]
pub struct RequiredDefaultPropertyMissingError {
    /// Name of the property
    pub name: String,
    /// Span of the component
    #[label("Component defined here")]
    pub span: Span,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Text is missing from the component")]
pub struct TextMissingError {
    /// Span of the component
    #[label("Component defined here")]
    pub span: Span,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Type mismatch. Expected '{expected}', got '{got}'")]
pub struct TypeMismatchError {
    /// Message for expected type
    pub expected: &'static str,
    /// Message for resolved type
    pub got: &'static str,
    /// Span of the value
    #[label("Value defined here")]
    pub span: Span,
}
