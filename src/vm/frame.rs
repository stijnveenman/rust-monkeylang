use crate::code::Instructions;

pub struct Frame {
    pub instructions: Instructions,
    pub ip: usize,
}

impl Frame {
    pub fn new(instructions: Instructions) -> Frame {
        Frame {
            instructions,
            ip: 0,
        }
    }
}
