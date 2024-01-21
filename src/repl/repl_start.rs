use std::io::{self, stdout, BufRead, Write};

use crate::tokens::{lexer::Lexer, token::Token};

const PROMPT: &str = ">>";

pub fn start() {
    let stdin = io::stdin();

    print!("{}", PROMPT);
    stdout().flush().expect("failed to flush stdout");

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line from stdin");

        let mut lexer = Lexer::new(line);

        loop {
            let token = lexer.next_token();
            println!("{:?}", token);
            if token == Token::EOF {
                break;
            }
        }
        print!("{}", PROMPT);
        stdout().flush().expect("failed to flush stdout");
    }
}
