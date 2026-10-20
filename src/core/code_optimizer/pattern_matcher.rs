use crate::core::builder::Instruction::{self, *};

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

//a single pass over the vector
pub fn optimization_pass(ins: &mut Vec<Instruction>) -> bool{
    let mut result=Vec::with_capacity(ins.len()/2);
    let mut i=0;

    let mut did_change=false;

    while i<ins.len(){
        //pass smaller and smaller chunks of the code through the pattern matching function
        if let Some((consumed, replacement))=optimize(&ins[i..]){
            //yes match
            result.extend(replacement);
            i+=consumed;
            did_change=true;
        }else{
            //no match
            result.push(ins[i].clone());
            i+=1;
        }
    }

    *ins=result;
    return did_change;
}

//commented things are to be implemented/fixed soon :tm:
//the core of the optimization process
optimization_table!(
        //operation conversion

        //removing useless operations
    [LoopBegin(_), LoopEnd(_)] => vec![ExitIfNonZero];
    [Move(n), Move(k)] => vec![Move(*n+*k)];
    [Add(n), Add(k)] => vec![Add(n.wrapping_add(*k))];

        //multiple -> single

        //seekers
    [LoopBegin(_), Move(m), LoopEnd(_)] => vec![SeekClean(0, *m)];
    [LoopBegin(_), Add(n), Move(m), Add(c), LoopEnd(_)] if *c==n.wrapping_neg() => vec![Seek(*n, *m)];
    [Add(a), Seek(n, m), Add(b)] if *a==n.wrapping_neg() && *b==*n => vec![SeekClean(*n, *m)];
    [Seek(n, m)] => vec![Add(*n), SeekClean(*n, *m), Add(n.wrapping_neg())];
);
