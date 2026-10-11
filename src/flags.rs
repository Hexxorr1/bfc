use std::error::Error;

use crate::error::BfcError;


#[derive(Debug, PartialEq)]
pub enum Flag{
    ShortenedInput,
    LogProgress,
    OutputSbf,
    AssureSecure,
    OutputOperationList,
    OutputOperationListPretty,
    OutputOptimizedOperationList,
    OutputOptimizedOperationListPretty,
    AutoAssemble,
    OptimizationLevel(u8),
}

impl Flag{
    pub fn from_string(s: &String) -> Result<Self, Box<dyn Error>>{
        use Flag::*;
        match s.as_str(){
            "-si" => Ok(ShortenedInput),
            "-so" => Ok(OutputSbf),
            "-log" => Ok(LogProgress),
            "-ass" => Ok(AssureSecure),
            "-ol" => Ok(OutputOperationList),
            "-olp" => Ok(OutputOperationListPretty),
            "-ool" => Ok(OutputOptimizedOperationList),
            "-oop" => Ok(OutputOptimizedOperationListPretty),
            "-bin" => Ok(AutoAssemble),
            "-O0" => Ok(OptimizationLevel(0)),
            other => Err(Box::new(BfcError::InvalidFlag(other.to_string()))),
        }
    }

    pub fn is_implemented(&self) -> bool{
        use Flag::*;
        let unimplemented_flags=vec![AssureSecure];
        return !unimplemented_flags.contains(self);
    }
}
