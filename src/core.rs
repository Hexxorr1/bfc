use std::error::Error;
use crate::utils::log_if;
use crate::utils::OutputType;
use crate::input::Input;
use crate::utils::Flag;

pub mod preprocess;
pub mod builder;
pub mod assembly_converter;
pub mod code_optimizer;

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

    if !parsed_input.flags.contains(&Flag::OptimizationLevel(0)){
        log_if(log, "\tOptimizing the instruction list...");

        let passes=code_optimizer::optimize_code(&mut instruction_list, parsed_input.tape_size, 30)?;

        if log{
            println!("\t\tOptimization finished after iteration {} ({} improvements)", passes.0, passes.1);
        }
    }
  
    if parsed_input.flags.contains(&Flag::DiffirentOutput(OutputType::OperationList)){
        return Ok(format!("{:?}", instruction_list));
    }

    log_if(log, "\tConverting instruction list to assembly...");

    let mut cg=assembly_converter::AssemblyGenerator::new();
    let assembly_code=cg.convert_to_assembly(&instruction_list, parsed_input.tape_size)?;

    log_if(log, "\tConverting finished");

    //default output with no flag for a diffirent one
    Ok(assembly_code)
}
