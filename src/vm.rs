use std::collections::HashSet;

use crate::compiler::{Instruction, Operation};

pub struct Vm;

impl Vm {
    pub fn eval(code: &[Instruction], tags: &HashSet<String>) -> bool {
        let mut stack: Vec<bool> = Vec::with_capacity(8);
        for instr in code {
            match instr {
                Instruction::PUSH(tag_name) => {
                    stack.push(tags.contains(tag_name));
                }
                Instruction::APPLY(Operation::NOT) => {
                    let val = stack.pop().unwrap_or(false);
                    stack.push(!val);
                }
                Instruction::APPLY(Operation::AND) => {
                    let a = stack.pop().unwrap_or(false);
                    let b = stack.pop().unwrap_or(false);
                    stack.push(a && b);
                }
                Instruction::APPLY(Operation::OR) => {
                    let a = stack.pop().unwrap_or(false);
                    let b = stack.pop().unwrap_or(false);
                    stack.push(a || b);
                }
            }
        }

        stack.pop().unwrap_or(false)
    }
}
