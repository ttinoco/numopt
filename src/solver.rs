
use super::problem::Problem;

pub trait Solver<T: Problem> {
    fn solve(&self, p: T) -> ();
}