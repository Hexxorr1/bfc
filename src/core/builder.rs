use crate::{utils::op_chars, utils::BfcError};
use std::{error::Error, io::SeekFrom::End};

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

    Nop,

    Set(u8),                                                                //sets the current cell to (u8)
    Seek(u8, isize),                                                   //goes (isize) at a time until it finds (u8)
    ExitIfNonZero,                                                          //closes the program if the current cell is not 0
    LinearLoop(Vec<(isize, u8)>),                                           //changes (isize) by (self*u8), consuming self, ending at self

    If(usize),                                                              //if the current cell is 0 jump to (usize)
    EndIf,                                                                  //only used when recalculating the if statements
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
    recalculate_scopes(&mut instructions)?;


    Ok(instructions)
}

//recalculate loop indices, also used when generating a new set in optimization
pub fn recalculate_scopes(code: &mut Vec<Instruction>) -> Result<(), Box<dyn Error>>{
    use crate::core::builder::Instruction::*;
    let mut scope_stack=Vec::new();

    for ins_idx in 0..code.len(){
        match code[ins_idx]{
            LoopBegin(_) | If(_) => scope_stack.push(ins_idx),
            LoopEnd(_) =>{
                let begin=scope_stack.pop().ok_or(BfcError::InvalidParenthesesClose)?;
                code[begin]=LoopBegin(ins_idx);
                code[ins_idx]=LoopEnd(begin);
            },
            EndIf =>{
                let begin=scope_stack.pop().ok_or(BfcError::InvalidParenthesesClose)?;
                code[begin]=If(ins_idx);
            }
            _ => (),
        };
    }

    if !scope_stack.is_empty(){
        return Err(Box::new(BfcError::InvalidParenthesesOpen));
    }

    return Ok(());
}
