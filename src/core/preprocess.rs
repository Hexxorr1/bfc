use crate::error::BfcError;
use crate::core::op_chars;
use std::error::Error;

//only leaves symbols that matter for the code
pub fn remove_comments(code: &mut String, is_shortened: bool){
    const VALID_CHARS: ([bool; 256], [bool; 256])=op_chars();
    
    let ch=if is_shortened{
        VALID_CHARS.1
    }else{
        VALID_CHARS.0
    };

    code.retain(|c| c.is_ascii() && ch[c as usize]);
}

//only call after remove_comments(code) with valid bf NOT sbf
//NOT PERFECT, thing like ">+<+>+<+" will break it
pub fn shorten(code: &mut String) -> Result<(), Box<dyn Error>>{
    //allocate with initial/5 length to minimize reallocations
    let mut shortened_code=String::with_capacity(code.len()/5);

    //'m', 'i', or ' ' standing for Move, Increment, NotShortenable
    let mut counter=(' ', 0isize);

    for c in code.chars(){
        if matches!(c, '+' | '-' | '<' | '>'){
            //increment the counter (and possibly append the instruction)
            process_shortenable(&mut shortened_code, &mut counter, c)?;
        }else{
            //add previous instruction
            shortened_code+=shorten_ins(counter)?.as_str();

            //add self
            shortened_code.push(c);
            counter=(' ', 0);
        }
    }
    //add any leftover shortenable instruction
    shortened_code+=shorten_ins(counter)?.as_str();

    *code=shortened_code;
    Ok(())
}

//converts the shortenable instructions counter into shortened instructions
pub fn shorten_ins(ins: (char, isize)) -> Result<String, BfcError>{
    if ins.1!=0{
        let mut added_instruction=match (ins.0, ins.1>0){
            ('m', true) => '>',
            ('m', false) => '<',
            ('i', true) => '+',
            ('i', false) => '-',
            (c, _) => return Err(BfcError::InvalidInstruction(c.to_string())),
        }.to_string();

        if ins.1.abs()>1{
            added_instruction+=ins.1.abs().to_string().as_str();
        }
        return Ok(added_instruction);
    }

    return Ok(String::new());
}

//increment the counter (and possibly append the instruction)
fn process_shortenable(code: &mut String, counter: &mut (char, isize), c: char) -> Result<(), BfcError>{
    match c{
        '+' | '-' =>{
            if counter.0=='m'{
                //output previous instruction
                *code+=shorten_ins(*counter)?.as_str();
                counter.1=0;
            }

            //change counter depending on +/-
            *counter=('i', counter.1+if c=='+'{1}else{-1});
        },
        '<' | '>' =>{
            if counter.0=='i'{
                //output previous instruction
                *code+=shorten_ins(*counter)?.as_str();
                counter.1=0;
            }

            //change counter depending on </>
            *counter=('m', counter.1+if c=='>'{1}else{-1});
        },
        _ => (),
    }
    Ok(())
}
