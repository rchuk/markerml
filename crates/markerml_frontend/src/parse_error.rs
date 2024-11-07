use miette::Diagnostic;
use thiserror::Error;
use crate::parser::ParserError;
use crate::validator::validator_error::ValidatorError;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("parser error {0:#?}")]
    ParserError(Vec<ParserError>),
    #[error(transparent)]
    #[diagnostic(transparent)]
    ValidatorError(#[from] ValidatorError)
}
