
use sprs::TriMat;
use num_traits::Float;

use super::utils::dot;

pub trait Problem<N: Float> {
    fn x(&self) -> &Vec<N>;
    fn phi(&self) -> N;
    fn gphi(&self) -> &Vec<N>;
    fn a(&self) -> &TriMat<N>;
    fn b(&self) -> &Vec<N>;
    fn l(&self) -> &Vec<N>;
    fn u(&self) -> &Vec<N>;
    fn eval(&mut self, x: &Vec<N>) -> ();
    fn setx(&mut self, x: &Vec<N>) -> ();
}

pub trait ProblemLp<N: Float> {
    fn x(&self) -> &Vec<N>;
    fn c(&self) -> &Vec<N>;
    fn a(&self) -> &TriMat<N>;
    fn b(&self) -> &Vec<N>;
    fn l(&self) -> &Vec<N>;
    fn u(&self) -> &Vec<N>;
    fn setx(&mut self, x: &Vec<N>) -> ();
}

/*
pub trait ProblemMILp<N: Float> {

}

pub trait ProblemNlp<N: Float> {

}
*/

impl<T: ProblemLp<N>, N: Float> Problem<N> for T {
    fn x(&self) -> &Vec<N> { self.x() }
    fn phi(&self) -> N { dot(self.c(), self.x()) }
    fn gphi(&self) -> &Vec<N> { self.c() }
    fn a(&self) -> &TriMat<N> { self.a() }
    fn b(&self) -> &Vec<N> { self.b() }
    fn l(&self) -> &Vec<N> { self.l() }
    fn u(&self) -> &Vec<N> { self.u() }
    fn eval(&mut self, x: &Vec<N>) -> () { self.setx(x); }
    fn setx(&mut self, x: &Vec<N>) -> () { self.setx(x); }
}

