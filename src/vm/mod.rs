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

type R = Result<(), String>;

impl Vm {
    pub fn new(bytecode: Bytecode) -> Vm {
        Vm {
            instructions: bytecode.instructions,
            constants: bytecode.constants,

            stack: std::array::from_fn(|_| Object::Null),
            sp: 0,
        }
    }

    pub fn stack_top(&self) -> &Object {
        if self.sp == 0 {
            panic!("stack_top on empty stack");
        }

        &self.stack[self.sp - 1]
    }

    fn exec_binary_integer_op(&mut self, op: Opcode, left: i64, right: i64) -> R {
        let result = match op {
            Opcode::OpAdd => left + right,
            Opcode::OpMul => left * right,
            Opcode::OpDiv => left / right,
            Opcode::OpSub => left - right,

            op => return Err(format!("unkown Integer operation {:?}", op)),
        };

        self.push(Object::Integer(result))
    }

    fn exec_binary_op(&mut self, op: Opcode) -> R {
        let right = self.pop();
        let left = self.pop();

        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => {
                self.exec_binary_integer_op(op, left, right)
            }
            (left, right) => Err(format!(
                "unsupported types for binary op {} {}",
                left.type_str(),
                right.type_str()
            )),
        }
    }

    fn exec_comparison(&mut self, op: Opcode) -> R {
        let right = self.pop();
        let left = self.pop();

        let result = match op {
            Opcode::OpEqual => left == right,
            Opcode::OpNotEqual => left != right,
            Opcode::OpGreaterThan => {
                let left: i64 = left.try_into()?;
                let right: i64 = right.try_into()?;

                left > right
            }
            op => return Err(format!("unsupported operation for comparison op {:?}", op)),
        };

        self.push(Object::Boolean(result))
    }

    fn exec_bang(&mut self) -> R {
        let operand = self.pop();

        match operand {
            Object::Boolean(val) => self.push(Object::Boolean(!val)),
            _ => self.push(Object::Boolean(false)),
        }
    }

    fn exec_minus(&mut self) -> R {
        let operand = self.pop();

        let Object::Integer(val) = operand else {
            return Err(format!(
                "unsupported type for negation: {}",
                operand.type_str()
            ));
        };

        self.push(Object::Integer(-val))
    }

    pub fn run(&mut self) -> R {
        let mut ip = 0;
        while ip < self.instructions.0.len() {
            let op: Opcode = self.instructions.0[ip].into();

            match op {
                Opcode::OpConstant => {
                    let const_index = read_u16(&self.instructions.0[ip + 1..]);
                    ip += 2;

                    self.push(self.constants[const_index].from_ref())?;
                }
                Opcode::OpAdd | Opcode::OpMul | Opcode::OpDiv | Opcode::OpSub => {
                    self.exec_binary_op(op)?;
                }
                Opcode::OpPop => {
                    self.pop();
                }
                Opcode::OpTrue => {
                    self.push(Object::Boolean(true))?;
                }
                Opcode::OpFalse => {
                    self.push(Object::Boolean(false))?;
                }
                Opcode::OpEqual | Opcode::OpNotEqual | Opcode::OpGreaterThan => {
                    self.exec_comparison(op)?;
                }
                Opcode::OpBang => {
                    self.exec_bang()?;
                }
                Opcode::OpMinus => {
                    self.exec_minus()?;
                }
                Opcode::OpJumpNotTruthy | Opcode::OpJump => {}
            };

            ip += 1;
        }

        Ok(())
    }

    fn pop(&mut self) -> Object {
        let o = self.stack[self.sp - 1].from_ref();
        self.sp -= 1;
        o
    }

    pub fn last_popped(&self) -> &Object {
        &self.stack[self.sp]
    }

    fn push(&mut self, object: Object) -> R {
        if self.sp > STACK_SIZE {
            return Err("stack overflow".into());
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
    #[case("1 + 2", 3)]
    #[case("1 - 2", -1)]
    #[case("1 * 2", 2)]
    #[case("4 / 2", 2)]
    #[case("50 / 2 * 2 + 10 - 5", 55)]
    #[case("5 + 5 + 5 + 5 - 10", 10)]
    #[case("2 * 2 * 2 * 2 * 2", 32)]
    #[case("5 * 2 + 10", 20)]
    #[case("5 + 2 * 10", 25)]
    #[case("5 * (2 + 10)", 60)]
    #[case("-5", -5)]
    #[case("-10", -10)]
    #[case("-50 + 100 + -50", 0)]
    #[case("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50)]
    fn test_integer_arithmetic(#[case] input: &str, #[case] expected: i32) {
        test_vm(input, expected)
    }

    #[rstest]
    #[case("true", true)]
    #[case("false", false)]
    #[case("1 < 2", true)]
    #[case("1 > 2", false)]
    #[case("1 < 1", false)]
    #[case("1 > 1", false)]
    #[case("1 == 1", true)]
    #[case("1 != 1", false)]
    #[case("1 == 2", false)]
    #[case("1 != 2", true)]
    #[case("true == true", true)]
    #[case("false == false", true)]
    #[case("true == false", false)]
    #[case("true != false", true)]
    #[case("false != true", true)]
    #[case("(1 < 2) == true", true)]
    #[case("(1 < 2) == false", false)]
    #[case("(1 > 2) == true", false)]
    #[case("(1 > 2) == false", true)]
    #[case("!true", false)]
    #[case("!false", true)]
    #[case("!5", false)]
    #[case("!!true", true)]
    #[case("!!false", false)]
    #[case("!!5", true)]
    fn test_boolean_expression(#[case] input: &str, #[case] expected: bool) {
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
        vm.run().expect("vm failed to run");

        let element = vm.last_popped();

        println!("{} -> {}", input, element);

        test_object(element, &expected)
    }
}
