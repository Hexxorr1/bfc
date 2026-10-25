use std::{error::Error, fmt::Formatter};

use crate::utils::Flag;

#[derive(Debug, PartialEq)]
pub enum BfcError{
    SourceNotSpecified,
    InvalidFlag(String),
    InvalidFlagArgs(Vec<String>),
    TooLittleFlagArgs(Flag),
    InvalidInstruction(String),
    InvalidParenthesesOpen,
    InvalidParenthesesClose,
    ConversionToAsmNotImplemented(crate::core::builder::Instruction),
    OutOfTapeBounds,
    GenerationTooLarge,
}

impl Error for BfcError{}

impl std::fmt::Display for BfcError{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        use BfcError::*;
        let err_msg: String=match self{
            SourceNotSpecified => "the source file has not been specified".into(),
            InvalidFlag(flag) => format!("invalid flag has been passed: {flag}"),
            InvalidFlagArgs(argv) => format!("invalid arguments have been passed: {argv:?}"),
            TooLittleFlagArgs(f) => format!("too little args have been passed to {f:?}"),

            InvalidInstruction(c) => format!("failed when parsing instruction: '{c}'"),
            InvalidParenthesesOpen => "too many scope openings code".into(),
            InvalidParenthesesClose => "too many scope endings in code".into(),
            ConversionToAsmNotImplemented(i) => format!("the converter doesnt know how to process {i:?}"),
            OutOfTapeBounds => "found an out of array access".into(),
            GenerationTooLarge => "the generation count has overflown u64".into(),
        };
        write!(formatter, "{err_msg}")
    }
}

pub enum BfcWarn{
}

impl std::fmt::Display for BfcWarn{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        use BfcWarn::*;
        let warn_msg: String=match self{
            _ => "".into(),
        };
        write!(formatter, "{warn_msg}")
    }
}
