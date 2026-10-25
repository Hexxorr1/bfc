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
    use crate::core::code_optimizer::straight_optimizer::get_straight_line_changes;
    let mut body_eval=get_straight_line_changes(body)?;

    //must begin and end at the same cell
    if body_eval.1!=0{
        return None;
    }

    //calculate whether the loop will ALWAYS be finite
    let self_change=body_eval.0.remove(&0).unwrap_or(0);
    if self_change%2==0{
        return None;
    }


    let mut transfers=Vec::new();

    const INVERSES: [u8; 128]=crate::utils::get_modular_inverses();
    for (offset, delta) in body_eval.0{
        let multiplier=delta.wrapping_mul(INVERSES[(self_change as usize-1)/2]).wrapping_neg();
        if multiplier!=0{
            transfers.push((offset, multiplier));
        }
    }

    return Some(transfers);
}
