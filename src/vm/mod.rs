mod frame;

use core::panic;
use std::collections::HashMap;

use crate::{
    code::{
        read_operands::{read_u16, read_u8},
        Opcode,
    },
    compiler::Bytecode,
    object::Object,
};

use self::frame::Frame;

const STACK_SIZE: usize = 2048;
const GLOBALS_SIZE: usize = 65536;

pub struct Vm {
    constants: Vec<Object>,

    stack: [Object; STACK_SIZE],
    globals: Box<[Object]>,
    sp: usize,

    frames: Vec<Frame>,
}

type R = Result<(), String>;

impl Vm {
    pub fn new() -> Vm {
        let bytecode = Bytecode::empty();
        Vm {
            constants: bytecode.constants,

            stack: std::array::from_fn(|_| Object::Null),
            globals: vec![Object::Null; GLOBALS_SIZE].into_boxed_slice(),
            sp: 0,

            frames: vec![],
        }
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame)
    }

    fn pop_frame(&mut self) -> Frame {
        self.frames.pop().unwrap()
    }

    fn frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    fn frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn with_bytecode(&mut self, bytecode: Bytecode) {
        let frame = Frame::new(bytecode.instructions, 0);
        self.constants = bytecode.constants;

        self.stack = std::array::from_fn(|_| Object::Null);
        self.sp = 0;

        self.push_frame(frame);
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

    fn exec_binary_string_op(&mut self, op: Opcode, left: String, right: String) -> R {
        let result = match op {
            Opcode::OpAdd => left + &right,
            op => return Err(format!("unkown String operation {:?}", op)),
        };

        self.push(Object::String(result))
    }

    fn exec_binary_op(&mut self, op: Opcode) -> R {
        let right = self.pop();
        let left = self.pop();

        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => {
                self.exec_binary_integer_op(op, left, right)
            }
            (Object::String(left), Object::String(right)) => {
                self.exec_binary_string_op(op, left, right)
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
            Object::Null => self.push(Object::Boolean(true)),
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
        while self.frame().ip < self.frame().instructions.0.len() {
            let instructions = &self.frame().instructions.0;
            let ip = self.frame().ip;
            let op: Opcode = instructions[ip].into();

            match op {
                Opcode::OpNoop => {}
                Opcode::OpConstant => {
                    let const_index = read_u16(&instructions[ip + 1..]);
                    self.frame_mut().ip += 2;

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
                Opcode::OpJumpNotTruthy => {
                    let pos = read_u16(&instructions[ip + 1..]);
                    self.frame_mut().ip += 2;

                    let condition = self.pop();
                    if !condition.is_truthy() {
                        self.frame_mut().ip = pos - 1;
                    }
                }
                Opcode::OpJump => {
                    let pos = read_u16(&instructions[ip + 1..]);
                    self.frame_mut().ip = pos - 1;
                }
                Opcode::OpNull => {
                    self.push(Object::Null)?;
                }
                Opcode::OpSetGlobal => {
                    let index = read_u16(&instructions[ip + 1..]);
                    self.frame_mut().ip += 2;

                    self.globals[index] = self.pop();
                }
                Opcode::OpGetGlobal => {
                    let index = read_u16(&instructions[ip + 1..]);
                    self.frame_mut().ip += 2;

                    self.push(self.globals[index].from_ref())?;
                }

                Opcode::OpArray => {
                    let count = read_u16(&instructions[ip + 1..]);
                    self.frame_mut().ip += 2;

                    let array = self.build_array(self.sp - count, self.sp);
                    self.sp -= count;

                    self.push(array)?;
                }

                Opcode::OpHash => {
                    let count = read_u16(&instructions[ip + 1..]);
                    self.frame_mut().ip += 2;

                    let hash = self.build_hash(self.sp - count, self.sp)?;
                    self.sp -= count;

                    self.push(hash)?;
                }

                Opcode::OpIndex => {
                    let index = self.pop();
                    let left = self.pop();

                    self.exec_index(left, index)?;
                }
                Opcode::OpCall => {
                    self.frame_mut().ip += 1;
                    let Object::CompiledFunction(instructions, num_locals) = self.stack_top()
                    else {
                        return Err("Calling non-function".into());
                    };

                    let frame = Frame::new(instructions.clone(), self.sp);

                    self.sp = frame.base_poiner + num_locals;

                    self.push_frame(frame);

                    continue;
                }
                Opcode::OpReturnValue => {
                    let value = self.pop();

                    let frame = self.pop_frame();
                    self.sp = frame.base_poiner - 1;

                    self.push(value)?;
                }
                Opcode::OpReturn => {
                    let frame = self.pop_frame();
                    self.sp = frame.base_poiner - 1;

                    self.push(Object::Null)?;
                }
                Opcode::OpSetLocal => {
                    let local_index = read_u8(&instructions[ip + 1..]);

                    self.frame_mut().ip += 1;

                    self.stack[self.frame().base_poiner + local_index] = self.pop();
                }
                Opcode::OpGetLocal => {
                    let local_index = read_u8(&instructions[ip + 1..]);

                    self.frame_mut().ip += 1;

                    let o = self.stack[self.frame().base_poiner + local_index].from_ref();
                    self.push(o)?;
                }
            };

            self.frame_mut().ip += 1;
        }

        Ok(())
    }

    fn exec_array_index(&mut self, left: Vec<Object>, i: i64) -> R {
        if i < 0 || i > (left.len() as i64) - 1 {
            return self.push(Object::Null);
        }

        self.push(left[i as usize].from_ref())
    }

    fn exec_hash_index(&mut self, left: HashMap<Object, Object>, i: Object) -> R {
        if !i.hashable() {
            return Err(format!("unusable as hash key: {}", i.type_str()));
        }

        let val = match left.get(&i) {
            Some(i) => i.from_ref(),
            None => Object::Null,
        };

        self.push(val)
    }

    fn exec_index(&mut self, left: Object, right: Object) -> R {
        match (left, right) {
            (Object::Array(left), Object::Integer(i)) => self.exec_array_index(left, i),
            (Object::Hash(left), i) => self.exec_hash_index(left, i),
            (left, _) => Err(format!("index operator not supported: {}", left.type_str())),
        }
    }

    fn build_hash(&mut self, start: usize, end: usize) -> Result<Object, String> {
        let mut hm = HashMap::new();

        for i in (start..end).step_by(2) {
            let key = self.stack[i].from_ref();
            let value = self.stack[i + 1].from_ref();

            if !key.hashable() {
                return Err(format!("unsable as hash key: {}", key.type_str()));
            }

            hm.insert(key, value);
        }

        Ok(Object::Hash(hm))
    }

    fn build_array(&mut self, start: usize, end: usize) -> Object {
        let mut elements = Vec::with_capacity(end - start);

        for i in start..end {
            elements.push(self.stack[i].from_ref());
        }

        Object::Array(elements)
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

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    use rstest::rstest;

    use crate::{
        compiler::Compiler,
        object::{
            test::{test_null, test_object},
            Object,
        },
        parser::Parser,
        vm::Vm,
    };

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
        let element = test_vm(input);
        test_object(&element, &expected)
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
    #[case("!(if (false) { 5; })", true)]
    fn test_boolean_expression(#[case] input: &str, #[case] expected: bool) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case("if (true) { 10 }", 10)]
    #[case("if (true) { 10 } else { 20 }", 10)]
    #[case("if (false) { 10 } else { 20 } ", 20)]
    #[case("if (1) { 10 }", 10)]
    #[case("if (1 < 2) { 10 }", 10)]
    #[case("if (1 < 2) { 10 } else { 20 }", 10)]
    #[case("if (1 > 2) { 10 } else { 20 }", 20)]
    #[case("if ((if (false) { 10 })) { 10 } else { 20 }", 20)]
    fn test_conditionals(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case("if (1 > 2) { 10 }")]
    #[case("if (false) { 10 }")]
    fn test_conditionals_null(#[case] input: &str) {
        let element = test_vm(input);
        test_null(&element)
    }

    #[rstest]
    #[case("let one = 1; one", 1)]
    #[case("let one = 1; let two = 2; one + two", 3)]
    #[case("let one = 1; let two = one + one; one + two", 3)]
    fn test_global_let(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case("\"monkey\"", "monkey")]
    #[case("\"mon\" + \"key\"", "monkey")]
    #[case("\"mon\" + \"key\" + \"banana\"", "monkeybanana")]
    fn test_string_expression(#[case] input: &str, #[case] expected: &'static str) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case("[]", vec![])]
    #[case("[1, 2, 3]", vec![1,2,3])]
    #[case("[1 + 2, 3 * 4, 5 + 6]", vec![3, 12, 11])]
    fn test_array_literals(#[case] input: &str, #[case] expected: Vec<i64>) {
        let element = test_vm(input);

        let Object::Array(array) = element else {
            panic!("expected Object::Array got {:?}", element);
        };

        assert_eq!(array.len(), expected.len());
        for (object, expected) in array.iter().zip(expected) {
            test_object(object, &expected);
        }
    }

    #[rstest]
    #[case("{}", vec![])]
    #[case("{1: 2, 2: 3}", vec![(1,2),(2,3)])]
    #[case("{1 + 1: 2 * 2, 3 + 3: 4 * 4}", vec![(2,4),(6,16)])]
    fn test_hash_literals(#[case] input: &str, #[case] expected: Vec<(i64, i64)>) {
        let element = test_vm(input);

        let Object::Hash(hm) = element else {
            panic!("expected Object::Hash got {:?}", element);
        };

        assert_eq!(hm.len(), expected.len());
        for (key, value) in expected {
            assert_eq!(hm.get(&Object::Integer(key)), Some(&Object::Integer(value)));
        }
    }

    #[rstest]
    #[case("[1, 2, 3][1]", 2)]
    #[case("[1, 2, 3][0 + 2]", 3)]
    #[case("[[1, 1, 1]][0][0]", 1)]
    #[case("{1: 1, 2: 2}[1]", 1)]
    #[case("{1: 1, 2: 2}[2]", 2)]
    fn test_index_expression(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case(
        "
let fivePlusTen = fn() { 5 + 10; };
fivePlusTen();",
        15
    )]
    #[case(
        "
let one = fn() { 1; };
let two = fn() { 2; };
one() + two()",
        3
    )]
    #[case(
        "
let a = fn() { 1 };
let b = fn() { a() + 1 };
let c = fn() { b() + 1 };
c();
",
        3
    )]
    #[case(
        "
let earlyExit = fn() { return 99; 100; };
earlyExit();
",
        99
    )]
    #[case(
        "
