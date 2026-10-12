use std::{error::Error, fs, process::ExitCode};
use crate::flags::Flag;
use crate::input::Input;
use std::time::SystemTime;
use std::process::Command;
use crate::core::log_if;

mod error;
mod input;
mod core;
mod flags;

mod tests;

//just a caller to run()
fn main() -> ExitCode{
    match run(){
        Ok(()) => ExitCode::SUCCESS,
        Err(e) =>{
            eprintln!("Error encountered:");
            eprintln!("\t{:?}", e);
            eprintln!("\t{}", e);
            ExitCode::FAILURE
        }
    }
}

//the real main() funcion, please do everything here
fn run() -> Result<(), Box<dyn Error>>{
    let time_begin=SystemTime::now();

    let mut argv=std::env::args();
    let mut parsed_input=input::parse_input(&mut argv)?;

    let log=parsed_input.flags.contains(&Flag::LogProgress);

    log_if(log, format!("Parsed succesfully with flags {:?}, compiling...", parsed_input.flags).as_str());

    let output_code=core::compile(&mut parsed_input)?;

    log_if(log, "Succesfully compiled, outputting...");

    let asm_file_path=parsed_input.destination_name.clone()+".asm";
    fs::write(&asm_file_path, output_code)?;

    if log{
        match time_begin.elapsed(){
            Ok(d) => println!("Success, {}ms", d.as_millis()),
            Err(_) => println!("congratulations, you have experienced time drift"),
        };
    }

    if parsed_input.flags.contains(&Flag::AutoAssemble){
        assemble(parsed_input)?;
    }

    return Ok(());
}

fn assemble(parsed_input: Input) -> Result<(), Box<dyn Error>>{
    let log=parsed_input.flags.contains(&Flag::LogProgress);
    log_if(log, "Assembling...");

    let asm_file_path=parsed_input.destination_name.clone()+".asm";
    let obj_file_path=parsed_input.destination_name.clone()+".obj";
    let status = Command::new("nasm").args(["-f", "win64", &asm_file_path, "-o", &obj_file_path]).status()?;
    if !status.success(){
        return Err("NASM failed".into());
    }

    log_if(log, "Linking...");

    let exe_file_path=parsed_input.destination_name.clone()+".exe";
    let status = Command::new("gcc").args([&obj_file_path, "-o", &exe_file_path]).status()?;
    if !status.success() {
        return Err("Linking failed".into());
    }

    log_if(log, "An executable has been created");

    return Ok(());
}
