use crate::{utils::op_chars, core::builder::Instruction::{LoopBegin, LoopEnd}, utils::BfcError};
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
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
    ExitIfNonZero,                                                          //closes the program if the current cell is not 0
    LinearLoop(Vec<(isize, u8)>),                                           //changes (isize) by (self*u8), consuming self, ending at self

        //pure asm x86-64 instructions
    //CompareFlag(isize),                                                     //cmp [rbx], [rbx+(isize)]
}

impl Instruction{
    //converts sbf to Self, some dont need v
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
    let mut instructions=Vec::with_capacity(code.len()/2);

    let mut current_op=None;
    let mut count=0isize;

    let mut instruction_chars=op_chars().0;
    instruction_chars[b'~' as usize]=true;

    //append ' ' to the end to mark it for the code
    for c in code.chars().chain(std::iter::once(' ')){
        if instruction_chars[c as usize] || c==' '{
            if let Some(op)=current_op{
                //push the PREVIOUS instruction, the reason we need ' ' at the end
                instructions.push(Instruction::from_sbf(op, count.max(1))?);
            }

            //either end or the next char
            current_op=if c==' '{None}else{Some(c)};
            count=0;
        }else{
            //increment the operations count
            count=count.checked_mul(10).and_then(|n| n.checked_add(c.to_digit(10)? as isize)).ok_or("Instruction count overflow")?;
        }
    }

    //since loops were pushed with junk, recalculate
    recalculate_loops(&mut instructions)?;


    Ok(instructions)
}

//recalculate loop indices, also used when generating a new set in optimization
pub fn recalculate_loops(code: &mut Vec<Instruction>) -> Result<(), Box<dyn Error>>{
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
