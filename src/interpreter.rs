use crate::parser::Token;

const MEM_SIZE: usize = 30_000;

pub struct Interpreter {
    program: Vec<Token>,
    tape: [u8; MEM_SIZE],
    index: usize,
    pc: usize,
    input_buffer: Vec<u8>,
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

    pub fn run(mut self) {
        while self.pc < self.program.len() {
            match self.program[self.pc] {
                Token::Right => self.index += 1,
                Token::Left => self.index -= 1,
                Token::Add => self.tape[self.index] += 1,
                Token::Sub => self.tape[self.index] -= 1,
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
    }
}
