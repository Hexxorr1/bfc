use std::{env::Args, error::Error, fs};
use crate::error::BfcError;
use crate::flags::Flag;

pub struct Input{
    pub source_code: String,
    pub destination_name: String,
    pub flags: Vec<Flag>,
}

pub fn parse_input(argv: &mut Args) -> Result<Input, Box<dyn Error>>{
    //discard "bfc.exe"
    argv.next();

    let source_path=argv.next().ok_or(BfcError::SourceNotSpecified)?;
    let source_code=fs::read_to_string(&source_path)?;

    let destination_name=source_path.rsplit_once('.').map_or("output", |(before, _)| before).to_string();

    let mut flags: Vec<Flag>=Vec::new();
    loop{
        let flag=argv.next();
        match flag{
            Some(f) =>{
                let internal_flag=Flag::from_string(&f)?;
                if internal_flag.is_implemented(){
                    flags.push(internal_flag);
                }else{
                    eprintln!("Warn: flag \"{f}\": {internal_flag:?} is not implemented and will be skipped");
                }
            },
            None => break
        }
    }

    let input=Input{
        source_code: source_code,
        destination_name: destination_name,
        flags: flags,
    };

    return Ok(input);
}
