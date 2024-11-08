use miette::Diagnostic;
use thiserror::Error;
use crate::parser::parser_error::MultiParserError;
use crate::validator::validator_error::ValidatorError;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    ParserError(#[from] MultiParserError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    ValidatorError(#[from] ValidatorError)
}
