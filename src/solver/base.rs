use std::fmt;
use simple_error::SimpleError;

use crate::problem::{Problem, ProblemSol};

pub enum SolverStatus {
    Solved,
    Unknown,
    Error,
}

pub trait Solver<T: Problem> {
    fn new() -> Self;
    fn status(&self) -> &SolverStatus;
    fn solution(&self) -> &Option<ProblemSol<T>>;
    fn solve(&mut self, p: T) -> Result<(), SimpleError>;
}

impl fmt::Display for SolverStatus {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolverStatus::Error => write!(f, "error"),
            SolverStatus::Unknown => write!(f, "unknown"),
            SolverStatus::Solved => write!(f, "solved")
        }
    }
}
