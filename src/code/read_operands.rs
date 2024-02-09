use core::panic;

use super::Definition;

pub fn read_operands(def: &Definition, instructions: &[u8]) -> (Vec<usize>, usize) {
    let mut operands = Vec::with_capacity(def.operand_widths.len());

    let mut offset = 0;
    for width in &def.operand_widths {
        let result = match width {
            2 => u16::from_be_bytes(instructions[offset..offset + 2].try_into().unwrap()) as usize,
            _ => panic!("not able to read operand with width: {width}"),
        };

        operands.push(result);

        offset += width;
    }

    (operands, offset)
}
