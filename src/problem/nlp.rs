
use sprs::{TriMat, TriMatBase};

use crate::problem::{Problem, 
                     ProblemFloat,
                     ProblemBase};

pub struct ProblemNlp<T> {
    base: Problem<T>,
}

pub trait ProblemNlpBase {
    type N: ProblemFloat;
    fn x(&self) -> &[Self::N];
    fn phi(&self) -> Self::N;
    fn gphi(&self) -> &[Self::N];
    fn hphi(&self) -> &TriMat<Self::N>;
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn f(&self) -> &[Self::N];
    fn j(&self) -> &TriMat<Self::N>;
    fn h(&self) -> &Vec<TriMat<Self::N>>;
    fn hcomb(&self) -> &TriMat<Self::N>; 
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn evaluate(&mut self, x: &[Self::N]) -> ();
    fn combine_h(&mut self, nu: &[Self::N]) -> ();
}

