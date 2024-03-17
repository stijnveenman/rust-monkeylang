use crate::code::Instructions;
use crate::vm::Object;

pub struct Frame {
    pub instructions: Instructions,
    pub ip: usize,
    pub base_poiner: usize,
    pub free: Vec<Object>,
}

impl Frame {
    pub fn new(instructions: Instructions, base_poiner: usize, free: Vec<Object>) -> Frame {
        Frame {
            instructions,
            ip: usize::MAX,
            base_poiner,
            free,
        }
    }
}
