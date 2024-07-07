pub mod parser;

#[derive(Debug)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisor_test: u64,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
}

#[derive(Debug)]
pub enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
}
impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(a, b) => todo!(),
            Operation::Mul(a, b) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Term {
    Old,
    Constant(u64),
}

fn main() {
    include_str!("test.txt").split("\n\n").map(|monkey| {});
}
