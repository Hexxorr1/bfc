use std::{error::Error, fs, process::ExitCode};

mod error;
mod input;
mod core;
mod flags;

//just a caller to run()
fn main() -> ExitCode{
    match run(){
        Ok(()) => ExitCode::SUCCESS,
        Err(e) =>{
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}

//the real main() funcion, please do everything here
fn run() -> Result<(), Box<dyn Error>>{
    let mut argv=std::env::args();

    println!("Parsing...");
    let mut parsed_input=input::parse_input(&mut argv)?;

    println!("Parsed succesfully, compiling...");
    let output_code=core::compile(&mut parsed_input)?;

    println!("Succesfully compiled, outputting...");
    fs::write(parsed_input.destination_path, output_code)?;

    println!("Success");
    return Ok(());
}
