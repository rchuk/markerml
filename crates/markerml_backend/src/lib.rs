//! This is a crate that provides HTML generation
//! backend for the MarkerML language.
//!
//! For the full grammar overview,
//! refer to the [`markerml`](https://crates.io/crates/markerml) crate.

pub mod error;
pub mod html_generator;

pub use error::BackendError;

use markerml_middleend::Span;

/// Generates HTML from the given IR
pub fn generate_html(ir: markerml_middleend::ir::Module<Span>) -> Result<String, BackendError> {
    html_generator::HtmlGenerator::new(ir).generate()
}
