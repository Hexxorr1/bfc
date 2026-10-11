use crate::core::builder::recalculate_loops;
use std::error::Error;

use super::Instruction::{self, *};

//unholy macro
macro_rules! optimization_table{
    ($([$($pattern:pat),+$(,)?]
        $(if $condition:expr)?
        => $result:expr);*$(;)?) =>{

        fn optimize(instructions: &[Instruction])
            -> Option<(usize, Vec<Instruction>)>
        {
            $(
                {
                    let pattern_len=<[()]>::len(&[$({let _=stringify!($pattern);()}),+]);

                    if instructions.len()>=pattern_len{
                        match &instructions[..pattern_len]{
                            [$($pattern),+]
                            $(if $condition)?
                            => return Some((pattern_len, $result)),

                            _ => {}
                        }
                    }
                }
            )*
            None
        }
    };
}

//find patterns in code and replace them with custom operations
pub fn optimize_code(ins: &Vec<Instruction>, max_passes: u32) -> Result<(Vec<Instruction>, u32), Box<dyn Error>>{
    let mut current_iteration_code=ins.clone();
    let mut i=max_passes;

    for iter in 0..max_passes{
        let before_len=current_iteration_code.len();
        current_iteration_code=optimization_pass(&current_iteration_code);
        if before_len==current_iteration_code.len(){
            i=iter;
            break;
        }else{
            recalculate_loops(&mut current_iteration_code)?;
        }
    }

    return Ok((current_iteration_code.clone(), i));
}

fn optimization_pass(ins: &Vec<Instruction>) -> Vec<Instruction>{
    let mut result = Vec::new();
    let mut i = 0;

    while i<ins.len() {
        if let Some((consumed, replacement))=optimize(&ins[i..]){
            result.extend(replacement);
            i+=consumed;
        }else{
            result.push(ins[i]);
            i+=1;
        }
    }

    return result;
}

//commented things are to be implemented soon :tm:
optimization_table!(
        //operation conversion
    [LoopBegin(_), Add(n), LoopEnd(_)] if *n==1 || *n==255 => vec![Set(0)];
    [Set(0), Add(n)] => vec![Set(*n)];
    [LoopBegin(_), Add(n), Move(m), Add(c), LoopEnd(_)] if *c==n.wrapping_neg() => vec![Seek(*n, *m)];
    [Add(a), Seek(n, m), Add(b)] if *a==n.wrapping_neg() && *b==*n => vec![SeekClean(*n, *m)];
    [LoopBegin(_), Move(m), LoopEnd(_)] => vec![SeekClean(0, *m)];
    [LoopBegin(_), Add(a), Move(n), Add(b), Move(d), LoopEnd(_)] if *a==b.wrapping_neg() && *n==d.wrapping_neg() => vec![AddSelfToAnother(*n)];
    //[LoopBegin(_), Seek(m1, s1), Add(d), Move(o1), Add(i), Seek(m2, s2), Add(s), Move(o2), LoopEnd(_)] if (???) => vec![AddSelfToAnotherByMarks(*m1, *s1, *o1, *m2, *s2, *o2)];

        //removing useless operations
    [LoopBegin(_), LoopEnd(_)] => vec![];
    [a, b] if a.only_modifies_self() && b.deterministic_self_from_instruction() => vec![*b];
);

impl Instruction{
    fn only_modifies_self(&self) -> bool{
        use std::mem::discriminant as d;
        let check_vec=vec![d(&Add(0)), d(&Set(0)), d(&Not), d(&Compare(0, 0, 0, 0)), d(&CompareValue(0, 0, 0, 0))];
        return check_vec.contains(&d(self));
    }

    fn deterministic_self_from_instruction(&self) -> bool{
        use std::mem::discriminant as d;
        let check_vec=vec![d(&Set(0))];
        return check_vec.contains(&d(self));
    }
}