let earlyExit = fn() { return 99; return 100; };
earlyExit();
",
        99
    )]
    #[case(
        "
let returnsOne = fn() { 1; };
let returnsOneReturner = fn() { returnsOne; };
returnsOneReturner()();
",
        1
    )]
    fn test_calling_functions_without_arguments(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case(
        "
           let one = fn() { let one = 1; one };
           one();
           ",
        1
    )]
    #[case(
        "
           let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
           oneAndTwo();
           ",
        3
    )]
    #[case(
        "
           let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
           let threeAndFour = fn() { let three = 3; let four = 4; three + four; };
           oneAndTwo() + threeAndFour();
           ",
        10
    )]
    #[case(
        "
           let firstFoobar = fn() { let foobar = 50; foobar; };
           let secondFoobar = fn() { let foobar = 100; foobar; };
           firstFoobar() + secondFoobar();
           ",
        150
    )]
    #[case(
        "
           let globalSeed = 50;
           let minusOne = fn() {
let num = 1;
               globalSeed - num;
           }
           let minusTwo = fn() {
               let num = 2;
               globalSeed - num;
           }
           minusOne() + minusTwo();
           ",
        97
    )]
    #[case(
        "
           let returnsOneReturner = fn() {
               let returnsOne = fn() { 1; };
               returnsOne;
           };
           returnsOneReturner()();
