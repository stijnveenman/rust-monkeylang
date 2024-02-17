use core::panic;

use super::Definition;

pub fn read_operands(def: &Definition, instructions: &[u8]) -> (Vec<usize>, usize) {
    let mut operands = Vec::with_capacity(def.operand_widths.len());

    let mut offset = 0;
    for width in &def.operand_widths {
        let result = match width {
            2 => read_u16(&instructions[offset..]),
            _ => panic!("read_operands: not able to read operand with width: {width}"),
        };

        operands.push(result);

        offset += width;
    }
    let total_width = def.operand_widths.iter().sum();
    if offset != total_width {
        panic!(
            "Did not read full operand with, read {} of {}",
            offset, total_width
        );
    }

    (operands, offset)
}

pub fn read_u16(instructions: &[u8]) -> usize {
    u16::from_be_bytes(instructions[..2].try_into().unwrap()) as usize
}

pub fn fmt_instruction(def: &Definition, operands: &[usize]) -> String {
    let count = def.operand_widths.len();

    if count != operands.len() {
        return format!(
            "ERROR: operand len {} does not match defined {count}",
            operands.len()
        );
    }

    match count {
        0 => def.name.to_string(),
        1 => format!("{} {}", def.name, operands[0]),
        _ => format!("ERROR: fmt_instruction unhandled operand count for {count}"),
    }
}
