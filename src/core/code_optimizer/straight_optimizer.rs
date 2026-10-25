use crate::core::builder::Instruction::{self, *};
use std::collections::BTreeMap;

pub fn get_straight_line_changes(ins: &[Instruction]) -> Option<(BTreeMap<isize, u8>, isize)>{
    let mut offset=0isize;
    let mut changes=BTreeMap::<isize, u8>::new();

    for instruction in ins{
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

    Some((changes, offset))
}
