mod lp;

use sprs::TriMat;
use num_traits::{Float, NumCast};

pub use crate::problem::lp::{ProblemLp,
                             ProblemLpIO};

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

pub struct ProblemSol<T: Problem> {
    x: Vec<T::N>,
    lam: Vec<T::N>,
    mu: Vec<T::N>,
    pi: Vec<T::N>,
}

impl<T: Problem> ProblemDims for T {
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
}

impl<T: Problem> ProblemSol<T> {
    pub fn new(nx: usize, na: usize) -> Self {
        let z = NumCast::from(0.).unwrap();
        Self {
            x: vec![z;nx],
            lam: vec![z;na],
            mu: vec![z;nx],
            pi: vec![z;nx]
        }
    }
}

