mod clp_cmd;

use simple_error::SimpleError;

use crate::problem::{Problem, ProblemSol};
pub use crate::solver::clp_cmd::SolverClpCMD;

pub enum SolverStatus {
    Solved,
    Unknown,
    Error,
}

pub trait Solver<T: Problem> {
    fn new() -> Self;
    fn status(&self) -> &SolverStatus;
    fn solution(&self) -> &Option<ProblemSol<T>>;
    fn solve(&self, p: T) -> Result<(), SimpleError>;
}
