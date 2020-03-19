use crate::solver::Solver;
use crate::problem::ProblemLp;

pub struct SolverClpCMD {

}

impl SolverClpCMD {
    pub fn new() -> Self { Self{} }
}

impl<T: ProblemLp> Solver<T> for SolverClpCMD {

    fn solve(&self, p: T) -> () {
        
    }
}
