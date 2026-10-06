use std::error::Error;
use crate::input::Input;
use crate::flags::Flag;

mod preprocess;
mod tree_builder;
mod assembly_converter;

//this destroys parsed_input
pub fn compile(parsed_input: &mut Input) -> Result<String, Box<dyn Error>>{
    let is_shortened=parsed_input.flags.contains(&Flag::ShortenedInput);
    preprocess::remove_comments(&mut parsed_input.source_code, is_shortened);

    if !is_shortened{
        preprocess::shorten(&mut parsed_input.source_code);
    }

    //we are now only working with pure sbf

    //quick test
    Ok(parsed_input.source_code.clone())
}
