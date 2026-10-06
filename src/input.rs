use std::{env::Args, error::Error, fs};
use crate::error::BfcError;
use crate::flags::Flag;

pub struct Input{
    pub source_code: String,
    pub destination_path: String,
    pub flags: Vec<Flag>,
}

pub fn parse_input(argv: &mut Args) -> Result<Input, Box<dyn Error>>{
    //discard "bfc.exe"
    argv.next();

    let source_path=argv.next().ok_or(BfcError::SourceNotSpecified)?;
    let source_code=fs::read_to_string(&source_path)?;

    let destination_path=argv.next().ok_or(BfcError::DestinationNotSpecified)?;

    let mut flags: Vec<Flag>=Vec::new();
    loop{
        let flag=argv.next();
        match flag{
            Some(f) =>{
                flags.push(Flag::from_string(f)?);
            },
            None => break
        }
    }

    let input=Input{
        source_code: source_code,
        destination_path: destination_path,
        flags: Vec::new(),
    };

    return Ok(input);
}
