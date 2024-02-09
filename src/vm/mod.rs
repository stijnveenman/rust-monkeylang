use core::panic;

use crate::{
    code::{read_operands::read_u16, Instructions, Opcode},
    compiler::Bytecode,
    object::Object,
};

const STACK_SIZE: usize = 2048;

pub struct Vm {
    instructions: Instructions,
    constants: Vec<Object>,

    stack: [Object; STACK_SIZE],
    sp: usize,
}

type R = Result<(), &'static str>;

impl Vm {
    fn new(bytecode: Bytecode) -> Vm {
        Vm {
            instructions: bytecode.instructions,
            constants: bytecode.constants,

            stack: std::array::from_fn(|_| Object::Null),
            sp: 0,
        }
    }

    fn stack_stop(&self) -> &Object {
        if self.sp == 0 {
            panic!("stack_top on empty stack");
        }

        &self.stack[self.sp - 1]
    }

    fn run(&mut self) -> R {
        let mut ip = 0;
        while ip < self.instructions.0.len() {
            let op: Opcode = self.instructions.0[ip].into();

            match op {
                Opcode::OpConstant => {
                    let const_index = read_u16(&self.instructions.0[ip + 1..]);
                    ip += 2;

                    self.push(self.constants[const_index].from_ref())?;
                }
            };

            ip += 1;
        }

        Ok(())
    }

    fn push(&mut self, object: Object) -> R {
        if self.sp > STACK_SIZE {
            return Err("stack overflow");
        }

        self.stack[self.sp] = object;
        self.sp += 1;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use rstest::rstest;

    use crate::{compiler::Compiler, object::test::test_object, parser::Parser, vm::Vm};

    #[rstest]
    #[case("1", 1)]
    #[case("2", 2)]
    #[case("1 + 2", 2)] //FIXME
    fn test_integer_arithmetic(#[case] input: &str, #[case] expected: i32) {
        test_vm(input, expected)
    }

    fn test_vm<T: Any>(input: &str, expected: T) {
        let mut parser = Parser::new(input.into());
        let (program, errors) = parser.parse_program();

        assert_eq!(errors, Vec::<String>::new());

        let mut compiler = Compiler::new();
        compiler
            .compile((&program).into())
            .expect("Failed to compile program");

        let mut vm = Vm::new(compiler.bytecode());
        vm.run();

        let element = vm.stack_stop();

        test_object(element, &expected)
    }
}
