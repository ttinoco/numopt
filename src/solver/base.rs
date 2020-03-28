use std::fmt;
use simple_error::SimpleError;

use crate::problem::{ProblemFloat, 
                     ProblemSol};

pub enum SolverStatus {
    Solved,
    Unknown,
    Error,
}

pub trait Solver<T, N: ProblemFloat> {
    fn new() -> Self;
    fn status(&self) -> &SolverStatus;
    fn solution(&self) -> &Option<ProblemSol<N>>;
    fn solve(&mut self, p: &mut T) -> Result<(), SimpleError>;
}

impl SolverStatus {
    pub fn is_solved(&self) -> bool {
        match self {
            SolverStatus::Solved => true,
            _ => false
        }
    }
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