",
        1
    )]
    fn test_calling_functions_with_bindings(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case(
        "
let noReturn = fn() { };
noReturn();"
    )]
    #[case(
        "
let noReturn = fn() { };
let noReturnTwo = fn() { noReturn(); };
noReturn();
noReturnTwo();"
    )]
    fn test_no_return_value(#[case] input: &str) {
        let element = test_vm(input);
        test_null(&element)
    }

    #[rstest]
    #[case("[][0]")]
    #[case("[1, 2, 3][99]")]
    #[case("[1][-1]")]
    #[case("{1: 1}[0]")]
    #[case("{}[0]")]
    fn test_index_expression_null(#[case] input: &str) {
        let element = test_vm(input);
        test_null(&element)
    }

    #[rstest]
    #[case(
        "let identity = fn (a) {a;};
identity(4);",
        4
    )]
    #[case(
        "let sum = fn (a, b) {a + b;};
identity(1,2);",
        3
    )]
    fn test_function_call_with_arguments(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    fn test_vm(input: &str) -> Object {
        let mut parser = Parser::new(input.into());
        let (program, errors) = parser.parse_program();

        assert_eq!(errors, Vec::<String>::new());

        let mut compiler = Compiler::new();
        compiler
            .compile((&program).into())
            .expect("Failed to compile program");

        let mut vm = Vm::new();
        vm.with_bytecode(compiler.bytecode());
        vm.run().expect("vm failed to run");

        let element = vm.last_popped();

        println!("{} -> {}", input, element);

        element.from_ref()
    }
}
