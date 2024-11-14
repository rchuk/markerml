//! # General
//! This library provides [`parse`] function
//! that attempts to convert given MarkerML code to HTML.
//! MarkerML stands for Marker Markup Language.
//! It's a simple language for formatting and layouting
//! text similar to HTML.
//!
//! Note that custom components are not yet implemented
//! in the backend library.
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
//!
//! // Example of variable interpolation
//! box[x_align = ${align}] {}
//! ```
//!
//! ## Identifiers
//! Identifier must begin with ascii alphabetic character
//! or underscore, followed by sequence ascii alphanumeric
//! characters or underscores.
//! Identifiers are used for component and property names,
//! although there are also several built-in component
//! names which are not identifiers, namely: @ (text) and # (link).
//!
//! ## Types
//! There are several types in this language:
//! - `int` - integers like 0, 42, or -252
//! - `bool` - `true` or `false`
//! - `string` - "Text inside quotes", might also
//!   have interpolated variables like: "Hello, ${user_name}"
//!
//! - `slot` and `slot[]` for component composition
//!
//! ## Component definitions
//! Custom component specify list
//! of properties, their types and default values.
//! Component might either have text property or children.
//! It might also have single default property.
//! ```markerml
//! component custom_component[
//!     default prop: string,
//!     smth: int,
//!     abc: string = "Default value"
//! ] {
//!     box {
//!         header[level = ${smth}](Header text)
//!         @(${abc} ${prop})
//!     }
//! }
//! ```
//!
//! ## Modules
//! Module is a top-level entity that is a sequence
//! of components and component definitions.
//! That's what was used in previous examples.
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
//! # Built-in components
//! ## Box
//! Name: `box` \
//! Properties:
//! - `vertical`
//! - `horizontal`
//! - `x_align: string = "start" | "center" | "end"`. Default: `"start"`
//! - `y_align: string = "start" | "center" | "end"`. Default: `"start"`
//!
//! ## Text
//! Name: `@` \
//! Properties:
//! - `text content`
//!
//! ## Image
//! Name: `image` \
//! Properties:
//! - `default url: string`
//!
//! ## Link
//! Name: `#` \
//! Properties:
//! - `default url: string`
//! - `text name`
//!
//! ## List
//! Name: `list` \
//! Properties:
//! - `unordered`
//! - `ordered`
//! - `children: slot[]`
//!
//! ## Header
//! Name: `header` \
//! Properties:
//! - `default level: integer = 1`
//!
//! ## Paragraph
//! Name: `paragraph` \
//! Properties:
//! - `text content`
//!
//! # Grammar
//! ```
//! WHITESPACE = _{ (" " | "\t" | NEWLINE)+ }
//!
//! COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* ~ NEWLINE }
//!
//! integer = @{ "-"? ~ ASCII_DIGIT+ }
//!
//! bool = @{ "true" | "false" }
//!
//! identifier = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }
//!
//! literal_newline = @{ NEWLINE ~ (" " | "\t")* }
//!
//! string_literal_segment = @{ (!("$" | "\"" | NEWLINE) ~ ANY)+ }
//!
//! text_literal_segment = @{ (!("$" | ")" | NEWLINE) ~ ANY)+ }
//!
//! variable_interpolation = { "${" ~ identifier ~ "}" }
//!
//! string_segment = ${ literal_newline | variable_interpolation | string_literal_segment }
//!
//! text_segment = ${ literal_newline | variable_interpolation | text_literal_segment }
//!
//! string = @{ "\"" ~ string_segment* ~ "\"" }
//!
//! text = @{ "(" ~ text_segment* ~ ")" }
//!
//! value = { variable_interpolation | bool | string | integer }
//!
//! component_name = { "@" | "#" | identifier }
//!
//! default_property = { value }
//!
//! named_property = { identifier ~ "=" ~ value }
//!
//! flag_property = { identifier }
//!
//! property = { named_property | flag_property }
//!
//! properties_list = _{ property ~ ("," ~ property)* }
//!
//! properties = { "[" ~ (properties_list | (default_property ~ ("," ~ properties_list)?))?  ~ ","? ~ "]" }
//!
//! children = { "{" ~ component* ~ "}" }
//!
//! component = { component_name ~ properties? ~ children? ~ text? }
//!
//! ty = @{ "string" | "int" | "bool" | "slot[]" | "slot" }
//!
//! default_property_definition = { "default" ~ identifier ~ ":" ~ ty }
//!
//! text_property_definition = { "text" ~ identifier }
//!
//! named_property_definition = { identifier ~ ":" ~ ty ~ ("=" ~ value)? }
//!
//! property_definition = { default_property_definition | text_property_definition | named_property_definition }
//!
//! properties_definition_list = _{ property_definition ~ ("," ~ property_definition)* }
//!
//! properties_definition = { "[" ~ properties_definition_list? ~ "]" }
//!
//! component_definition = { "component" ~ identifier ~ properties_definition? ~ children? }
//!
//! module_item = _{ component_definition | component }
//!
//! module = { SOI ~ module_item* ~ EOI}
//! ```
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
