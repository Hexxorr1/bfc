use crate::core::builder::Instruction;
use std::error::Error;
use GeneratedCodeType::*;

mod asmx86_64;
mod llvmir;

pub enum GeneratedCodeType{
    Asmx86_64,
    //Aarch64,
    LLVMir,
}

//the converter, holds some data used in asm generation and its optimization
//tape refers to the memory, seekers have custom labels
pub struct CodeGenerator{
    output_language: GeneratedCodeType,
    tape_size: usize,

    tape_offset: isize,
    current_ins_idx: usize,
    seekers_encountered: usize,


    next_id: usize,
    loops: Vec<(String, String)>,
    ifs: Vec<String>,
}

impl CodeGenerator{
    //creates an instance of a generator
    pub fn new(output_language: GeneratedCodeType, tape_size: usize) -> Self{
        return Self{output_language: output_language, tape_size: tape_size, tape_offset: 0, current_ins_idx: 0, seekers_encountered: 0,
        next_id: 0, loops: Vec::new(), ifs: Vec::new()};
    }

    //converts an instruction list to a valid asm x86-64 instruction list
    pub fn generate_target(&mut self, instruction_list: &Vec<Instruction>) -> Result<String, Box<dyn Error>>{
        let mut generated=String::new();

        //setup the file
        generated.push_str(&get_generated_base(self));

        //translate every instruction
        for ins in instruction_list{
            //the function updates the file automatically
            ins.to_output(&mut generated, self)?;
            self.current_ins_idx+=1;
        }

        //close the file
        generated.push_str(&get_generated_closing(self));

        return Ok(generated);
    }

    //get the current memory including tape_offset and user-given offset
    fn mem(&self, offset: isize) -> String{
        let offset=self.tape_offset+offset;
        match self.output_language{
            Asmx86_64 => asmx86_64::asm_memory_address(offset),
            LLVMir => llvmir::llvmir_memory_address(offset),
        }
    }

    //flushes tape_offset, while generating the asm instruction for that
    fn flush_pointer_offset(&mut self) -> String{
        if self.tape_offset==0{
            return String::new();
        }else{
            let tmp=match self.output_language{
                Asmx86_64 => asmx86_64::asm_move_pointer(self.tape_offset),
                LLVMir => llvmir::llvmir_move_pointer(self.tape_offset), 
            };
            self.tape_offset=0;
            return tmp;
        }
    }
}

fn get_generated_base(cg: &CodeGenerator) -> String{
    match cg.output_language{
        Asmx86_64 => asmx86_64::get_asm_base(cg.tape_size),
        LLVMir => llvmir::get_llvmir_base(cg.tape_size),
    }
}

fn get_generated_closing(cg: &CodeGenerator) -> String{
    match cg.output_language{
        Asmx86_64 => asmx86_64::get_asm_closing(),
        LLVMir => llvmir::get_llvmir_closing(),
    }
}

impl Instruction{
    //translates the instruction into its target counterpart
    fn to_output(&self, code: &mut String, cg: &mut CodeGenerator) -> Result<(), Box<dyn Error>>{
        match cg.output_language{
            Asmx86_64 => asmx86_64::to_asm(code, self, cg),
            LLVMir => llvmir::to_llvmir(code, self, cg),
        }?;
        Ok(())
    }
}
