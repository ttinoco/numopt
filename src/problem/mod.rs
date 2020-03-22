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
    x: Option<Vec<T::N>>,
    lam: Option<Vec<T::N>>,
    mu: Option<Vec<T::N>>,
    pi: Option<Vec<T::N>>,
}

impl<T: Problem> ProblemDims for T {
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
}

impl<T: Problem> ProblemSol<T> {
    pub fn new(nx: usize, na: usize) -> Self {
        let z = NumCast::from(0.).unwrap();
        Self {
            x: Some(vec![z;nx]),
            lam: Some(vec![z;na]),
            mu: Some(vec![z;nx]),
            pi: Some(vec![z;nx])
        }
    }
}

