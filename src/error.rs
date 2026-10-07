use std::{error::Error, fmt::{Formatter, format}};

#[derive(Debug)]
pub enum BfcError{
    SourceNotSpecified,
    DestinationNotSpecified,
    InvalidFlag(String),
    InvalidInstruction(String),
}

impl Error for BfcError{}

impl std::fmt::Display for BfcError{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        use BfcError::*;
        let err_msg: String=match self{
            SourceNotSpecified => "the source file has not been specified".into(),
            DestinationNotSpecified => "the target file has not been specified".into(),
            InvalidFlag(flag) => format!("invalid flag has been passed: {flag}"),
            InvalidInstruction(c) => format!("failed when parsing instructiom: {c}")
        };
        write!(formatter, "{err_msg}")
    }
}
