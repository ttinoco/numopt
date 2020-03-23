use std::fmt;
use sprs::TriMat; 
use std::str::FromStr;
use num_traits::Float;

use crate::problem::ProblemMilp;

pub trait ProblemLp {
    type N: Float + FromStr + fmt::LowerExp + fmt::Debug;
    fn x(&self) -> &[Self::N];
    fn c(&self) -> &[Self::N];
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn setx(&mut self, x: &[Self::N]) -> ();
}

impl<T: ProblemLp> ProblemMilp for T {
    type N = T::N;
    fn x(&self) -> &[Self::N] { self.x() }
    fn c(&self) -> &[Self::N] { self.c() }
    fn a(&self) -> &TriMat<Self::N> { self.a() }
    fn b(&self) -> &[Self::N] { self.b() }
    fn l(&self) -> &[Self::N] { self.l() }
    fn u(&self) -> &[Self::N] { self.u() }
    fn p(&self) -> Option<&[bool]> { None }
    fn setx(&mut self, x: &[Self::N]) -> () { self.setx(x); }
}