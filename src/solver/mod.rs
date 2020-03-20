mod clp_cmd;

use simple_error::SimpleError;

use crate::problem::Problem;
pub use crate::solver::clp_cmd::SolverClpCMD;

pub trait Solver<T: Problem> {
    fn solve(&self, p: T) -> Result<(), SimpleError>;
}
