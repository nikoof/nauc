use parser::{ast, Token};
use std::{fs::File, io::Read};

mod parser;

fn main() {
    let mut file = File::open("sample.bf").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).ok();

    let ast = ast(&code);

    let mut memory: [u8; 30_000] = [0; 30_000];
    let mut pointer = 0;
    let mut current = 0;
    while current < ast.len() {
        match ast[current] {
            Token::Right => pointer += 1,
            Token::Left => pointer -= 1,
            Token::Add => memory[pointer] += 1,
            Token::Sub => memory[pointer] -= 1,
            Token::Read => (),
            Token::Write => print!("{}", memory[pointer] as char),
            Token::Loop(jump) => {
                if memory[pointer] != 0 {
                    current = jump
                }
            }
            Token::Break(jump) => {
                if memory[pointer] == 0 {
                    current = jump;
                }
            }
            Token::Comment => (),
        }
        current += 1;
    }
}
