use std::error::Error;

use crate::error::BfcError;


#[derive(PartialEq)]
pub enum Flag{
    ShortenedInput,
}

impl Flag{
    pub fn from_string(s: String) -> Result<Self, Box<dyn Error>>{
        use Flag::*;
        match s.as_str(){
            "-sbf" => Ok(ShortenedInput),
            other => Err(Box::new(BfcError::InvalidFlag(other.to_string()))),
        }
    }
}
