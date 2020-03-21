mod lp;

use sprs::TriMat;
use num_traits::Float;

pub use crate::problem::lp::ProblemLp;
pub use crate::problem::lp::ProblemLpWriter;

pub trait Problem {
    type N: Float;
    fn x(&self) -> &[Self::N];
    fn phi(&self) -> Self::N;
    fn gphi(&self) -> &[Self::N];
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn eval(&mut self, x: &[Self::N]) -> ();
    fn setx(&mut self, x: &[Self::N]) -> ();
}

pub trait ProblemDims {
    fn nx(&self) -> usize;
    fn na(&self) -> usize;
}

impl<T: Problem> ProblemDims for T {
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
}

