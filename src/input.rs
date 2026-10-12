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
    let mut flag_args: Vec<String>=Vec::new();
    let mut last_flag: Option<String>=None;

    let mut argv=argv.chain(std::iter::once("-".to_string()));
    loop{
        let next_arg=argv.next();
        match next_arg{
            Some(arg) =>{
                match arg.chars().nth(0).ok_or("nice arg bro")?{
                    '-' =>{
                        match last_flag{
                            None => (),
                            Some(f) =>{
                                let internal_flag=Flag::from_string(&f, &flag_args)?;
                                flags.push(internal_flag);
                            }
                        }
                        last_flag=Some(arg);
                        flag_args.clear();
                    },
                    _  => flag_args.push(arg),
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
