use std::error::Error;
use std::fmt::Write;

use crate::core::{builder::Instruction, code_generator::CodeGenerator};
use crate::utils::BfcError;

//the start of the generated file
pub fn get_llvmir_base(tape_size: usize) -> String{
    return format!("@tape = internal global [{} x i8] zeroinitializer

declare i32 @getchar()
declare i32 @putchar(i32)

define i32 @main() {{
entry:
  %pos = alloca i64
  store i64 0, ptr %pos
", tape_size);
}

//the end of the generated file
pub fn get_llvmir_closing() -> String{
    return r#"
    ret i32 0
}
    "#.to_string();
}

pub fn llvmir_memory_address(offset: isize) -> String{
    use std::cmp::Ordering::*;
    todo!();
    match offset.cmp(&0){
        Less => format!("{}", -offset),
        Greater => format!("{}", offset),
        Equal => "".to_string(),
    }
}

pub fn llvmir_move_pointer(offset: isize) -> String{
    todo!();
    String::new()
}

impl CodeGenerator {
    fn id(&mut self) -> usize{
        let id=self.next_id;
        self.next_id+=1;
        id
    }

    fn reg(&mut self) -> String{
        format!("%r{}", self.id())
    }

    fn label(&mut self, prefix: &str) -> String{
        format!("{}_{}", prefix, self.id())
    }
}

use std::fmt::Write as _;

fn cell_ptr(ll: &mut String,cg: &mut CodeGenerator,offset: isize,) -> Result<String, std::fmt::Error>{
    let pos=cg.reg();
    let idx=cg.reg();
    let ptr=cg.reg();

    writeln!(ll, "\t{} = load i64, ptr %pos", pos)?;
    writeln!(ll, "\t{} = add i64 {}, {}", idx, pos, offset)?;
    writeln!(ll, "\t{} = getelementptr [30000 x i8], ptr @tape, i64 0, i64 {}", ptr, idx)?;
    Ok(ptr)
}

fn load_cell(ll: &mut String, cg: &mut CodeGenerator, offset: isize,) -> Result<(String, String), std::fmt::Error>{
    let ptr=cell_ptr(ll, cg, offset)?;
    let value=cg.reg();
    writeln!(ll, "\t{} = load i8, ptr {}", value, ptr)?;
    Ok((ptr, value))
}

pub fn to_llvmir(ll: &mut String, ins: &Instruction, cg: &mut CodeGenerator) -> Result<(), Box<dyn Error>>{
    use crate::core::builder::Instruction::*;

    match ins{
        Add(n) =>{
            let (ptr, val)=load_cell(ll, cg, 0)?;
            let result=cg.reg();

            writeln!(ll, "\t{result} = add i8 {val}, {n}")?;
            writeln!(ll, "\tstore i8 {result}, ptr {ptr}")?;
        }

        Move(n) =>{
            let pos=cg.reg();
            let result=cg.reg();

            writeln!(ll, "\t{pos} = load i64, ptr %pos")?;
            writeln!(ll, "\t{result} = add i64 {pos}, {n}")?;
            writeln!(ll, "\tstore i64 {result}, ptr %pos")?;
        }

        Set(n) =>{
            let ptr=cell_ptr(ll, cg, 0)?;
            writeln!(ll, "\tstore i8 {n}, ptr {ptr}")?;
        }

        Read =>{
            let ptr=cell_ptr(ll, cg, 0)?;
            let input=cg.reg();
            let byte=cg.reg();

            writeln!(ll, "\t{input} = call i32 @getchar()")?;
            writeln!(ll, "\t{byte} = trunc i32 {input} to i8")?;
            writeln!(ll, "\tstore i8 {byte}, ptr {ptr}")?;
        }

        Print =>{
            let (_, val)=load_cell(ll, cg, 0)?;
            let extended=cg.reg();

            writeln!(ll, "\t{extended} = zext i8 {val} to i32")?;
            writeln!(ll, "\tcall i32 @putchar(i32 {extended})")?;
        }

        LoopBegin(_) =>{
            let cond=cg.label("loop_cond");
            let body=cg.label("loop_body");
            let end=cg.label("loop_end");

            cg.loops.push((cond.clone(), end.clone()));

            writeln!(ll, "\tbr label %{cond}")?;
            writeln!(ll, "{cond}:")?;

            let (_, val)=load_cell(ll, cg, 0)?;
            let cmp = cg.reg();

            writeln!(ll, "\t{cmp} = icmp ne i8 {val}, 0")?;
            writeln!(ll, "\tbr i1 {cmp}, label %{body}, label %{end}")?;
            writeln!(ll, "{body}:")?;
        }

        LoopEnd(_) =>{
            let (cond, end)=cg.loops.pop().ok_or("LoopEnd without LoopBegin")?;

            writeln!(ll, "\tbr label %{cond}")?;
            writeln!(ll, "{end}:")?;
        }

        Seek(target, step) =>{
            let cond=cg.label("seek_cond");
            let advance=cg.label("seek_advance");
            let end=cg.label("seek_end");

            writeln!(ll, "\tbr label %{cond}")?;
            writeln!(ll, "{cond}:")?;

            let (_, val)=load_cell(ll, cg, 0)?;
            let cmp=cg.reg();

            writeln!(ll, "\t{cmp} = icmp eq i8 {val}, {target}")?;
            writeln!(ll, "\tbr i1 {cmp}, label %{end}, label %{advance}")?;
            writeln!(ll, "{advance}:")?;

            let pos=cg.reg();
            let next=cg.reg();

            writeln!(ll, "\t{pos} = load i64, ptr %pos")?;
            writeln!(ll, "\t{next} = add i64 {pos}, {step}")?;
            writeln!(ll, "\tstore i64 {next}, ptr %pos")?;
            writeln!(ll, "\tbr label %{cond}")?;

            writeln!(ll, "{end}:")?;
        }

        LinearLoop(changes) =>{
            // Read original value before modifying anything.
            let (self_ptr, original)=load_cell(ll, cg, 0)?;

            for &(offset, multiplier) in changes{
                let (ptr, old)=load_cell(ll, cg, offset)?;

                let product=cg.reg();
                let result=cg.reg();

                writeln!(ll, "\t{product} = mul i8 {original}, {multiplier}")?;
                writeln!(ll, "\t{result} = add i8 {old}, {product}")?;
                writeln!(ll, "  store i8 {result}, ptr {ptr}")?;
            }

            writeln!(ll, "\tstore i8 0, ptr {self_ptr}")?;
        }

        ExitIfNonZero =>{
            let (_, val)=load_cell(ll, cg, 0)?;
            let cmp=cg.reg();

            let exit=cg.label("exit");
            let cont=cg.label("continue");

            writeln!(ll, "\t{cmp} = icmp ne i8 {val}, 0")?;
            writeln!(ll, "\tbr i1 {cmp}, label %{exit}, label %{cont}")?;
            writeln!(ll, "{exit}:")?;
            writeln!(ll, "\tret i32 0")?;
            writeln!(ll, "{cont}:")?;
        }

        If(_) =>{
            let body=cg.label("if_body");
            let end=cg.label("if_end");

            cg.ifs.push(end.clone());

            let (_, val)=load_cell(ll, cg, 0)?;
            let cmp=cg.reg();

            writeln!(ll, "\t{cmp} = icmp ne i8 {val}, 0")?;
            writeln!(ll, "\tbr i1 {cmp}, label %{body}, label %{end}")?;
            writeln!(ll, "{body}:")?;
        }

        EndIf =>{
            let end=cg.ifs.pop().ok_or(BfcError::InvalidParenthesesClose)?;

            writeln!(ll, "\tbr label %{end}")?;
            writeln!(ll, "{end}:")?;
        }

        Debug =>{
            writeln!(ll, "  ; Debug")?;
        }

        Nop => {}
    }

    Ok(())
}
