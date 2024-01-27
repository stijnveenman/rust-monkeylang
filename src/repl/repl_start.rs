use std::io::{self, stdout, BufRead, Write};

use crate::{
    ast::AstNode,
    parser::Parser,
    tokens::{lexer::Lexer, token::Token},
};

const PROMPT: &str = ">>";

pub fn start() {
    let stdin = io::stdin();

    print!("{}", PROMPT);
    stdout().flush().expect("failed to flush stdout");

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line from stdin");

        let mut parser = Parser::new(line);

        let (program, errors) = parser.parse_program();

        if !errors.is_empty() {
            println!("{}", errors.join("\n"));
        } else {
            println!("{}", program.string());
        }

        print!("{}", PROMPT);
        stdout().flush().expect("failed to flush stdout");
    }
}
