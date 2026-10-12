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
pub fn optimize_code(ins: &mut Vec<Instruction>, max_passes: u32) -> Result<u32, Box<dyn Error>>{
    //working code
    let mut current_iteration_code=ins;
    let mut i=max_passes;

    //max_passes is a hard limit
    for iter in 0..max_passes{
        let before_len=current_iteration_code.len();
        optimization_pass(&mut current_iteration_code);
        if before_len==current_iteration_code.len(){
            //we couldnt optimize any further
            i=iter;
            break;
        }else{
            //the code got optimized and the loops got messed up, maybe optimize this?
            recalculate_loops(&mut current_iteration_code)?;
        }
    }

    //return the optimized code, alongside with passes done
    return Ok(i);
}

//a single pass over the vector
fn optimization_pass(ins: &mut Vec<Instruction>){
    let mut result=Vec::with_capacity(ins.len()/2);
    let mut i=0;

    while i<ins.len(){
        //pass smaller and smaller chunks of the code through the pattern matching function
        if let Some((consumed, replacement))=optimize(&ins[i..]){
            //yes match
            result.extend(replacement);
            i+=consumed;
        }else{
            //no match
            result.push(ins[i].clone());
            i+=1;
        }
    }

    *ins=result;
}

//commented things are to be implemented/fixed soon :tm:
//the core of the optimization process
optimization_table!(
        //operation conversion
    [LoopBegin(_), Add(n), LoopEnd(_)] if *n==1 || *n==255 => vec![Set(0)];
    [Set(0), Add(n)] => vec![Set(*n)];

        //removing useless operations
    [LoopBegin(_), LoopEnd(_)] => vec![ExitIfNonZero];
    [a] if a.is_pointless_operation() => vec![];
    [a, b] if a.only_modifies_self() && b.deterministic_self_from_instruction() => vec![b.clone()];

        //multiple -> single

        //seekers
    [LoopBegin(_), Move(m), LoopEnd(_)] => vec![SeekClean(0, *m)];
    [LoopBegin(_), Add(n), Move(m), Add(c), LoopEnd(_)] if *c==n.wrapping_neg() => vec![Seek(*n, *m)];
    [Add(a), Seek(n, m), Add(b)] if *a==n.wrapping_neg() && *b==*n => vec![SeekClean(*n, *m)];
    [Add(a), Seek(n, m), Add(b)] => vec![Add(a.wrapping_add(*n)), SeekClean(*n, *m), Add(b.wrapping_sub(*n))];

        //self to another
    //ONLY SUPPORTS 2 DUE TO COMPILETIME MACRO
    [LoopBegin(_), Add(a), Move(n), Add(b), Move(d), LoopEnd(_)] if *a==b.wrapping_neg() && *n==d.wrapping_neg() => vec![AddSelfToMultiple(vec![*n])];
);

//some generics for the optimization function
impl Instruction{
    fn only_modifies_self(&self) -> bool{
        use std::mem::{discriminant as d, Discriminant};
        const SATISFIES: &[Discriminant<(Instruction)>]=&[d(&Add(0)), d(&Set(0)), d(&Not)];
        return SATISFIES.contains(&d(self));
    }

    fn deterministic_self_from_instruction(&self) -> bool{
        use std::mem::{discriminant as d, Discriminant};
        const SATISFIES: &[Discriminant<(Instruction)>]=&[d(&Set(0))];
        return SATISFIES.contains(&d(self));
    }

    fn is_pointless_operation(&self) -> bool{
        match self{
            Add(n) => *n==0,
            Move(n) => *n==0,
            Seek(_, n) => *n==0,
            SeekClean(_, n) => *n==0,
            AddSelfToMultiple(v) => v.len()==0,
            _ => false,
        }
    }
}
