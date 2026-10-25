use crate::core::builder::Instruction;
use crate::core::code_optimizer::constant_propagator::CellState;

fn partially_evaluate_loop(body: &Vec<Instruction>, known_values: &Vec<CellState>, max_iterations: usize) -> Option<Vec<Instruction>>{
    None
}
