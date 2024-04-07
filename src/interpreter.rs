use crate::parser::Token;
use thiserror::Error;

const MEM_SIZE: usize = 30_000;

pub struct Interpreter {
    program: Vec<Token>,
    tape: [u8; MEM_SIZE],
    index: usize,
    pc: usize,
    input_buffer: Vec<u8>,
}

// TODO: Replace this with builder pattern for Interpreter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterpreterProps {
    pub wrapping: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InterpreterError {
    #[error("Integer overflow at cell {0}")]
    IntegerOverflow(usize),

    #[error("Integer underflow at cell {0}")]
    IntegerUnderflow(usize),

    #[error("Pointer points to out of bounds memory.")]
    OutOfBounds,
}

impl Interpreter {
    pub fn new(program: Vec<Token>) -> Self {
        Self {
            program,
            tape: [0u8; MEM_SIZE],
            index: 0,
            pc: 0,
            input_buffer: Vec::new(),
        }
    }

    pub fn run(mut self, props: InterpreterProps) -> Result<(), InterpreterError> {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                Token::Right => {
                    self.index = self
                        .index
                        .checked_add(1)
                        .ok_or(InterpreterError::OutOfBounds)?
                }
                Token::Left => {
                    self.index = self
                        .index
                        .checked_sub(1)
                        .ok_or(InterpreterError::OutOfBounds)?
                }
                Token::Add => {
                    if props.wrapping {
                        self.tape[self.index] = self.tape[self.index].wrapping_add(1)
                    } else {
                        self.tape[self.index] = self.tape[self.index]
                            .checked_add(1)
                            .ok_or(InterpreterError::IntegerOverflow(self.pc))?
                    }
                }
                Token::Sub => {
                    if props.wrapping {
                        self.tape[self.index] = self.tape[self.index].wrapping_sub(1)
                    } else {
                        self.tape[self.index] = self.tape[self.index]
                            .checked_sub(1)
                            .ok_or(InterpreterError::IntegerUnderflow(self.pc))?
                    }
                }
                Token::Read => {
                    if let Some(value) = self.input_buffer.pop() {
                        self.tape[self.index] = value;
                    } else {
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).unwrap();
                        self.input_buffer = buf.into_bytes().into_iter().rev().collect();
                        self.tape[self.index] = self.input_buffer.pop().unwrap();
                    }
                }
                Token::Write => {
                    print!("{}", self.tape[self.index] as char)
                }
                Token::Loop(jump) => {
                    if self.tape[self.index] != 0 {
                        self.pc = jump
                    }
                }
                Token::Break(jump) => {
                    if self.tape[self.index] == 0 {
                        self.pc = jump;
                    }
                }
                Token::Comment => (),
            }
            self.pc += 1;
        }

        Ok(())
    }
}
