use std::error::Error;

use crate::ex1::solution::Ex1;

pub trait SolveSolution {
    fn solve(number: SolutionType) -> Result<String, Box<dyn Error>>;
}

pub enum SolutionType {
    Sol1,
    Sol2,
}

pub mod ex1 {
    pub mod solution;
}

fn main() {
    let result = Ex1::solve(SolutionType::Sol2);

    println!("Result: {:?}", result);
}
