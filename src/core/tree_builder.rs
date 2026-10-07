use crate::error::BfcError;
use std::error::Error;

pub enum Instruction{
    Add(u8),
    Sub(u8),
    Left(usize),
    Right(usize),
    LParen(usize),
    RParen(usize),
    Read(),
    Print()
}

impl Instruction{
    //some dont need v
    fn from(c: char, v: usize) -> Result<Self, BfcError>{
        use Instruction::*;
        match c{
            '+' => Ok(Add(v as u8)),
            '-' => Ok(Sub(v as u8)),
            '<' => Ok(Left(v)),
            '>' => Ok(Right(v)),
            '[' => Ok(LParen(v)),
            ']' => Ok(RParen(v)),
            ',' => Ok(Read()),
            '.' => Ok(Print()),
            other => Err(BfcError::InvalidInstruction(other.to_string()))
        }
    }
}

pub fn build_instruction_set(code: &String) -> Result<Vec<Instruction>, Box<dyn Error>>{
    let mut instructions=Vec::new();

    let shortenable_chars=vec!['+', '-', '<', '>'];
    let mut counter=(' ', 0usize);

    for c in code.chars(){
        if shortenable_chars.contains(&c){
            if counter.0==c||counter.0==' '{
                counter=(c, counter.1+1);
            }else{
                instructions.push(Instruction::from(counter.0, counter.1)?);
                counter=(c, 1);
            }
        }else{
            instructions.push(Instruction::from(c, 0)?);
            counter=(' ', 0);
        }
    }

    if counter.1>0{
        instructions.push(Instruction::from(counter.0, counter.1)?);
    }

    return Ok(instructions);
}
