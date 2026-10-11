use crate::{core::builder::Instruction::{LoopBegin, LoopEnd}, error::BfcError};
use std::error::Error;

pub mod code_optimizer;
pub use code_optimizer::optimize_code;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction{
    //basic bf operations
    Add(u8),
    Move(isize),
    LoopBegin(usize),
    LoopEnd(usize),
    Read,
    Print,

    //custom sbf ops
    Debug,

    //custom, optimized operations, please specify what do they do after conversion
    //these are NOT to be represented in bf or sbf, theyre only used for optimization
    //NOTE: SUBJECT TO CHANGE, operations will be added and removed
    //PLEASE LOOK INTO OPTIMIZATION WHEN CHANGING WHAT THE OPERATION DOES

        //basic operations
    Set(u8),                                                                //sets the current cell to (u8)
    Seek(u8, isize),                                                        //repeatedly goes (isize) in memory until it finds (u8), MODIFIES START AND END BY (-u8) AND (u8)
    SeekClean(u8, isize),                                                   //performs a Seek() without modifying start and end
    AddSelfToAnother(isize),                                                //the cell (isize) to the right is incremented by this, sets this cell to 0
    AddSelfToAnotherByMarks(u8, isize, isize, u8, isize, isize),            //AddSelfToAnother but with two user-placed markers (Seek() points) and offset to them
    Copy(isize),                                                            //copies the value of the current cell into (isize) away
    CompareValue(u8, u8, u8, u8),                                           //compares self and (u8.1), outputting to self: (u8.2)=less, (u8.3)=equal, (u8.4)=more
    Compare(isize, u8, u8, u8),                                             //compares self and (isize away), outputting to self: (u8.1)=less, (u8.2)=equal, (u8.3)=more
    Not,                                                                    //if self is 0, sets it to 1, else sets it to 0
    
        //multiIO, basically useless
    ReadMany(usize),                                                        //reads into the next (usize) cells in memory
    ReadUntil(u8),                                                          //reads into consecutive cells to the right until it encounters (u8) BEFORE read
    ReadUntilInput(u8),                                                     //reads into consecutive cells to the right until (u8) has been input WHICH IS NOT READ
    PrintMany(usize),                                                       //prints from the next (usize) cells in memory
    PrintUntil(u8),                                                         //prints from consecutive cells to the right until it encounters (u8) WHICH IS NOT PRINTED

        //pure asm x86-64 instructions
    //CompareFlag(isize),                                                     //cmp [rbx], [rbx+(isize)]
}

impl Instruction{
    //some dont need v
    fn from_sbf(c: char, v: isize) -> Result<Self, BfcError>{
        use Instruction::*;
        match c{
            '+' => Ok(Add(v as u8)),
            '-' => Ok(Add((256-v) as u8)),
            '<' => Ok(Move(-v)),
            '>' => Ok(Move(v)),
            '[' => Ok(LoopBegin(v as usize)),
            ']' => Ok(LoopEnd(v as usize)),
            ',' => Ok(Read),
            '.' => Ok(Print),
            '~' => Ok(Debug),
            other => Err(BfcError::InvalidInstruction(other.to_string()))
        }
    }
}

//from sbf
pub fn build_instruction_list(code: &str) -> Result<Vec<Instruction>, Box<dyn Error>>{
    let mut instructions=Vec::new();

    let mut current_op=None;
    let mut count=0isize;

    for c in code.chars().chain(std::iter::once(' ')) {
        if matches!(c, '+' | '-' | '<' | '>' | '[' | ']' | ',' | '.' | '~') || c==' '{
            if let Some(op)=current_op{
                instructions.push(Instruction::from_sbf(op, count.max(1))?);
            }

            current_op=if c==' '{None}else{Some(c)};
            count=0;
        }else{
            count=count.checked_mul(10).and_then(|n| n.checked_add(c.to_digit(10).unwrap() as isize)).ok_or("Instruction count overflow")?;
        }
    }

    recalculate_loops(&mut instructions)?;

    Ok(instructions)
}

fn recalculate_loops(code: &mut Vec<Instruction>) -> Result<(), Box<dyn Error>>{
    let mut loop_stack=Vec::new();

    for ins_idx in 0..code.len(){
        match code[ins_idx]{
            LoopBegin(_) => loop_stack.push(ins_idx),
            LoopEnd(_) =>{
                let begin=loop_stack.pop().ok_or(BfcError::InvalidParenthesesClose)?;
                code[begin]=Instruction::LoopBegin(ins_idx);
                code[ins_idx]=Instruction::LoopEnd(begin);
            },
            _ => (),
        };
    }

    if !loop_stack.is_empty(){
        return Err(Box::new(BfcError::InvalidParenthesesOpen));
    }

    return Ok(());
}
