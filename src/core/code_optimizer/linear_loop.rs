use crate::core::builder::Instruction::{self, *};

pub fn optimize_linear_loops(code: &mut Vec<Instruction>) -> bool{
    let mut did_change=false;
    let mut optimized_code=Vec::new();
    let mut idx=0;
    while idx<code.len(){
        match &code[idx]{
            LoopBegin(e) =>{
                let result=recognize_linear_loop_body(&code[idx+1..*e]);
                match result{
                    Some(replacement) =>{
                        idx=e+1;
                        did_change=true;
                        optimized_code.push(LinearLoop(replacement));
                    }
                    None =>{
                        idx+=1;
                        optimized_code.push(LoopBegin(*e));
                    },
                }
            }
            other =>{
                idx+=1;
                optimized_code.push(other.clone());
            },
        }
    }
    *code=optimized_code;
    return did_change;
}

fn recognize_linear_loop_body(body: &[Instruction]) -> Option<Vec<(isize, u8)>>{
    use std::collections::BTreeMap;
    let mut offset=0isize;
    let mut changes=BTreeMap::<isize, u8>::new();

    for instruction in body{
        match instruction{
            Add(n) =>{
                let entry=changes.entry(offset).or_default();
                *entry=entry.wrapping_add(*n);
            }

            Move(n) =>{
                offset=offset.checked_add(*n)?;
            }

            //other than add and move, not supported
            _ => return None,
        }
    }

    //must begin and end at the same cell
    if offset!=0{
        return None;
    }

    //calculate whether the loop will ALWAYS be finite
    let self_change=changes.remove(&0).unwrap_or(0);
    if self_change%2==0{
        return None;
    }


    let mut transfers=Vec::new();

    const INVERSES: [u8; 128]=crate::utils::get_modular_inverses();
    for (offset, delta) in changes{
        let multiplier=delta.wrapping_mul(INVERSES[(self_change as usize-1)/2]).wrapping_neg();
        if multiplier!=0{
            transfers.push((offset, multiplier));
        }
    }

    return Some(transfers);
}
