mod clp_cmd;

use crate::problem::Problem;
pub use crate::solver::clp_cmd::SolverClpCMD;

pub trait Solver<T: Problem> {
    fn solve(&self, p: T) -> ();
}
