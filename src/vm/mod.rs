mod frame;

use core::panic;
use std::collections::HashMap;

use crate::{
    builtin::{BuiltinFunction, BUILTINS},
    code::{
        read_operands::{read_u16, read_u8},
        Instructions, Opcode,
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
        let frame = Frame::new(bytecode.instructions, 0, vec![]);
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
        while self.frame().ip < self.frame().instructions.0.len() - 1
            || self.frame().ip == usize::MAX
        {
            self.frame_mut().ip = self.frame().ip.wrapping_add(1);

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
                    let num_args = read_u8(&instructions[ip + 1..]);
                    self.frame_mut().ip += 1;

                    self.exec_call(num_args)?;
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
                Opcode::OpGetBuiltin => {
                    let builtin_index = read_u8(&instructions[ip + 1..]);
                    self.frame_mut().ip += 1;

                    let builtin = BUILTINS.get(builtin_index).unwrap();

                    self.push(Object::Builtin(builtin.1))?;
                }
                Opcode::OpClosure => {
                    let cost_index = read_u16(&instructions[ip + 1..]);
                    let num_free = read_u8(&instructions[ip + 3..]);
                    self.frame_mut().ip += 3;

                    self.push_closure(cost_index, num_free)?;
                }
                Opcode::OpGetFree => {
                    let free_index = read_u8(&instructions[ip + 1..]);
                    self.frame_mut().ip += 1;

                    let object = self.frame().free[free_index].from_ref();
                    self.push(object)?;
                }
            };
        }

        Ok(())
    }

    fn push_closure(&mut self, cost_index: usize, num_free: usize) -> R {
        let Object::CompiledFunction(ins, a, b) = &self.constants[cost_index] else {
            return Err(format!("Not a function {}", self.constants[cost_index]));
        };

        let free = (0..num_free)
            .map(|i| self.stack[self.sp - num_free + i].from_ref())
            .collect::<Vec<_>>();
        self.sp -= num_free;

        let closure = Object::Closure(Instructions(ins.0.to_vec()), *a, *b, free);
        self.push(closure)?;

        Ok(())
    }

    fn exec_call(&mut self, num_args: usize) -> R {
        let item = self.stack[self.sp - 1 - num_args].from_ref();
        match item {
            Object::Closure(instructions, num_locals, num_parameters, free) => {
                self.call_closure(&instructions, num_locals, num_parameters, num_args, free)
            }
            Object::Builtin(builtin) => self.exec_builtin(builtin, num_args),
            _ => Err("calling non-function and non-built-in".into()),
        }
    }

    fn exec_builtin(&mut self, builtin: BuiltinFunction, num_args: usize) -> R {
        let args = self.stack[self.sp - num_args..self.sp].to_vec();

        let result = builtin.0(args);
        self.sp = self.sp - num_args - 1;

        self.push(result)
    }

    fn call_closure(
        &mut self,
        instructions: &Instructions,
        num_locals: usize,
        num_parameters: usize,
        num_args: usize,
        free: Vec<Object>,
    ) -> R {
        if num_args != num_parameters {
            return Err(format!(
                "wrong number of arguments: want={}, got={}",
                num_parameters, num_args
            ));
        }

        let frame = Frame::new(instructions.clone(), self.sp - num_args, free);

        self.sp = frame.base_poiner + num_locals;

        self.push_frame(frame);
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
sum(1,2);",
        3
    )]
    #[case(
        "
let sum = fn(a, b) {
    let c = a + b;
    c; 
};
sum(1, 2);
",
        3
    )]
    #[case(
        "
let sum = fn(a, b) {
    let c = a + b;
    c; 
};
sum(1, 2) + sum(3, 4);
",
        10
    )]
    #[case(
        "
