use std::fs::File;
use std::fmt::LowerExp;
use std::io::BufWriter;
use std::io::prelude::*;
use sprs::{TriMat, CsMat};
use simple_error::SimpleError;
use num_traits::{Float, NumCast};

use crate::utils::dot;
use crate::problem::{Problem, ProblemDims};

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

        let mut pre: char;
        let mut j: usize;
        let mut d: T::N;
        let mut b: T::N;

        let f = match File::create(filename) {
            Ok(ff) => ff,
            Err(_e) => return Err(SimpleError::new("unable to create LP file")) 
        };

        let mut w = BufWriter::new(f);

        // Objective
        w.write("Minimize\n".as_bytes()).unwrap();
        w.write(" obj:\n".as_bytes()).unwrap();
        for (i, c) in self.c().iter().enumerate() {
            if c > &NumCast::from(0.).unwrap() {
                pre = '+';
            }
            else if c < &NumCast::from(0.).unwrap() {
                pre = '-';
            }
            else {
                continue;
            }
            if c.abs() == NumCast::from(1.).unwrap() {
                w.write(format!("     {} x_{}\n", pre, i).as_bytes()).unwrap();
            }
            else {
                w.write(format!("     {} {:.10e} x_{}\n", 
                                pre, 
                                c.abs(), i).as_bytes()).unwrap();
            }
        }

        // Constraints
        w.write("Subject to\n".as_bytes()).unwrap();
        let a: CsMat<T::N> = self.a().to_csr();
        for i in 0..a.rows() {
            b = self.b()[i];
            w.write(format!("  c_{}:\n", i).as_bytes()).unwrap();
            for k in a.indptr()[i]..a.indptr()[i+1] {
                j = a.indices()[k];
                d = a.data()[k];
                if d > NumCast::from(0.).unwrap() {
                    pre = '+';
                }
                else if d < NumCast::from(0.).unwrap() {
                    pre = '-';
                }
                else {
                    continue;
                }
                if d.abs() == NumCast::from(1.).unwrap() {
                    w.write(format!("     {} x_{}\n", pre, j).as_bytes()).unwrap();
                }
                else {
                    w.write(format!("     {} {:.10e} x_{}\n", 
                                    pre, 
                                    d.abs(), 
                                    j).as_bytes()).unwrap();
                }
            }
            w.write(format!("     = {:.10e}\n", b).as_bytes()).unwrap();
        }

        // Bounds
        w.write("Bounds\n".as_bytes()).unwrap();
        for i in 0..self.nx() {
            w.write(format!(" {:.10e} <= x_{} <= {:.10e}\n",
                            self.l()[i],
                            i,
                            self.u()[i]).as_bytes()).unwrap();
        }

        // General
        w.write("General\n".as_bytes()).unwrap();

        // End
        w.write("End\n".as_bytes()).unwrap();

        w.flush().unwrap();

        Ok(())
    }
}