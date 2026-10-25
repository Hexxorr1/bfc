use std::error::Error;
use std::fmt::Write;

use crate::core::{builder::Instruction, code_generator::CodeGenerator};
use crate::utils::BfcError;

//the start of the generated file
pub fn get_asm_base(tape_size: usize) -> String{
    return format!("default rel

extern putchar
extern getchar
global main

section .bss
    tape: resb {}

section .text
main:
    push r12
    sub rsp, 32
    lea r12, [tape]

", tape_size);
}

//the end of the generated file
pub fn get_asm_closing() -> String{
    return r#"
EXIT_PROGRAM:
    xor eax, eax
    add rsp, 32
    pop r12
    ret
    "#.to_string();
}

pub fn asm_memory_address(offset: isize) -> String{
    use std::cmp::Ordering::*;
    match offset.cmp(&0){
        Less => format!("byte [r12 - {}]", -offset),
        Greater => format!("byte [r12 + {}]", offset),
        Equal => "byte [r12]".to_string(),
    }
}

pub fn asm_move_pointer(offset: isize) -> String{
    return format!("\tadd r12, {}", offset);
}

pub fn to_asm(asm: &mut String, ins: &Instruction, cg: &mut CodeGenerator) -> Result<(), Box<dyn Error>>{
    use crate::core::builder::Instruction::*;
    match ins{
        Nop => (),

        Add(n) => writeln!(asm, "\tadd {}, {}", cg.mem(0), n)?,
        
        Move(n) => cg.tape_offset+=n,

        LoopBegin(_) =>{
            writeln!(asm, "{}", cg.flush_pointer_offset())?;
            writeln!(asm, "\tjmp loop_{}_check", cg.current_ins_idx)?;
            writeln!(asm, "loop_{}:", cg.current_ins_idx)?;
        },

        LoopEnd(bi) =>{
            writeln!(asm, "{}", cg.flush_pointer_offset())?;
            writeln!(asm, "loop_{}_check:", bi)?;
            writeln!(asm, "\tcmp {}, 0", cg.mem(0))?;
            writeln!(asm, "\tjne loop_{}", bi)?;
        },

        Set(n) => writeln!(asm, "\tmov {}, {}", cg.mem(0), n)?,

        Read => writeln!(asm, "\tcall getchar\n\tmov {}, al", cg.mem(0))?,
        
        Print => writeln!(asm, "\tmovzx ecx, {}\n\tcall putchar", cg.mem(0))?,

        Debug => return Err(Box::new(BfcError::ConversionToAsmNotImplemented(ins.clone()))),

        ExitIfNonZero =>{
            writeln!(asm, "\tcmp {}, 0", cg.mem(0))?;
            writeln!(asm, "\tjne EXIT_PROGRAM")?;
        },

        LinearLoop(affected) =>{
            writeln!(asm, "\tmovzx eax, {}", cg.mem(0))?;
            for change in affected{
                if change.1==1{
                    writeln!(asm, "\tadd {}, al", cg.mem(change.0))?;
                }else{
                    writeln!(asm, "\timul edx, eax, {}", change.1)?;
                    writeln!(asm, "\tadd {}, dl", cg.mem(change.0))?;
                }
            }
            writeln!(asm, "\tmov {}, 0", cg.mem(0))?;
        },

        Seek(num, step) =>{
            writeln!(asm, "{}", cg.flush_pointer_offset())?;
            writeln!(asm, "\tjmp loop_seeker_{}_check", cg.seekers_encountered)?;
            writeln!(asm, "loop_seeker_{}:", cg.seekers_encountered)?;
            writeln!(asm, "\tadd r12, {}", step)?;
            writeln!(asm, "loop_seeker_{}_check:", cg.seekers_encountered)?;
            writeln!(asm, "\tcmp {}, {}", cg.mem(0), num)?;
            writeln!(asm, "\tjne loop_seeker_{}", cg.seekers_encountered)?;
            cg.seekers_encountered+=1;
        },

        If(_) =>{
            return Err(Box::new(BfcError::ConversionToAsmNotImplemented(ins.clone())));
        },

        EndIf =>{
            return Err(Box::new(BfcError::ConversionToAsmNotImplemented(ins.clone())));
        }
    };
    Ok(())
}
