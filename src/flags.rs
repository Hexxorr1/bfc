use std::error::Error;

use crate::error::BfcError;
use crate::core::OutputType;

#[derive(Debug, PartialEq)]
pub enum Flag{
    ShortenedInput,
    LogProgress,
    DiffirentOutput(OutputType),
    AutoAssemble,
    OptimizationLevel(u8),
}

impl Flag{
    pub fn from_string(s: &String, argv: &Vec<String>) -> Result<Self, Box<dyn Error>>{
        use Flag::*;
        use BfcError::TooLittleFlagArgs;
        match s.as_str(){
            "-si" => Ok(ShortenedInput),
            "-log" => Ok(LogProgress),
            "-bin" => Ok(AutoAssemble),
            "-ot" => Ok(DiffirentOutput(OutputType::from_string(argv.get(0).ok_or(TooLittleFlagArgs(DiffirentOutput(OutputType::Sbf)))?.to_string())?)),
            "-O" => Ok(OptimizationLevel(argv.get(0).ok_or(TooLittleFlagArgs(OptimizationLevel(0)))?.parse::<u8>()?)),
            other => Err(Box::new(BfcError::InvalidFlag(other.to_string()))),
        }
    }
}
