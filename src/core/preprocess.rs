use crate::error::BfcError;
use std::error::Error;


//only leaves symbols that matter for the code
pub fn remove_comments(code: &mut String, is_shortened: bool){
    //basic bf chars
    let basic_chars=vec!['+', '-', '<', '>', '[', ']', ',', '.'];

    //additional sbf chars
    let additional_chars=vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '~'];
    let valid_chars=if is_shortened{
        [basic_chars, additional_chars].concat()
    }else{
        basic_chars
    };

    *code=code.chars().filter(|c| valid_chars.contains(&c)).collect::<String>();
}

//only call after remove_comments(code) with NORMAL bf
//NOT PERFECT
//mess, to be optimised
pub fn shorten(code: &mut String) -> Result<(), Box<dyn Error>>{
    let mut shortened_code=String::new();

    let shortenable_chars=vec!['+', '-', '<', '>'];

    //'m', 'i', or ' '
    let mut counter=(' ', 0isize);

    for c in code.chars(){
        if shortenable_chars.contains(&c){
            match c{
                '+' | '-' =>{
                    if counter.0=='m'{
                        shortened_code+=shorten_ins(counter)?.as_str();
                        counter.1=0;
                    }
                    counter=('i', counter.1+if c=='+'{1}else{-1})
                },
                '<' | '>' =>{
                    if counter.0=='i'{
                        shortened_code+=shorten_ins(counter)?.as_str();
                        counter.1=0;
                    }
                    counter=('m', counter.1+if c=='>'{1}else{-1})
                },
                _ => (),
            }
        }else{
            shortened_code+=shorten_ins(counter)?.as_str();

            shortened_code+=c.to_string().as_str();
            counter=(' ', 0);
        }
    }
    shortened_code+=shorten_ins(counter)?.as_str();

    *code=shortened_code;
    Ok(())
}

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
