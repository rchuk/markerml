use miette::{Diagnostic, LabeledSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("Parser error")]
pub struct MultiParserError {
    #[related]
    pub related: Vec<ParserError>
}

#[derive(Error, Diagnostic, Debug)]
#[error("Unexpected token")]
pub struct ParserError {
    pub label: &'static str,
    #[label(collection)]
    pub spans: Vec<LabeledSpan>
}
