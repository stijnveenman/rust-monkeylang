use super::Statement;

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}
