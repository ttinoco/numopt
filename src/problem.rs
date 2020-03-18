
use sprs::TriMat;
use num_traits::Float;

use super::utils::dot;

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

pub trait ProblemLp {
    type N: Float;
    fn x(&self) -> &[Self::N];
    fn c(&self) -> &[Self::N];
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn setx(&mut self, x: &[Self::N]) -> ();
}

/*
pub trait ProblemMILp<T: Float> {

}

pub trait ProblemQp<T: Float> {

}

pub trait ProblemNlp<T: Float> {

}
*/

impl<T: ProblemLp> Problem for T {
    type N = T::N;
    fn x(&self) -> &[Self::N] { self.x() }
    fn phi(&self) -> Self::N { dot(self.c(), self.x()) }
    fn gphi(&self) -> &[Self::N] { self.c() }
    fn a(&self) -> &TriMat<Self::N> { self.a() }
    fn b(&self) -> &[Self::N] { self.b() }
    fn l(&self) -> &[Self::N] { self.l() }
    fn u(&self) -> &[Self::N] { self.u() }
    fn eval(&mut self, x: &[Self::N]) -> () { self.setx(x); }
    fn setx(&mut self, x: &[Self::N]) -> () { self.setx(x); }
}

