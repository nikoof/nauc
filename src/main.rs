use interpreter::Interpreter;
use parser::{ast, Token};
use std::{fs::File, io::Read};

mod interpreter;
mod parser;

fn main() {
    let mut file = File::open("sample.bf").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).ok();

    let ast = ast(&code);
    let interpreter = Interpreter::new(ast);
    interpreter.run();
}
