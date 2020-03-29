use std::ops::{Mul,
               AddAssign};
use std::str::FromStr;
use num_traits::{Float, NumCast};
use std::fmt::{self, LowerExp, Debug};

use crate::matrix::CooMat;

pub trait ProblemFloat: Float + 
                        FromStr + 
                        LowerExp + 
                        Debug + 
                        Mul + 
                        AddAssign {}

pub type ProblemEval<T> = Box<dyn Fn(&mut T,              // phi
                                     &mut Vec<T>,         // gphi
                                     &mut CooMat<T>,      // Hphi
                                     &mut Vec<T>,         // f
                                     &mut CooMat<T>,      // J
                                     &mut Vec<CooMat<T>>, // H
                                     &[T]                 // x
                                    ) -> ()>;

pub struct Problem<T: ProblemFloat> 
{
    x: Vec<T>,
    
    phi: T,
    gphi: Vec<T>,
    hphi: CooMat<T>,  // lower triangular
    
    a: CooMat<T>,
    b: Vec<T>,
    
    f: Vec<T>,
    j: CooMat<T>,
    h: Vec<CooMat<T>>,
    hcomb: CooMat<T>, // lower triangular
    
    l: Vec<T>,
    u: Vec<T>,
    
    p: Vec<bool>,
    
    eval_fn: ProblemEval<T>,
}

pub trait ProblemBase {
    type N: ProblemFloat;
    fn x(&self) -> &[Self::N];
    fn phi(&self) -> Self::N;
    fn gphi(&self) -> &[Self::N];
    fn hphi(&self) -> &CooMat<Self::N>;
    fn a(&self) -> &CooMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn f(&self) -> &[Self::N];
    fn j(&self) -> &CooMat<Self::N>;
    fn h(&self) -> &Vec<CooMat<Self::N>>;
    fn hcomb(&self) -> &CooMat<Self::N>; 
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn p(&self) -> &[bool];
    fn evaluate(&mut self, x: &[Self::N]) -> ();
    fn combine_h(&mut self, nu: &[Self::N]) -> ();
}

pub trait ProblemDims {
    fn nx(&self) -> usize;
    fn na(&self) -> usize;
    fn nf(&self) -> usize;
}

pub struct ProblemSol<T: ProblemFloat> {
    pub x: Vec<T>,
    pub lam: Vec<T>,
    pub nu: Vec<T>,
    pub mu: Vec<T>,
    pub pi: Vec<T>,
}

impl<T: Float + FromStr + LowerExp + Debug + Mul + AddAssign> ProblemFloat for T { }

impl<T: ProblemFloat> Problem<T> 
{
    pub fn new(hphi: CooMat<T>, 
               a: CooMat<T>, 
               b: Vec<T>,
               j: CooMat<T>,
               h: Vec<CooMat<T>>,  
               l: Vec<T>, 
               u: Vec<T>, 
               p: Vec<bool>,
               eval_fn: ProblemEval<T>) -> Self {

        let nx = a.cols();
        let na = a.rows();
        let nf = j.rows();

        assert_eq!(hphi.cols(), nx);
        assert_eq!(hphi.rows(), nx);

        assert_eq!(a.cols(), nx);
        assert_eq!(a.rows(), na);
        assert_eq!(b.len(), na);

        assert_eq!(j.cols(), nx);
        assert_eq!(j.rows(), nf);
        assert_eq!(h.len(), nf);
        for hh in h.iter() {
            assert_eq!(hh.rows(), nx);
            assert_eq!(hh.cols(), nx);
        }

        assert_eq!(l.len(), nx);
        assert_eq!(u.len(), nx);

        assert_eq!(p.len(), nx);

        let mut k: usize = 0;
        let hcomb_nnz = h.iter().map(|h| h.nnz()).sum();
        let mut hcomb: CooMat<T> = CooMat::from_nnz((nx, nx), hcomb_nnz);
        for hh in h.iter() {
            for (row, col, _val) in hh.iter() {
                hcomb.set_row_ind(k, row);
                hcomb.set_col_ind(k, col);
                k += 1;
            }
        }
        
        Self {
            x: vec![NumCast::from(0.).unwrap();nx],
            phi: NumCast::from(0.).unwrap(),
            gphi: vec![NumCast::from(0.).unwrap();nx],
            hphi: hphi,
            a: a,
            b: b,
            f: vec![NumCast::from(0.).unwrap();nf],
            j: j,
            h: h,
            hcomb: hcomb,
            l: l,
            u: u,
            p: p,
            eval_fn: eval_fn
        }
    }
}

impl<N: ProblemFloat> ProblemBase for Problem<N> {

    type N = N;
    fn x(&self) -> &[N] { &self.x }
    fn phi(&self) -> N { self.phi }
    fn gphi(&self) -> &[N] { &self.gphi }
    fn hphi(&self) -> &CooMat<N> { &self.hphi }
    fn a(&self) -> &CooMat<N> { &self.a } 
    fn b(&self) -> &[N] { &self.b }
    fn f(&self) -> &[N] { &self.f }
    fn j(&self) -> &CooMat<N> { &self.j } 
    fn h(&self) -> &Vec<CooMat<N>> { &self.h } 
    fn hcomb(&self) -> &CooMat<N> { &self.hcomb }
    fn l(&self) -> &[N] { &self.l }
    fn u(&self) -> &[N] { &self.u }
    fn p(&self) -> &[bool] { &self.p } 
    
    fn evaluate(&mut self, x: &[N]) -> () {
        (self.eval_fn)(&mut self.phi, 
                       &mut self.gphi,
                       &mut self.hphi,
                       &mut self.f,
                       &mut self.j,
                       &mut self.h,
                       x)
    }

    fn combine_h(&mut self, nu: &[N]) -> () {

        assert_eq!(self.nf(), nu.len());
 
        let mut k: usize = 0;
        for (h, nuval) in self.h.iter().zip(nu.iter()) {
            for val in h.data().iter() {
                self.hcomb.set_data(k, (*nuval)*(*val));
                k += 1;
            }
        }    
    }
}

impl<T: ProblemBase> ProblemDims for T {
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
    fn nf(&self) -> usize { self.f().len() }
}

impl<T: ProblemFloat> ProblemSol<T> {
    pub fn new(nx: usize, na: usize, nf: usize) -> Self {
        let z = NumCast::from(0.).unwrap();
        Self {
            x: vec![z;nx],
            lam: vec![z;na],
            nu: vec![z;nf],
            mu: vec![z;nx],
            pi: vec![z;nx]
        }
    }
}

impl<T: ProblemFloat> Debug for ProblemSol<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProblemSol")
         .field("x", &self.x)
         .field("lam", &self.lam)
         .field("nu", &self.nu)
         .field("mu", &self.mu)
         .field("pi", &self.pi)
         .finish()
    }
}