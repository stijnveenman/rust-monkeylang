use super::Statement;

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
