use std::fs::File;
use std::io::{self, Write, BufWriter};
use ndarray::ArrayView1;

use crate::matrix::coo::CooMat;
use crate::problem::minlp::ProblemMinlp;

/// Mixed-integer linear optimization problem (Milp).
pub struct ProblemMilp {
    c: Vec<f64>,
    base: ProblemMinlp,
}

/// A trait for mixed-integer linear optimization 
/// problems (Milp) of the form
/// ```ignore
/// minimize   c^T*x
/// subject to a*x = b
///            l <= x <= u
///            p*x in integers
/// ```
// pub trait ProblemMilpBase {

//     /// Initial point.
//     fn x0(&self) -> Option<&[f64]>;

//     /// Objective function gradient.
//     fn c(&self) -> &[f64];

//     /// Jacobian matrix of linear equality constraints.
//     fn a(&self) -> &CooMat<f64>;

//     /// Right-hand-side vector of linear equality constraints.
//     fn b(&self) -> &[f64];

//     /// Vector of optimization variable lower limits.
//     fn l(&self) -> &[f64];

//     /// Vector of optimization variable upper limits.
//     fn u(&self) -> &[f64];

//     /// Vector of boolean values indicating optimization variables that are constrained
//     /// to be integers.
//     fn p(&self) -> &[bool];

//     /// A reference to the problem as an Minlp problem.
//     fn base(&self) -> &ProblemMinlp;

//     /// A mutable reference to the problem as an Minlp problem.
//     fn base_mut(&mut self) -> &mut ProblemMinlp;

//     /// Number of optimization variables.
//     fn nx(&self) -> usize { self.c().len() }

//     /// Number of linear equality cosntraints.
//     fn na(&self) -> usize { self.b().len() }
// }

/// A trait for reading and writing mixed-integer linear 
/// optimization problems (Milp).
pub trait ProblemMilpIO {

    /// Reads problem from LP file.
    fn read_from_lp_file(filename: &str) -> io::Result<ProblemMilp>;

    /// Writes problem to LP file.
    fn write_to_lp_file(&self, filename: &str) -> io::Result<()>;
}

impl ProblemMilp {

    /// Creates new mixed-integer linear optimization
    /// problem (Milp).
    pub fn new(c: Vec<f64>,
               a: CooMat<f64>,
               b: Vec<f64>,  
               l: Vec<f64>,
               u: Vec<f64>, 
               p: Vec<bool>,
               x0: Option<Vec<f64>>) -> Self {
        let cc = c.clone();
        let eval_fn = Box::new(move | phi: &mut f64, 
                                      gphi: &mut Vec<f64>, 
                                      _hphi: &mut CooMat<f64>,
                                      _f: &mut Vec<f64>,
                                      _j: &mut CooMat<f64>,
                                      _h: &mut Vec<CooMat<f64>>,
                                      x: &[f64] | {
            *phi = ArrayView1::from(&c).dot(&ArrayView1::from(x));
            gphi.copy_from_slice(&c);
        });
        let nx = a.cols();
        let base = ProblemMinlp::new(CooMat::from_nnz((nx, nx), 0), // Hphi
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

    pub fn x0(&self) -> Option<&[f64]> { self.base.x0() }
    pub fn c(&self) -> &[f64] { &self.c }
    pub fn a(&self) -> &CooMat<f64> { &self.base.a() } 
    pub fn b(&self) -> &[f64] { &self.base.b() }
    pub fn l(&self) -> &[f64] { &self.base.l() }
    pub fn u(&self) -> &[f64] { &self.base.u() }
    pub fn p(&self) -> &[bool] { self.base.p() }
    pub fn nx(&self) -> usize { self.c().len() }
    pub fn na(&self) -> usize { self.b().len() }

    pub fn as_mut_minlp(&mut self) -> &mut ProblemMinlp { &mut self.base }
    
    // pub fn to_minlp(&self) -> ProblemMinlp { 
    //     let c = self.c().to_vec();
    //     let eval_fn = Box::new(move | phi: &mut f64, 
    //                                   gphi: &mut Vec<f64>, 
    //                                   _hphi: &mut CooMat<f64>,
    //                                   _f: &mut Vec<f64>,
    //                                   _j: &mut CooMat<f64>,
    //                                   _h: &mut Vec<CooMat<f64>>,
    //                                   x: &[f64] | {
    //         *phi = ArrayView1::from(&c).dot(&ArrayView1::from(x));
    //         gphi.copy_from_slice(&c);
    //     });
    //     let nx = self.a().cols();
    //     let x0: Option<Vec<f64>> = match self.x0() {
    //         Some(x) => Some(x.to_vec()),
    //         None => None,
    //     };
    //     ProblemMinlp::new(CooMat::from_nnz((nx, nx), 0),
    //                       self.a().clone(), 
    //                       self.b().to_vec(),
    //                       CooMat::from_nnz((0, nx), 0),
    //                       Vec::new(), 
    //                       self.l().to_vec(), 
    //                       self.u().to_vec(), 
    //                       self.p().to_vec(), 
    //                       x0,
    //                       eval_fn)
    // }
}

//impl ProblemMilpBase for ProblemMilp {
//
//}

// impl ProblemMinlpBase for ProblemMilp {
//     fn x0(&self) -> Option<&[f64]> { self.base.x0() }
//     fn phi(&self) -> f64 { self.base().phi() }
//     fn gphi(&self) -> &[f64] { self.base().gphi() }
//     fn hphi(&self) -> &CooMat<f64> { self.base().hphi() }
//     fn a(&self) -> &CooMat<f64> { self.base.a() }
//     fn b(&self) -> &[f64] { self.base.b() }
//     fn f(&self) -> &[f64] { self.base().f() }
//     fn j(&self) -> &CooMat<f64> { self.base().j() }
//     fn h(&self) -> &Vec<CooMat<f64>> { self.base().h() }
//     fn hcomb(&self) -> &CooMat<f64> { self.base().hcomb() }
//     fn l(&self) -> &[f64] { self.base.l() }
//     fn u(&self) -> &[f64] { self.base.u() }
//     fn p(&self) -> &[bool] { self.base.p() }
//     fn evaluate(&mut self, x: &[f64]) -> () { self.base_mut().evaluate(x) }
//     fn combine_h(&mut self, _nu: &[f64]) -> () {}
// }

impl ProblemMilpIO for ProblemMilp {
    
    fn read_from_lp_file(_filename: &str) -> io::Result<ProblemMilp> {

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
        let mut a = self.a().to_csr();
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