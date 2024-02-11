use std::io::{self, stdout, BufRead, Write};

use crate::{compiler::Compiler, parser::Parser, vm::Vm};

const PROMPT: &str = ">>";

fn run(compiler: &mut Compiler, vm: &mut Vm, line: String) -> Result<String, String> {
    let mut parser = Parser::new(line);

    let (program, errors) = parser.parse_program();

    if !errors.is_empty() {
        Err(errors.join("\n").to_string())?;
    }

    compiler.compile((&program).into())?;

    vm.with_bytecode(compiler.bytecode());

    vm.run()?;

    let result = vm.last_popped();
    Ok(format!("{}", result))
}

pub fn start() {
    let stdin = io::stdin();

    print!("{}", PROMPT);
    stdout().flush().expect("failed to flush stdout");

    let mut compiler = Compiler::new();
    let mut vm = Vm::new();

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line from stdin");
        compiler = compiler.new_from();

        match run(&mut compiler, &mut vm, line) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e}"),
        }

        print!("{}", PROMPT);
        stdout().flush().expect("failed to flush stdout");
    }
}
