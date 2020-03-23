use std::fmt;
use sprs::TriMat;
use std::str::FromStr;
use num_traits::{Float, NumCast};

pub trait Problem {
    type N: Float + FromStr + fmt::LowerExp + fmt::Debug;
    fn x(&self) -> &[Self::N];
    fn phi(&self) -> Self::N;
    fn gphi(&self) -> &[Self::N];
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn p(&self) -> Option<&[bool]>;
    fn eval(&mut self, x: &[Self::N]) -> ();
    fn setx(&mut self, x: &[Self::N]) -> ();
}

pub trait ProblemDims {
    fn nx(&self) -> usize;
    fn na(&self) -> usize;
}

pub struct ProblemSol<T: Problem> {
    pub x: Vec<T::N>,
    pub lam: Vec<T::N>,
    pub mu: Vec<T::N>,
    pub pi: Vec<T::N>,
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

impl<T: Problem> fmt::Debug for ProblemSol<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProblemSol")
         .field("x", &self.x)
         .field("lam", &self.lam)
         .field("mu", &self.mu)
         .field("pi", &self.pi)
         .finish()
    }
}