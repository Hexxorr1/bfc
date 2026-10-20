use crate::{core::builder::Instruction::{self, *}, utils::BfcError};

#[derive(Clone)]
struct CellValue{
    known_value: u8,
    generation: u64,
}

#[derive(Clone)]
struct CellWrite{
    removable_index: usize,
    generation: u64,
}

#[derive(Clone)]
struct CellState{
    value: CellValue,
    last_write: Option<CellWrite>,
}

pub struct ConstantPropagator{
    cell_states: Vec<CellState>,
    value_generation: u64,
    write_generation: u64,
    pointer: usize,
    tape_length: usize,
}

fn emit_ins(optimized_instructions: &mut Vec<Instruction>, dead_writes: &mut Vec<bool>, ins: Instruction){
    optimized_instructions.push(ins);
    dead_writes.push(false);
}

impl ConstantPropagator{
    pub fn new_populate_zeros(tape_length: usize) -> Self{
        let initial_cell=CellState{value: CellValue{known_value: 0, generation: 0}, last_write: None};

        //2n-1 in order to support unknown pointer movements
        let initial_tape=vec![initial_cell; 2*tape_length-1];
        return Self{cell_states: initial_tape, value_generation: 0, write_generation: 0, pointer: tape_length-1, tape_length: tape_length};
    }

    fn invalidate_pointer_position(&mut self) -> Result<(), BfcError>{
        self.invalidate_all_values()?;
        self.invalidate_all_writes()?;
        //exactly the middle of the 2n-1 tape
        self.set_pointer((self.tape_length-1) as isize);
        Ok(())
    }

    fn invalidate_all_values(&mut self) -> Result<(), BfcError>{
        self.value_generation=self.value_generation.checked_add(1).ok_or(BfcError::GenerationTooLarge)?;
        Ok(())
    }

    fn invalidate_all_writes(&mut self) -> Result<(), BfcError>{
        self.write_generation=self.write_generation.checked_add(1).ok_or(BfcError::GenerationTooLarge)?;
        Ok(())
    }

    fn invalidate_current_value(&mut self) -> Result<(), BfcError>{
        let current=self.cell_states.get_mut(self.pointer).ok_or(BfcError::OutOfTapeBounds)?;
        current.value.generation=self.value_generation.wrapping_sub(1);
        Ok(())
    }

    //mark last write as deleted and add a new one
    fn register_new_write(&mut self, index: usize, dead_writes: &mut [bool]){
        self.kill_last_write(dead_writes);
        self.cell_states[self.pointer].last_write=Some(CellWrite{removable_index: index, generation: self.write_generation});
    }

    fn kill_last_write(&mut self, dead_writes: &mut [bool]){
        if let Some(lw)=&self.cell_states[self.pointer].last_write{
            if lw.generation==self.write_generation{
                dead_writes[lw.removable_index]=true;
            }
        }
    }

    fn preserve_last_write(&mut self) -> Result<(), BfcError>{
        let current=self.cell_states.get_mut(self.pointer).ok_or(BfcError::OutOfTapeBounds)?;
        current.last_write=None;
        Ok(())
    }
    
    fn get_cell_value(&mut self) -> Result<Option<u8>, BfcError>{
        let current=self.cell_states.get_mut(self.pointer).ok_or(BfcError::OutOfTapeBounds)?;
        return Ok((self.value_generation==current.value.generation).then_some(current.value.known_value));
    }

    fn set_pointer(&mut self, value: isize){
        self.pointer=value as usize;
    }

    fn move_pointer(&mut self, change: isize){
        self.set_pointer(self.pointer as isize+change);
    }

    fn set(&mut self, value: u8) -> Result<(), BfcError>{
        let current=self.cell_states.get_mut(self.pointer).ok_or(BfcError::OutOfTapeBounds)?;
        current.value.known_value=value;
        current.value.generation=self.value_generation;
        Ok(())
    }

    fn add(&mut self, value: u8) -> Result<Option<u8>, BfcError>{
        let current=self.cell_states.get_mut(self.pointer).ok_or(BfcError::OutOfTapeBounds)?;
        if current.value.generation==self.value_generation{
            current.value.known_value=current.value.known_value.wrapping_add(value);
            return Ok(Some(current.value.known_value));
        }else{
            return Ok(None);
        }
    }

