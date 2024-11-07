use crate::parser::Token;
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InterpreterError {
    #[error("Integer overflow at cell {0}")]
    IntegerOverflow(usize),

    #[error("Integer underflow at cell {0}")]
    IntegerUnderflow(usize),

    #[error("Pointer points to out of bounds memory.")]
    OutOfBounds,
}

#[derive(Clone, Default)]
pub struct NoProgram;
#[derive(Clone, Default)]
pub struct Program(Vec<Token>);

#[derive(Clone, Default)]
pub struct InterpreterBuilder<P> {
    program: P,
    wrapping: Option<bool>,
    memory: Option<usize>,
}

impl InterpreterBuilder<NoProgram> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<P> InterpreterBuilder<P> {
    pub fn program(self, program: Vec<Token>) -> InterpreterBuilder<Program> {
        InterpreterBuilder {
            program: Program(program),
            wrapping: self.wrapping,
            memory: self.memory,
        }
    }

    pub fn wrapping(mut self, wrapping: bool) -> Self {
        self.wrapping = Some(wrapping);
        self
    }

    pub fn memory(mut self, memory: usize) -> Self {
        self.memory = Some(memory);
        self
    }
}

impl InterpreterBuilder<Program> {
    pub fn build(self) -> Interpreter {
        let memory = self.memory.unwrap_or(30_000);
        let wrapping = self.wrapping.unwrap_or(true);

        Interpreter {
            program: self.program.0,
            tape: vec![0u8; memory],
            input_buffer: vec![],
            pc: 0,
            index: 0,
            wrapping,
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    program: Vec<Token>,
    tape: Vec<u8>,
    index: usize,
    pc: usize,
    input_buffer: Vec<u8>,
    wrapping: bool,
}

impl Interpreter {
    pub fn run(mut self) -> Result<(), InterpreterError> {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                Token::Right(count) => {
                    self.index = (self.index + count < self.tape.capacity())
                        .then(|| self.index + count)
                        .ok_or(InterpreterError::OutOfBounds)?
                }
                Token::Left(count) => {
                    self.index = self
                        .index
                        .checked_sub(count)
                        .ok_or(InterpreterError::OutOfBounds)?
                }
                Token::Add(count) => {
                    if self.wrapping {
                        self.tape[self.index] = self.tape[self.index].wrapping_add(count);
                    } else {
                        self.tape[self.index] = self.tape[self.index]
                            .checked_add(count)
                            .ok_or(InterpreterError::IntegerOverflow(self.pc))?
                    }
                }
                Token::Sub(count) => {
                    if self.wrapping {
                        self.tape[self.index] = self.tape[self.index].wrapping_sub(count);
                    } else {
                        self.tape[self.index] = self.tape[self.index]
                            .checked_sub(count)
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
