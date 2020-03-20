use sprs::TriMat;
use num_traits::Float;
use simple_error::SimpleError;

use crate::utils::dot;
use crate::problem::Problem;

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

pub trait ProblemLpWriter {
    fn write_to_lp_file(&self, filename: &str) -> Result<(), SimpleError>;
}

impl<T: ProblemLp> ProblemLpWriter for T {
    
    fn write_to_lp_file(&self, filename: &str) -> Result<(), SimpleError> {

        println!("Problem write to LP file");

        Ok(())
    }
}