    pub fn propagate_constants(&mut self, instructions: &mut Vec<Instruction>) -> Result<bool, BfcError>{
        let mut optimized_instructions=Vec::with_capacity(instructions.len());
        let mut dead_writes=vec![];
        let mut did_change=false;

        let mut ins_index=0;
        while ins_index<instructions.len(){
            let ins=instructions[ins_index].clone();
            match ins{
                Add(n) =>{
                    if let Some(new)=self.add(n)?{
                        did_change=true;
                        self.register_new_write(optimized_instructions.len(), &mut dead_writes);
                        emit_ins(&mut optimized_instructions, &mut dead_writes, Set(new));
                    }else{
                        self.preserve_last_write()?;
                        emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                    }
                },

                Set(n) =>{
                    let current=self.get_cell_value()?;
                    if let Some(v)=current && v==n{
                        did_change=true;
                    }else{
                        self.set(n)?;
                        self.register_new_write(optimized_instructions.len(), &mut dead_writes);
                        emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                    }
                },

                Move(n) =>{
                    self.move_pointer(n);
                    emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                },

                Read =>{
                    self.invalidate_current_value()?;
                    self.kill_last_write(&mut dead_writes);
                    self.preserve_last_write()?;
                    emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                },

                Print =>{
                    self.preserve_last_write()?;
                    emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                },

                Debug =>{
                    //INCORRECT
                    self.preserve_last_write()?;
                    emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                },

                LoopBegin(end) =>{
                    let current=self.get_cell_value()?;
                    match current{
                        Some(0) =>{
                            did_change=true;

                            //delete the loop
                            ins_index=end;
                        },
                        _ =>{
                            self.invalidate_all_values()?;
                            self.invalidate_all_writes()?;
                            emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                        }
                    }
                },

                LoopEnd(begin) =>{
                    let exit_cell=self.get_cell_value()?;
                    match exit_cell{
                        //possible optimization, replace with a single check
                        //Some(0) =>{
                        //    dead_writes[begin]=true;
                        //},
                        Some(_) | None =>{
                            self.invalidate_all_values()?;
                            self.invalidate_all_writes()?;
                            self.set(0)?;
                            emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                        }
                    }
                },

                LinearLoop(v) =>{
                    let current=self.get_cell_value()?;
                    match current{
                        Some(0) =>{
                            //delete
                            did_change=true;
                        },
                        Some(start_value) =>{
                            let base_offset=self.pointer as isize;
                            did_change=true;

                            //unroll the loop
                            for target in v{
                                self.set_pointer(base_offset+target.0);
                                let change=start_value.wrapping_mul(target.1);
                                self.add(change)?;

                                let target_val=self.get_cell_value()?;
                                emit_ins(&mut optimized_instructions, &mut dead_writes, Move(target.0));
                                if let Some(val)=target_val{
                                    emit_ins(&mut optimized_instructions, &mut dead_writes, Set(val));
                                    self.register_new_write(optimized_instructions.len()-1, &mut dead_writes);
                                }else{
                                    emit_ins(&mut optimized_instructions, &mut dead_writes, Add(change));
                                    self.preserve_last_write()?;
                                }
                                emit_ins(&mut optimized_instructions, &mut dead_writes, Move(-target.0));
                            }

                            self.set_pointer(base_offset);
                            self.set(0)?;
                            self.register_new_write(optimized_instructions.len(), &mut dead_writes);
                            emit_ins(&mut optimized_instructions, &mut dead_writes, Set(0));
                        },
                        None =>{
                            let base_offset=self.pointer as isize;
                            for &change in v.iter(){
                                self.set_pointer(base_offset+change.0);
                                self.invalidate_current_value()?;
                                self.preserve_last_write()?;
                            }
                            self.set_pointer(base_offset);
                            self.set(0)?;
                            self.preserve_last_write()?;
                            if v.is_empty(){
                                emit_ins(&mut optimized_instructions, &mut dead_writes, Set(0));
                            }else{
                                emit_ins(&mut optimized_instructions, &mut dead_writes, LinearLoop(v));
                            }
                        },
                    }
                },

                ExitIfNonZero =>{
                    let current=self.get_cell_value()?;
                    if let Some(n)=current{
                        did_change=true;
                        if n!=0{
                            //terminate, but keep loop ends
                            //TODO
                            panic!("found infinite loop");
                        }
                    }else{
                        emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                    }
                    self.preserve_last_write()?;
                },

                SeekClean(target_val, _) =>{
                    let current=self.get_cell_value()?;
                    if let Some(v)=current && v==target_val{
                        did_change=true;
                    }else{
                        self.invalidate_pointer_position()?;
                        self.set(target_val)?;
                        emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                    }

                },

                Seek(_, _) =>{
                    let current=self.get_cell_value()?;
                    if let Some(v)=current && v==0{
                        did_change=true;
                    }else{
                        self.invalidate_pointer_position()?;
                        self.set(0)?;
                        emit_ins(&mut optimized_instructions, &mut dead_writes, ins);
                    }

                },
            }
            ins_index+=1;
        }

        let mut final_instructions=Vec::with_capacity(optimized_instructions.len());
        for index in 0..optimized_instructions.len(){
            if !dead_writes[index]{
                final_instructions.push(optimized_instructions[index].clone());
            }else{
                did_change=true;
            }
        }

        *instructions=final_instructions;
        return Ok(did_change);
    }
}
