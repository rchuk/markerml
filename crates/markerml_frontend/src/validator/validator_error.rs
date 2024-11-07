use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ValidatorError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    DuplicatedPropertyName(#[from] DuplicatedPropertyNameError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    TextComponentWithChildren(#[from] TextComponentWithChildrenError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    MultipleTextProps(#[from] MultipleTextPropsError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    MultipleDefaultProps(#[from] MultipleDefaultPropsError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    MultipleSlotProps(#[from] MultipleSlotPropsError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    MultipleSlotListProps(#[from] MultipleSlotListPropsError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    SlotAndSlotListProps(#[from] SlotAndSlotListPropsError)
}

#[derive(Error, Diagnostic, Debug)]
#[error("property named '{name}' is used multiple times")]
#[diagnostic()]
pub struct DuplicatedPropertyNameError {
    #[label("component")]
    pub component_name: SourceSpan,
    #[label("first name was used here")]
    pub first: SourceSpan,
    #[label("then it was used here")]
    pub second: SourceSpan,
    pub name: String
}

#[derive(Error, Diagnostic, Debug)]
#[error("text component can't have any children")]
pub struct TextComponentWithChildrenError {
    #[label("component")]
    pub component_name: SourceSpan,
    #[label("text")]
    pub text: SourceSpan,
    #[label("children")]
    pub children: SourceSpan
}

#[derive(Error, Diagnostic, Debug)]
#[error("component can have only single 'text' property")]
pub struct MultipleTextPropsError {
    #[label("component")]
    pub component_name: SourceSpan,
    #[label("first 'text' property was defined here")]
    pub first: SourceSpan,
    #[label("then it was defined here")]
    pub second: SourceSpan
}

#[derive(Error, Diagnostic, Debug)]
#[error("component can have only single 'default' property")]
pub struct MultipleDefaultPropsError {
    #[label("component")]
    pub component_name: SourceSpan,
    #[label("first 'default' property was defined here")]
    pub first: SourceSpan,
    #[label("then it was defined here")]
    pub second: SourceSpan
}

#[derive(Error, Diagnostic, Debug)]
#[error("component can have only single 'slot' property")]
pub struct MultipleSlotPropsError {
    #[label("component")]
    pub component_name: SourceSpan,
    #[label("first 'slot' property was defined here")]
    pub first: SourceSpan,
    #[label("then it was defined here")]
    pub second: SourceSpan
}

#[derive(Error, Diagnostic, Debug)]
#[error("component can have only single 'slot[]' property")]
pub struct MultipleSlotListPropsError {
    #[label("component")]
    pub component_name: SourceSpan,
    #[label("first 'slot[]' property was defined here")]
    pub first: SourceSpan,
    #[label("then it was defined here")]
    pub second: SourceSpan
}

#[derive(Error, Diagnostic, Debug)]
#[error("component can have either 'slot' or 'slot[]' property")]
pub struct SlotAndSlotListPropsError {
    #[label("component")]
    pub component_name: SourceSpan,
    #[label("'slot' property is defined here")]
    pub slot: SourceSpan,
    #[label("'slot[]' property is defined here")]
    pub slot_list: SourceSpan
}

