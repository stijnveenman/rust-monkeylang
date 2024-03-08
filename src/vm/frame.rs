use crate::code::Instructions;

pub struct Frame {
    pub instructions: Instructions,
    pub ip: usize,
    pub base_poiner: usize,
}

impl Frame {
    pub fn new(instructions: Instructions, base_poiner: usize) -> Frame {
        Frame {
            instructions,
            ip: usize::MAX,
            base_poiner,
        }
    }
}
