//! This is a crate that provides intermediate
//! representation for the MarkerML language.
//!
//! For the full grammar overview,
//! refer to the [`markerml`]() crate.

pub mod error;
pub mod ir;
pub mod ir_generator;

/// IR generator error
pub use error::IrGeneratorError;
/// Source code span. Used for error reporting
pub use markerml_frontend::parser::Span;

use markerml_frontend::ast;

/// Generates IR from the given AST
pub fn generate_ir(ast: ast::Module<Span>) -> Result<ir::Module<Span>, IrGeneratorError> {
    ir_generator::IrGenerator::new(ast).generate()
}
