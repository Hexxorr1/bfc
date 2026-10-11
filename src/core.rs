use std::error::Error;
use crate::input::Input;
use crate::flags::Flag;

pub mod preprocess;
pub mod builder;
pub mod assembly_converter;

//this destroys parsed_input
pub fn compile(parsed_input: &mut Input) -> Result<String, Box<dyn Error>>{
    let is_shortened=parsed_input.flags.contains(&Flag::ShortenedInput);

    if parsed_input.flags.contains(&Flag::LogProgress){
        println!("\tRemoving comments...");
    }
    preprocess::remove_comments(&mut parsed_input.source_code, is_shortened);

    if !is_shortened{
        if parsed_input.flags.contains(&Flag::LogProgress){
            println!("\tConverting source to sbf (shortening)...");
        }
        preprocess::shorten(&mut parsed_input.source_code)?;
    }

    if parsed_input.flags.contains(&Flag::OutputSbf){
        return Ok(parsed_input.source_code.clone());
    }

    //we are now only working with pure sbf

    if parsed_input.flags.contains(&Flag::LogProgress){
        println!("\tConverting sbf to instruction list...");
    }

    let mut instruction_list=builder::build_instruction_list(&parsed_input.source_code)?;

    if parsed_input.flags.contains(&Flag::OutputOperationList){
        return Ok(format!("{:?}", instruction_list));
    }
    if parsed_input.flags.contains(&Flag::OutputOperationListPretty){
        return Ok(format!("{:#?}", instruction_list));
    }

    if !parsed_input.flags.contains(&Flag::OptimizationLevel(0)){
        if parsed_input.flags.contains(&Flag::LogProgress){
            println!("\tOptimizing the instruction list...");
        }

        let optimization_output=builder::optimize_code(&instruction_list, 10)?;
        instruction_list=optimization_output.0;

        if parsed_input.flags.contains(&Flag::LogProgress){
            let i=optimization_output.1;
            println!("\t\tOptimization finished after iteration {} ({} improvements)", i+1, i);
        }
    }
  
    if parsed_input.flags.contains(&Flag::OutputOptimizedOperationList){
        return Ok(format!("{:?}", instruction_list));
    }
    if parsed_input.flags.contains(&Flag::OutputOptimizedOperationListPretty){
        return Ok(format!("{:#?}", instruction_list));
    }

    if parsed_input.flags.contains(&Flag::LogProgress){
        println!("\tConverting instruction list to assembly...");
    }

    let mut cg=assembly_converter::AssemblyGenerator::new();
    let assembly_code=cg.convert_to_assembly(&instruction_list)?;

    if parsed_input.flags.contains(&Flag::LogProgress){
        println!("\tConverting finished :3");
    }

    //default output with no flag for a diffirent one
    Ok(assembly_code)
}
