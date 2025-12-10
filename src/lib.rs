use std::error::Error;

pub trait SolveSolution {
    fn solve_1() -> Result<String, Box<dyn Error>>;
    fn solve_2() -> Result<String, Box<dyn Error>>;
}

pub mod ex1 {
    pub mod solution;
}
pub mod ex2 {
    pub mod solution;
}
pub mod ex3 {
    pub mod solution;
}
pub mod ex4 {
    pub mod solution;
}
pub mod ex5 {
    pub mod solution;
}
pub mod ex6 {
    pub mod solution;
}
pub mod ex7 {
    pub mod solution;
}
// pub mod ex8 {
//     pub mod solution;
// }
// pub mod ex9 {
//     pub mod solution;
// }
// pub mod ex10 {
//     pub mod solution;
// }
