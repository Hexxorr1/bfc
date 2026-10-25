use std::error::Error;
use constant_propagator::ConstantPropagator;
use linear_loop::optimize_linear_loops;
use pattern_matcher::optimization_pass;
use crate::{core::builder::{Instruction, recalculate_scopes}, utils::BfcError};

mod constant_propagator;
mod pattern_matcher;
mod linear_loop;
mod loop_evaluator;
mod straight_optimizer;

//perform several cycles of passes of code optimization
pub fn optimize_code(ins: &mut Vec<Instruction>, tape_size: usize, max_passes: u32) -> Result<(u32, u32), Box<dyn Error>>{
    //working code
    let mut current_iteration_code=ins;
    let mut iterations_done=0u32;
    let mut iterations_succesful=0u32;

    let cycle=get_optimization_cycle();
    //max_passes is a hard limit
    for _ in 0..max_passes{
        let mut did_change=vec![false; cycle.len()];
        for step_idx in 0..cycle.len(){
            iterations_done+=1;

            let changed=cycle[step_idx].apply_technique(current_iteration_code, tape_size)?;

            iterations_succesful+=changed as u32;
            if changed{
                //the code got optimized and the loops got messed up, maybe optimize this?
                recalculate_scopes(&mut current_iteration_code)?;
                did_change[step_idx]=true;
            }
        }
        if !did_change.contains(&true){
            break;
        }
    }
    //return the optimized code, alongside with passes done
    return Ok((iterations_done, iterations_succesful));
}

fn get_optimization_cycle() -> Vec<OptimizationTechnique>{
    use OptimizationTechnique::*;
    let mut layout=Vec::new();

    layout.push(LinearLoops);
    layout.push(PatternMatcher);
    layout.push(ConstantPropagation);

    return layout;
}

enum OptimizationTechnique{
    LinearLoops,
    PatternMatcher,
    ConstantPropagation,
}

impl OptimizationTechnique{
    fn apply_technique(&self, instructions: &mut Vec<Instruction>, tape_size: usize) -> Result<bool, BfcError>{
        use OptimizationTechnique::*;
        match self{
            LinearLoops => Ok(optimize_linear_loops(instructions)),
            PatternMatcher => Ok(optimization_pass(instructions)),
            ConstantPropagation =>{
                let mut propagator=ConstantPropagator::new_populate_zeros(tape_size);
                return propagator.propagate_constants(instructions);
            },
        }
    }
}
