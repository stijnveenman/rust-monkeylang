use core::panic;

use super::Opcode;

pub fn make(op: Opcode, operands: &[usize]) -> Vec<u8> {
    let def = op.definition();

    let instruction_len = def.operand_widths.iter().sum::<usize>() + 1;

    let mut instruction: Vec<u8> = Vec::with_capacity(instruction_len);

    instruction.push(op.into());

    for (operand, width) in operands.iter().zip(def.operand_widths) {
        let bytes: Vec<u8> = match width {
            1 => [*operand as u8].to_vec(),
            2 => u16::try_from(*operand).unwrap().to_be_bytes().to_vec(),
            _ => panic!("unhandled width: {}", width),
        };

        instruction.extend_from_slice(&bytes);
    }

    instruction
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::code::Opcode;

    use super::make;

    #[rstest]
    #[case(Opcode::OpConstant, vec![65534usize], vec![Opcode::OpConstant.into(), 255u8, 254u8])]
    #[case(Opcode::OpAdd, vec![], vec![Opcode::OpAdd.into()])]
    #[case(Opcode::OpGetLocal, vec![255usize], vec![Opcode::OpGetLocal.into(), 255])]
    #[case(Opcode::OpClosure, vec![65534usize, 255usize], vec![Opcode::OpClosure.into(), 255, 254, 255])]
    fn name(#[case] op: Opcode, #[case] operands: Vec<usize>, #[case] expected: Vec<u8>) {
        let result = make(op, &operands);

        assert_eq!(expected, result)
    }
}
