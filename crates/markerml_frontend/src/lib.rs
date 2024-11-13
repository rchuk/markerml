//! This is a crate that provides parser
//! for the MarkerML language.
//!
//! For the full grammar overview,
//! refer to the [`markerml`]() crate.

pub mod ast;
pub mod ast_span_helpers;
pub mod parser;
pub mod span;

/// Parser error
pub use parser::ParserError;
/// Source code span. Used for error reporting
pub use span::Span;

/// Parses given code into AST
pub fn parse(code: &str) -> Result<ast::Module<Span>, Box<ParserError>> {
    parser::parse(code)
}
