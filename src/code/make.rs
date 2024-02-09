use core::panic;

use super::Opcode;

pub fn make(op: Opcode, operands: &[usize]) -> Vec<u8> {
    let def = op.definition();

    let instruction_len = def.operand_widths.iter().sum::<usize>() + 1;

    let mut instruction: Vec<u8> = Vec::with_capacity(instruction_len);

    instruction.push(op.into());

    for (operand, width) in operands.iter().zip(def.operand_widths) {
        let bytes = match width {
            2 => u16::try_from(*operand).unwrap().to_be_bytes(),
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
    fn name(#[case] op: Opcode, #[case] operands: Vec<usize>, #[case] expected: Vec<u8>) {
        let result = make(op, &operands);

        assert_eq!(expected, result)
    }
}
