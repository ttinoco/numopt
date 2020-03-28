use std::fs::File;
use num_traits::{Float, NumCast};
use std::io::{self, Write, BufWriter};
use sprs::{TriMat, TriMatBase, CsMat};
 
use crate::utils::dot;
use crate::problem::{Problem, 
                     ProblemFloat,
                     ProblemBase, 
                     ProblemDims};

pub struct ProblemMilp<T> {
    c: Vec<T>,
    base: Problem<T>,
}

pub trait ProblemMilpBase {
    type N: ProblemFloat;
    fn x(&self) -> &[Self::N];
    fn c(&self) -> &[Self::N];
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn p(&self) -> &[bool];
    fn base(&self) -> &Problem<Self::N>;
    fn base_mut(&mut self) -> &mut Problem<Self::N>;
}

pub trait ProblemMilpIO {
    type P: ProblemMilpBase;
    fn read_from_lp_file(filename: &str) -> io::Result<Self::P>;
    fn write_to_lp_file(&self, filename: &str) -> io::Result<()>;
}

impl<T: 'static + ProblemFloat> ProblemMilp<T> {
    pub fn new(c: Vec<T>,
               a: TriMat<T>,
               b: Vec<T>,  
               l: Vec<T>,
               u: Vec<T>, 
               p: Vec<bool>) -> Self {
        let cc = c.clone();
        let eval_fn = Box::new(move | phi: &mut T, 
                                      gphi: &mut Vec<T>, 
                                      _hphi: &mut TriMat<T>,
                                      _f: &mut Vec<T>,
                                      _j: &mut TriMat<T>,
                                      _h: &mut Vec<TriMat<T>>,
                                      x: &[T] | {
            *phi = dot(&c, x);
            gphi.copy_from_slice(&c);
        });
        let nx = a.cols();
        let base = Problem::new(TriMatBase::new((nx, nx)), // Hphi
                                a, 
                                b,
                                TriMatBase::new((0, nx)),  // J
                                Vec::new(), 
                                l, 
                                u, 
                                p, 
                                eval_fn);
        Self {
            c: cc,
            base: base,
        }
    }
}

impl<N: ProblemFloat> ProblemMilpBase for ProblemMilp<N> {
    type N = N;
    fn x(&self) -> &[N] { &self.base.x() }
    fn c(&self) -> &[N] { &self.c }
    fn a(&self) -> &TriMat<N> { &self.base.a() } 
    fn b(&self) -> &[N] { &self.base.b() }
    fn l(&self) -> &[N] { &self.base.l() }
    fn u(&self) -> &[N] { &self.base.u() }
    fn p(&self) -> &[bool] { self.base.p() }
    fn base(&self) -> &Problem<Self::N> { &self.base }
    fn base_mut(&mut self) -> &mut Problem<Self::N> { &mut self.base }
}

impl<N: ProblemFloat> ProblemBase for ProblemMilp<N> {
    type N = N;
    fn x(&self) -> &[N] { self.base.x() }
    fn phi(&self) -> N { self.base().phi() }
    fn gphi(&self) -> &[N] { self.base().gphi() }
    fn hphi(&self) -> &TriMat<N> { self.base().hphi() }
    fn a(&self) -> &TriMat<N> { self.base.a() }
    fn b(&self) -> &[N] { self.base.b() }
    fn f(&self) -> &[N] { self.base().f() }
    fn j(&self) -> &TriMat<N> { self.base().j() }
    fn h(&self) -> &Vec<TriMat<N>> { self.base().h() }
    fn hcomb(&self) -> &TriMat<N> { self.base().hcomb() }
    fn l(&self) -> &[N] { self.base.l() }
    fn u(&self) -> &[N] { self.base.u() }
    fn p(&self) -> &[bool] { self.base.p() }
    fn evaluate(&mut self, x: &[N]) -> () { self.base_mut().evaluate(x) }
    fn combine_h(&mut self, _nu: &[N]) -> () {}
}

impl<T: ProblemMilpBase + ProblemDims> ProblemMilpIO for T {
    
    type P = T;

    fn read_from_lp_file(_filename: &str) -> io::Result<Self::P> {

        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }

    fn write_to_lp_file(&self, filename: &str) -> io::Result<()> {

        let mut pre: char;
        let mut j: usize;
        let mut d: T::N;
        let mut b: T::N;

        let f = File::create(filename)?;

        let mut w = BufWriter::new(f);

        // Objective
        w.write("Minimize\n".as_bytes())?;
        w.write(" obj:\n".as_bytes())?;
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
        let a: CsMat<T::N> = self.a().to_csr();
        for i in 0..a.rows() {
            b = self.b()[i];
            w.write(format!("  c_{}:\n", i).as_bytes())?;
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