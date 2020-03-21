use sprs::TriMat;
use std::fs::File;
use std::fmt::LowerExp;
use num_traits::{Float, NumCast};
use std::io::BufWriter;
use std::io::prelude::*;
use simple_error::SimpleError;

use crate::utils::dot;
use crate::problem::Problem;

pub trait ProblemLp {
    type N: Float + LowerExp;
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

        let f = match File::create(filename) {
            Ok(ff) => ff,
            Err(_e) => return Err(SimpleError::new("unable to create LP file")) 
        };

        let mut w = BufWriter::new(f);

        let mut pre: char;

        w.write("Minimize\n".as_bytes()).unwrap();
        w.write(" obj:\n".as_bytes()).unwrap();
        for (i, ci) in self.c().iter().enumerate() {
            if ci > &NumCast::from(0.).unwrap() {
                pre = '+';
            }
            else if ci < &NumCast::from(0.).unwrap() {
                pre = '-';
            }
            else {
                continue;
            }
            if ci.abs() == NumCast::from(1.).unwrap() {
                w.write(format!("     {} x_{}\n", pre, i).as_bytes()).unwrap();
            }
            else {
                w.write(format!("     {} {:.10e} x_{}\n", 
                                pre, 
                                ci.abs(), i).as_bytes()).unwrap();
            }
        }

        w.flush().unwrap();

        Ok(())
    }
}