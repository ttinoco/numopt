use std::fs::File;
use std::io::{self, Write, BufWriter};
 
use crate::vector::dot;
use crate::matrix::{CooMat,
                    CsrMat};
use crate::problem::{Problem, 
                     ProblemBase}; 

pub struct ProblemMilp {
    c: Vec<f64>,
    base: Problem,
}

pub trait ProblemMilpBase {
    fn x0(&self) -> Option<&[f64]>;
    fn c(&self) -> &[f64];
    fn a(&self) -> &CooMat;
    fn b(&self) -> &[f64];
    fn l(&self) -> &[f64];
    fn u(&self) -> &[f64];
    fn p(&self) -> &[bool];
    fn base(&self) -> &Problem;
    fn base_mut(&mut self) -> &mut Problem;
    fn nx(&self) -> usize { self.c().len() }
    fn na(&self) -> usize { self.b().len() }
}

pub trait ProblemMilpIO {
    type P: ProblemMilpBase;
    fn read_from_lp_file(filename: &str) -> io::Result<Self::P>;
    fn write_to_lp_file(&self, filename: &str) -> io::Result<()>;
}

impl ProblemMilp {
    pub fn new(c: Vec<f64>,
               a: CooMat,
               b: Vec<f64>,  
               l: Vec<f64>,
               u: Vec<f64>, 
               p: Vec<bool>,
               x0: Option<Vec<f64>>) -> Self {
        let cc = c.clone();
        let eval_fn = Box::new(move | phi: &mut f64, 
                                      gphi: &mut Vec<f64>, 
                                      _hphi: &mut CooMat,
                                      _f: &mut Vec<f64>,
                                      _j: &mut CooMat,
                                      _h: &mut Vec<CooMat>,
                                      x: &[f64] | {
            *phi = dot(&c, x);
            gphi.copy_from_slice(&c);
        });
        let nx = a.cols();
        let base = Problem::new(CooMat::from_nnz((nx, nx), 0), // Hphi
                                a, 
                                b,
                                CooMat::from_nnz((0, nx), 0),  // J
                                Vec::new(), 
                                l, 
                                u, 
                                p, 
                                x0,
                                eval_fn);
        Self {
            c: cc,
            base: base,
        }
    }
}

impl ProblemMilpBase for ProblemMilp {
    fn x0(&self) -> Option<&[f64]> { self.base.x0() }
    fn c(&self) -> &[f64] { &self.c }
    fn a(&self) -> &CooMat { &self.base.a() } 
    fn b(&self) -> &[f64] { &self.base.b() }
    fn l(&self) -> &[f64] { &self.base.l() }
    fn u(&self) -> &[f64] { &self.base.u() }
    fn p(&self) -> &[bool] { self.base.p() }
    fn base(&self) -> &Problem { &self.base }
    fn base_mut(&mut self) -> &mut Problem { &mut self.base }
}

impl ProblemBase for ProblemMilp {
    fn x0(&self) -> Option<&[f64]> { self.base.x0() }
    fn phi(&self) -> f64 { self.base().phi() }
    fn gphi(&self) -> &[f64] { self.base().gphi() }
    fn hphi(&self) -> &CooMat { self.base().hphi() }
    fn a(&self) -> &CooMat { self.base.a() }
    fn b(&self) -> &[f64] { self.base.b() }
    fn f(&self) -> &[f64] { self.base().f() }
    fn j(&self) -> &CooMat { self.base().j() }
    fn h(&self) -> &Vec<CooMat> { self.base().h() }
    fn hcomb(&self) -> &CooMat { self.base().hcomb() }
    fn l(&self) -> &[f64] { self.base.l() }
    fn u(&self) -> &[f64] { self.base.u() }
    fn p(&self) -> &[bool] { self.base.p() }
    fn evaluate(&mut self, x: &[f64]) -> () { self.base_mut().evaluate(x) }
    fn combine_h(&mut self, _nu: &[f64]) -> () {}
}

impl<T: ProblemMilpBase> ProblemMilpIO for T {
    
    type P = T;

    fn read_from_lp_file(_filename: &str) -> io::Result<Self::P> {

        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }

    fn write_to_lp_file(&self, filename: &str) -> io::Result<()> {

        let mut pre: char;
        let mut j: usize;
        let mut d: f64;
        let mut b: f64;

        let f = File::create(filename)?;

        let mut w = BufWriter::new(f);

        // Objective
        w.write("Minimize\n".as_bytes())?;
        w.write(" obj:\n".as_bytes())?;
        for (i, c) in self.c().iter().enumerate() {
            if *c > 0. {
                pre = '+';
            }
            else if *c < 0. {
                pre = '-';
            }
            else {
                continue;
            }
            if c.abs() == 1. {
                w.write(format!("     {} x_{}\n", pre, i).as_bytes())?;
            }
            else {
                w.write(format!("     {} {:.10e} x_{}\n", 
                                pre, 
                                c.abs(), i).as_bytes())?;
            }
        }

        // Constraints
        w.write("Subject to\n".as_bytes())?;
        let mut a: CsrMat = self.a().to_csr();
        a.sum_duplicates();
        for i in 0..a.rows() {
            b = self.b()[i];
            w.write(format!("  c_{}:\n", i).as_bytes())?;
            for k in a.indptr()[i]..a.indptr()[i+1] {
                j = a.indices()[k];
                d = a.data()[k];
                if d > 0. {
                    pre = '+';
                }
                else if d < 0. {
                    pre = '-';
                }
                else {
                    continue;
                }
                if d.abs() == 1. {
                    w.write(format!("     {} x_{}\n", pre, j).as_bytes())?;
                }
                else {
                    w.write(format!("     {} {:.10e} x_{}\n", 
                                    pre, 
                                    d.abs(), 
                                    j).as_bytes())?;
                }
            }
            w.write(format!("     = {:.10e}\n", b).as_bytes())?;
        }

        // Bounds
        w.write("Bounds\n".as_bytes())?;
        for i in 0..self.nx() {
            w.write(format!(" {:.10e} <= x_{} <= {:.10e}\n",
                            self.l()[i],
                            i,
                            self.u()[i]).as_bytes())?;
        }

        // General
        w.write("General\n".as_bytes())?;
        for (i,f) in self.p().iter().enumerate() {
            if *f {
                w.write(format!(" x_{}\n", i).as_bytes())?;
            }
        }

        // End
        w.write("End\n".as_bytes())?;

        w.flush()?;

        Ok(())
    }
}