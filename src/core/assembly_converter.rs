use crate::{core::builder::Instruction, error::BfcError};
use std::fmt::Write;
use std::error::Error;

pub struct AssemblyGenerator{
    tape_offset: isize,
    current_ins_idx: usize,
    seekers_encountered: usize,
}

impl AssemblyGenerator{
    pub fn new() -> Self{
        return Self{tape_offset: 0, current_ins_idx: 0, seekers_encountered: 0};
    }

    pub fn convert_to_assembly(&mut self, instruction_list: &Vec<Instruction>) -> Result<String, Box<dyn Error>>{
        let mut asm=String::new();

        asm.push_str(&get_asm_base());

        for (idx, ins) in instruction_list.iter().enumerate(){
            self.current_ins_idx=idx;
            ins.to_asm(&mut asm, self)?;
        }

        asm.push_str(&get_asm_closing());

        Ok(asm)
    }

    fn mem(&self, offset: isize) -> String{
        use std::cmp::Ordering;
        let offset=self.tape_offset+offset;
        match offset.cmp(&0){
            Ordering::Less => format!("byte [r12 - {}]", -offset),
            Ordering::Greater => format!("byte [r12 + {}]", offset),
            Ordering::Equal => "byte [r12]".to_string(),
        }
    }

    fn flush_pointer_offset(&mut self) -> String{
        if self.tape_offset==0{
            return String::new();
        }else{
            let tmp=format!("\tadd r12, {}", self.tape_offset);
            self.tape_offset=0;
            return tmp;
        }
    }
}

impl Instruction{
    fn to_asm(&self, asm: &mut String, cg: &mut AssemblyGenerator) -> Result<(), Box<dyn Error>>{
        use crate::core::builder::Instruction::*;
        //writeln!(asm, "")?;
        match self{
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
                writeln!(asm, "\tcmp byte {}, 0", cg.mem(0))?;
                writeln!(asm, "\tjne loop_{}\n", bi)?;
            },

            Set(n) => writeln!(asm, "\tmov {}, {}", cg.mem(0), n)?,

            Read => writeln!(asm, "\tcall getchar\n\tmov {}, al", cg.mem(0))?,
            
            Print => writeln!(asm, "\tmovzx ecx, {}\n\tcall putchar", cg.mem(0))?,

            AddSelfToAnother(n) =>{
                writeln!(asm, "\tmov al, {}", cg.mem(0))?;
                writeln!(asm, "\tadd {}, al", cg.mem(*n))?;
                writeln!(asm, "\tmov {}, 0", cg.mem(0))?;
            },

            Seek(num, step) =>{
                writeln!(asm, "{}", cg.flush_pointer_offset())?;
                writeln!(asm, "\tcmp {}, 0", cg.mem(0))?;
                writeln!(asm, "\tje done_seeker_{}", cg.seekers_encountered)?;
                writeln!(asm, "\tadd {}, {}", cg.mem(0), num)?;
                writeln!(asm, "\tadd r12, {}", step)?;
                SeekClean(*num, *step).to_asm(asm, cg)?;
                writeln!(asm, "\tmov {}, 0", cg.mem(0))?;
                writeln!(asm, "done_seeker_{}:", cg.seekers_encountered-1)?;
            },

            SeekClean(num, step) =>{
                writeln!(asm, "{}", cg.flush_pointer_offset())?;
                writeln!(asm, "\tjmp loop_seeker_{}_check", cg.seekers_encountered)?;
                writeln!(asm, "loop_seeker_{}:", cg.seekers_encountered)?;
                writeln!(asm, "\tadd r12, {}", step)?;
                writeln!(asm, "loop_seeker_{}_check:", cg.seekers_encountered)?;
                writeln!(asm, "\tcmp {}, {}", cg.mem(0), num)?;
                writeln!(asm, "\tjne loop_seeker_{}\n", cg.seekers_encountered)?;
                cg.seekers_encountered+=1;
            },

            unim => return Err(Box::new(BfcError::ConversionToAsmNotImplemented(*unim))),
        };
        Ok(())
    }
}

//eww
fn get_asm_base() -> String{
    return r#"
default rel

extern putchar
extern getchar
global main

section .bss
    tape: resb 30000

section .text
main:
    push r12
    sub rsp, 32
    lea r12, [tape]

"#.to_string();
}

fn get_asm_closing() -> String{
    return r#"
    xor eax, eax
    add rsp, 32
    pop r12
    ret
    "#.to_string();
}
