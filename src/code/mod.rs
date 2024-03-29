use self::read_operands::{fmt_instruction, read_operands};

pub mod make;
use std::fmt::{Debug, Display};
pub mod read_operands;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    OpConstant,
    OpPop,
    OpNull,
    OpArray,
    OpHash,
    OpIndex,
    OpGetBuiltin,

    OpCall,
    OpReturnValue,
    OpReturn,

    OpAdd,
    OpSub,
    OpMul,
    OpDiv,

    OpTrue,
    OpFalse,

    OpEqual,
    OpNotEqual,
    OpGreaterThan,

    OpMinus,
    OpBang,

    OpJumpNotTruthy,
    OpJump,

    OpSetGlobal,
    OpGetGlobal,

    OpSetLocal,
    OpGetLocal,

    OpClosure,
    OpGetFree,
    OpCurrentClosure,

    OpNoop,
}

impl Opcode {
    pub fn find_definition(op: &Opcode) -> Definition {
        let operand_widths = match op {
            Opcode::OpSetLocal
            | Opcode::OpGetLocal
            | Opcode::OpCall
            | Opcode::OpGetBuiltin
            | Opcode::OpGetFree => {
                vec![1]
            }

            Opcode::OpConstant
            | Opcode::OpArray
            | Opcode::OpHash
            | Opcode::OpJumpNotTruthy
            | Opcode::OpJump
            | Opcode::OpGetGlobal
            | Opcode::OpSetGlobal => vec![2],

            Opcode::OpPop
            | Opcode::OpCurrentClosure
            | Opcode::OpNull
            | Opcode::OpAdd
            | Opcode::OpSub
            | Opcode::OpMul
            | Opcode::OpDiv
            | Opcode::OpTrue
            | Opcode::OpFalse
            | Opcode::OpEqual
            | Opcode::OpNotEqual
            | Opcode::OpGreaterThan
            | Opcode::OpMinus
            | Opcode::OpIndex
            | Opcode::OpReturnValue
            | Opcode::OpReturn
            | Opcode::OpNoop
            | Opcode::OpBang => vec![],

            Opcode::OpClosure => vec![2, 1],
        };

        Definition {
            name: format!("{:?}", op),
            operand_widths,
        }
    }

    pub fn definition(&self) -> Definition {
        Opcode::find_definition(self)
    }

    pub fn is(&self, op: &Opcode) -> bool {
        self == op
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Instructions(pub Vec<u8>);

impl Debug for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        while i < self.0.len() {
            let op: Opcode = self.0[i].into();
            let def = op.definition();

            let (ops, read) = read_operands(&def, &self.0[i + 1..]);

            writeln!(f, "{:04} {}", i, fmt_instruction(&def, &ops))?;

            i += 1 + read
        }

        Ok(())
    }
}

impl From<Vec<u8>> for Instructions {
    fn from(value: Vec<u8>) -> Self {
        Instructions(value)
    }
}

impl From<Opcode> for u8 {
    fn from(m: Opcode) -> u8 {
        m as u8
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[allow(dead_code)]
pub struct Definition {
    name: String,
    operand_widths: Vec<usize>,
}

#[cfg(test)]
pub mod test {
    use rstest::rstest;

    use crate::code::{make::make, read_operands::read_operands, Instructions, Opcode};

    #[rstest]
    #[case(Opcode::OpConstant, vec![65535], 2)]
    #[case(Opcode::OpGetLocal, vec![255], 1)]
    fn test_read_operands(
        #[case] op: Opcode,
        #[case] operands: Vec<usize>,
        #[case] bytes_read: usize,
    ) {
        let def = op.definition();
        let instruction = make(op, &operands);

        let (operands_read, n) = read_operands(&def, &instruction[1..]);
        assert_eq!(bytes_read, n);

        assert_eq!(operands, operands_read)
    }

    #[test]
    fn test_instructions_string() {
        let instructions = vec![
            make(Opcode::OpAdd, &[]),
            make(Opcode::OpGetLocal, &[1]),
            make(Opcode::OpConstant, &[2]),
            make(Opcode::OpConstant, &[65535]),
            make(Opcode::OpClosure, &[65535, 255]),
        ];

        let expected = "0000 OpAdd
0001 OpGetLocal 1
0003 OpConstant 2
0006 OpConstant 65535
0009 OpClosure 65535 255\n";

        let bytecode = Instructions(instructions.into_iter().flatten().collect::<Vec<_>>());

        assert_eq!(expected, bytecode.to_string())
    }
}
