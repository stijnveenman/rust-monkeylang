use crate::code::Instructions;

pub struct Frame {
    pub instructions: Instructions,
    pub ip: usize,
}

impl Frame {
    pub fn new(function: Instructions) -> Frame {
        Frame {
            instructions: function,
            ip: 0,
        }
    }
}
