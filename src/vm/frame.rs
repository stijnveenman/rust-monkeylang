use crate::code::Instructions;
use crate::vm::Object;

pub struct Frame {
    pub instructions: Instructions,
    pub ip: usize,
    pub base_poiner: usize,
    pub free: Vec<Object>,
    pub num_locals: usize,
    pub num_parameters: usize,
}
