pub mod make;
pub mod read_operands;

#[repr(u8)]
pub enum Opcode {
    OpConstant,
}

impl From<Opcode> for u8 {
    fn from(m: Opcode) -> u8 {
        m as u8
    }
}

#[allow(dead_code)]
pub struct Definition {
    name: &'static str,
    operand_widths: Vec<usize>,
}

impl Opcode {
    pub fn find_definition(op: &Opcode) -> Definition {
        match op {
            Opcode::OpConstant => Definition {
                name: "OpConstant",
                operand_widths: vec![2],
            },
        }
    }

    pub fn definition(&self) -> Definition {
        Opcode::find_definition(self)
    }
}

pub trait Instructions {
    fn format_instructions(&self) -> String;
}

impl Instructions for Vec<u8> {
    fn format_instructions(&self) -> String {
        "".into()
    }
}

#[cfg(test)]
pub mod test {
    use rstest::rstest;

    use crate::code::{make::make, read_operands::read_operands, Instructions, Opcode};

    #[rstest]
    #[case(Opcode::OpConstant, vec![65535], 2)]
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
            make(Opcode::OpConstant, &[1]),
            make(Opcode::OpConstant, &[2]),
            make(Opcode::OpConstant, &[65535]),
        ];

        let expected = "0000 OpConstant 1
0003 OpConstant 2
0006 OpConstant 65535";

        let bytecode = instructions.into_iter().flatten().collect::<Vec<_>>();

        assert_eq!(expected, bytecode.format_instructions())
    }
}
