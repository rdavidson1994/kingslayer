use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading stdin");
    input.trim().to_owned()
}

mod lexer;
mod parser;

pub use lexer::Lexer;
pub use parser::Parser;