let sum = fn(a, b) {
    let c = a + b;
    c; 
};
let outer = fn() {
    sum(1, 2) + sum(3, 4);
};
outer();
",
        10
    )]
    fn test_function_call_with_arguments(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected)
    }

    #[rstest]
    #[case("fn() { 1; }(1);", "wrong number of arguments: want=0, got=1")]
    #[case("fn(a) { a; }();", "wrong number of arguments: want=1, got=0")]
    #[case("fn(a, b) { a + b; }(1);", "wrong number of arguments: want=2, got=1")]
    fn test_invalid_argument_count(#[case] input: &str, #[case] expected_error: &str) {
        let result = test_vm_result(input);
        assert_eq!(result, Err(expected_error.into()));
    }

    #[rstest]
    #[case("len(\"\")", 0)]
    #[case("len(\"four\")", 4)]
    #[case("len(\"hello world\")", 11)]
    #[case("len([1, 2, 3])", 3)]
    #[case("len([])", 0)]
    #[case("first([1, 2, 3])", 1)]
    #[case("last([1, 2, 3])", 3)]
    fn test_builtin_functions(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected);
    }

    #[rstest]
    #[case(
        "
let newClosure = fn(a) {
    fn() { a; };
};
let closure = newClosure(99);
closure();
",
        99
    )]
    #[case(
        "
let newAdder = fn(a, b) {
    fn(c) { a + b + c };
};
let adder = newAdder(1, 2);
adder(8);
",
        11
    )]
    #[case(
        "
let newAdder = fn(a, b) {
    let c = a + b;
    fn(d) { c + d };
};
let adder = newAdder(1, 2);
adder(8);
",
        11
    )]
    #[case(
        "
let newAdderOuter = fn(a, b) {
    let c = a + b;
    fn(d) {
        let e = d + c;
        fn(f) { e + f; };
    };
};
let newAdderInner = newAdderOuter(1, 2)
let adder = newAdderInner(3);
adder(8);
",
        14
    )]
    #[case(
        "
let a = 1;
let newAdderOuter = fn(b) {
    fn(c) {
        fn(d) { a + b + c + d };
    }; 
};
let newAdderInner = newAdderOuter(2)
let adder = newAdderInner(3);
adder(8);
",
        14
    )]
    #[case(
        "
let newClosure = fn(a, b) {
    let one = fn() { a; };
    let two = fn() { b; };
    fn() { one() + two(); };
};
let closure = newClosure(9, 90);
closure();
",
        99
    )]
    fn test_closure(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected);
    }

    #[rstest]
    #[case("puts(\"hello\", \"world!\")")]
    #[case("first([])")]
    #[case("last([])")]
    #[case("rest([])")]
    fn test_builtin_function_null(#[case] input: &str) {
        let element = test_vm(input);
        test_null(&element);
    }

    #[rstest]
    #[case(
        "
let countDown = fn(x) {
    if (x == 0) {
        return 0;
    } else {
        countDown(x - 1);
    }
};
countDown(1);
",
        0
    )]
    fn test_recursive_closure(#[case] input: &str, #[case] expected: i64) {
        let element = test_vm(input);
        test_object(&element, &expected);
    }

    #[rstest]
    #[case("rest([1, 2, 3])", Object::Array(vec![Object::Integer(2), Object::Integer(3)]))]
    #[case("push([], 1)", Object::Array(vec![Object::Integer(1)]))]
    #[case(
        "len(1)",
        Object::Error("arguments to `len` not supported, got INTEGER".into())
    )]
    #[case(
        "len(\"one\", \"two\")",
        Object::Error("wrong number of arguments. got=2, want=1".into())
    )]
    #[case(
        "first(1)",
        Object::Error("argument to `first` must be ARRAY, got INTEGER".into())
    )]
    #[case(
        "last(1)",
        Object::Error("argument to `last` must be ARRAY, got INTEGER".into())
    )]
    #[case(
        "push(1,1)",
        Object::Error("argument to `push` must be ARRAY, got INTEGER".into())
    )]
    fn test_builtin_object(#[case] input: &str, #[case] output: Object) {
        let element = test_vm(input);

        assert_eq!(element, output);
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

    fn test_vm_result(input: &str) -> Result<Object, String> {
        let mut parser = Parser::new(input.into());
        let (program, errors) = parser.parse_program();

        assert_eq!(errors, Vec::<String>::new());

        let mut compiler = Compiler::new();
        compiler
            .compile((&program).into())
            .expect("Failed to compile program");

        let mut vm = Vm::new();
        vm.with_bytecode(compiler.bytecode());
        vm.run()?;

        let element = vm.last_popped();

        println!("{} -> {}", input, element);

        Ok(element.from_ref())
    }
}
