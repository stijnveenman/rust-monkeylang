pub mod make;

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
