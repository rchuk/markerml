//! # About
//! This library provides [`parse`] function
//! that attempts to convert given MarkerML code to HTML.
//!
//! # Syntax
//! Here is an overview of the syntax.
//!
//! ## Component
//! The main building block is the component
//! ```markerml
//! // Turns into an empty div
//! box
//!
//! // Component might also specify properties
//! box[x_align = "center"]
//!
//! // And also have children
//! box[x_align = "center"] {
//!     box {
//!
//!     }
//! }
//! ```
//!
//! There are also text components, which can't have
//! children, but contain text instead.
//! ```markerml
//! // Example of a text component
//! header[1](This is a header)
//! box {
//!     @(This is some text)
//!     paragraph(This is a paragraph)
//! }
//! ```
//!
//! ## Properties
//! More about properties. Properties contain optional
//! default property that doesn't need to be named,
//! then list of either flag or key value properties.
//! ```markerml
//! // Example of a default property
//! #["//google.com"](Link to Google)
//!
//! // Example of flag property. It is simply an identifier
//! box[horizontal] {}
//!
//! // Example of named property
//! // Identifier is followed by `'='` sign and then by value
//! box[x_align = "center", y_align = "center"] {}
//!
//! // Flag and named properties can be combined
//! box[x_align = "center", vertical] {}
//! ```
//!
//! ## Identifier
//! Identifiers must begin with ascii alphabetic character
//! or underscore, followed by sequence ascii alphanumeric
//! characters or underscores.
//! Identifiers are used for component and property names,
//! although there are also several built-in component
//! names which are not identifiers, namely: @ (text) and # (link).
//!
//! ## Types
//! There are several types in this language:
//! - int - integers like 0, 42, or -252
//! - bool - `true` or `false`
//! - string - "Text inside quotes"
//!
//! **TODO**
//!
//! ## Comments
//! These examples make heavy use of the comments,
//! which are lines that begin with `//` and then ignored.
//!
//! ## Whitespaces
//! Most of the syntax elements can also be separated
//! by whitespaces, which are a sequence of space `' '`,
//! tab `'\t'` or newline `'\n'` characters.
//!

pub use markerml_backend;
pub use markerml_frontend;
pub use markerml_middleend;

use miette::Diagnostic;
use thiserror::Error;

/// Error type that encompasses all errors that might
/// occur while parsing code and generating HTML
#[derive(Debug, Error, Diagnostic)]
pub enum MarkermlError {
    /// Error from the parser stage
    #[error(transparent)]
    Parser(#[from] Box<markerml_frontend::ParserError>),
    /// Error from the Intermediate Representation generation stage
    #[error(transparent)]
    #[diagnostic(transparent)]
    IrGenerator(#[from] markerml_middleend::IrGeneratorError),
    /// Error from the HTML emitting stage
    #[error(transparent)]
    #[diagnostic(transparent)]
    Backend(#[from] markerml_backend::BackendError),
}

/// Converts given MarkerML code into HTML
pub fn parse(code: &str) -> Result<String, MarkermlError> {
    let ast = markerml_frontend::parse(code)?;
    let ir = markerml_middleend::generate_ir(ast)?;
    let html = markerml_backend::generate_html(ir)?;

    Ok(html)
}