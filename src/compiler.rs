use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    OR,
    AND,
    NOT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    PUSH(String),
    APPLY(Operation),
}

pub struct Compiler;

impl Compiler {
    pub fn compile(tokens: impl IntoIterator<Item = Token>) -> Vec<Instruction> {
        let mut op_stack: Vec<Operation> = Vec::new();
        let mut output: Vec<Instruction> = Vec::new();

        for token in tokens {
            match token {
                Token::TAG(t) => {
                    output.push(Instruction::PUSH(t));
                    if let Some(Operation::NOT) = op_stack.last() {
                        output.push(Instruction::APPLY(
                            op_stack.pop().expect("Won't panic since we checked last"),
                        ))
                    }
                }
                Token::NOT => {
                    op_stack.push(Operation::NOT);
                }
                Token::AND => {
                    while let Some(op) = op_stack.last() {
                        if *op >= Operation::AND {
                            output.push(Instruction::APPLY(
                                op_stack.pop().expect("Won't panic since we checked last"),
                            ));
                        } else {
                            break;
                        }
                    }
                    op_stack.push(Operation::AND);
                }
                Token::OR => {
                    while let Some(op) = op_stack.last() {
                        if *op >= Operation::OR {
                            output.push(Instruction::APPLY(
                                op_stack.pop().expect("Won't panic since we checked last"),
                            ));
                        } else {
                            break;
                        }
                    }
                    op_stack.push(Operation::OR);
                }
                Token::INVALID => {}
            }
        }

        while let Some(op) = op_stack.pop() {
            output.push(Instruction::APPLY(op));
        }

        output
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::OR => write!(f, "OR"),
            Operation::AND => write!(f, "AND"),
            Operation::NOT => write!(f, "NOT "),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::PUSH(op) => writeln!(f, "PUSH({op})"),
            Instruction::APPLY(operation) => writeln!(f, "APPLY({operation})"),
        }
    }
}
