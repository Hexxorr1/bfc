use std::error::Error;
use crate::error::BfcError;
use crate::input::Input;
use crate::flags::Flag;

pub mod preprocess;
pub mod builder;
pub mod assembly_converter;

#[derive(Debug, PartialEq)]
pub enum OutputType{
    Sbf,
    UnoptimizedOperationList,
    OptimizedOperationList,
}

impl OutputType{
    pub fn from_string(s: String) -> Result<Self, BfcError>{
        use OutputType::*;
        match s.as_str(){
            "sbf" => Ok(Sbf),
            "uol" => Ok(UnoptimizedOperationList),
            "ool" => Ok(OptimizedOperationList),
            _ => Err(BfcError::InvalidFlagArgs(vec![s])),
        }
    }
}

const fn op_chars() -> ([bool; 256], [bool; 256]){
    let mut normal_table=[false; 256];
    let normal_ops=b"+-<>[],.";

    let mut i=0;
    while i<normal_ops.len(){
        normal_table[normal_ops[i] as usize]=true;
        i+=1;
    }

    let mut sbf_table=normal_table;
    let sbf_ops=b"0123456789~";

    let mut i=0;
    while i<sbf_ops.len(){
        sbf_table[sbf_ops[i] as usize]=true;
        i+=1;
    }

    return (normal_table, sbf_table);
}

pub fn log_if(log: bool, text: &str){
    if log{
        println!("{}", text);
    }
}

//this destroys parsed_input
pub fn compile(parsed_input: &Input) -> Result<String, Box<dyn Error>>{
    let is_shortened=parsed_input.flags.contains(&Flag::ShortenedInput);
    let log=parsed_input.flags.contains(&Flag::LogProgress);

    //potentially expensive
    let mut working_code=parsed_input.source_code.clone();

    log_if(log, "\tRemoving comments...");
    preprocess::remove_comments(&mut working_code, is_shortened);

    if !is_shortened{
        log_if(log, "\tConverting source to sbf (shortening)...");
        preprocess::shorten(&mut working_code)?;
    }

    if parsed_input.flags.contains(&Flag::DiffirentOutput(OutputType::Sbf)){
        return Ok(working_code);
    }

    log_if(log, "\tConverting sbf to instruction list...");

    let mut instruction_list=builder::build_instruction_list(&working_code)?;

    if parsed_input.flags.contains(&Flag::DiffirentOutput(OutputType::UnoptimizedOperationList)){
        return Ok(format!("{:?}", instruction_list));
    }

    if !parsed_input.flags.contains(&Flag::OptimizationLevel(0)){
        log_if(log, "\tOptimizing the instruction list...");

        let passes=builder::optimize_code(&mut instruction_list, 10)?;

        if log{
            println!("\t\tOptimization finished after iteration {} ({} improvements)", passes+1, passes);
        }
    }
  
    if parsed_input.flags.contains(&Flag::DiffirentOutput(OutputType::OptimizedOperationList)){
        return Ok(format!("{:?}", instruction_list));
    }

    log_if(log, "\tConverting instruction list to assembly...");

    let mut cg=assembly_converter::AssemblyGenerator::new();
    let assembly_code=cg.convert_to_assembly(&instruction_list)?;

    log_if(log, "\tConverting finished");

    //default output with no flag for a diffirent one
    Ok(assembly_code)
}
