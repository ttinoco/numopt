use std::fmt::{self, Debug};

use crate::matrix::CooMat;

pub type ProblemEval = Box<dyn Fn(&mut f64,         // phi
                                  &mut Vec<f64>,    // gphi
                                  &mut CooMat,      // Hphi
                                  &mut Vec<f64>,    // f
                                  &mut CooMat,      // J
                                  &mut Vec<CooMat>, // H
                                  &[f64]            // x
                                 ) -> ()>;

pub struct Problem 
{
    x: Vec<f64>,
    
    phi: f64,
    gphi: Vec<f64>,
    hphi: CooMat,  // lower triangular
    
    a: CooMat,
    b: Vec<f64>,
    
    f: Vec<f64>,
    j: CooMat,
    h: Vec<CooMat>,
    hcomb: CooMat, // lower triangular
    
    l: Vec<f64>,
    u: Vec<f64>,
    
    p: Vec<bool>,
    
    eval_fn: ProblemEval,
}

pub trait ProblemBase {
    fn x(&self) -> &[f64];
    fn phi(&self) -> f64;
    fn gphi(&self) -> &[f64];
    fn hphi(&self) -> &CooMat;
    fn a(&self) -> &CooMat;
    fn b(&self) -> &[f64];
    fn f(&self) -> &[f64];
    fn j(&self) -> &CooMat;
    fn h(&self) -> &Vec<CooMat>;
    fn hcomb(&self) -> &CooMat; 
    fn l(&self) -> &[f64];
    fn u(&self) -> &[f64];
    fn p(&self) -> &[bool];
    fn evaluate(&mut self, x: &[f64]) -> ();
    fn combine_h(&mut self, nu: &[f64]) -> ();
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
    fn nf(&self) -> usize { self.f().len() }
}

pub struct ProblemSol {
    pub x: Vec<f64>,
    pub lam: Vec<f64>,
    pub nu: Vec<f64>,
    pub mu: Vec<f64>,
    pub pi: Vec<f64>,
}

impl Problem {
    pub fn new(hphi: CooMat, 
               a: CooMat, 
               b: Vec<f64>,
               j: CooMat,
               h: Vec<CooMat>,  
               l: Vec<f64>, 
               u: Vec<f64>, 
               p: Vec<bool>,
               eval_fn: ProblemEval) -> Self {

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
        let mut hcomb = CooMat::from_nnz((nx, nx), hcomb_nnz);
        for hh in h.iter() {
            for (row, col, _val) in hh.iter() {
                hcomb.set_row_ind(k, row);
                hcomb.set_col_ind(k, col);
                k += 1;
            }
        }
        
        Self {
            x: vec![0.;nx],
            phi: 0.,
            gphi: vec![0.;nx],
            hphi: hphi,
            a: a,
            b: b,
            f: vec![0.;nf],
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

impl ProblemBase for Problem {

    fn x(&self) -> &[f64] { &self.x }
    fn phi(&self) -> f64 { self.phi }
    fn gphi(&self) -> &[f64] { &self.gphi }
    fn hphi(&self) -> &CooMat { &self.hphi }
    fn a(&self) -> &CooMat { &self.a } 
    fn b(&self) -> &[f64] { &self.b }
    fn f(&self) -> &[f64] { &self.f }
    fn j(&self) -> &CooMat { &self.j } 
    fn h(&self) -> &Vec<CooMat> { &self.h } 
    fn hcomb(&self) -> &CooMat { &self.hcomb }
    fn l(&self) -> &[f64] { &self.l }
    fn u(&self) -> &[f64] { &self.u }
    fn p(&self) -> &[bool] { &self.p } 
    
    fn evaluate(&mut self, x: &[f64]) -> () {
        (self.eval_fn)(&mut self.phi, 
                       &mut self.gphi,
                       &mut self.hphi,
                       &mut self.f,
                       &mut self.j,
                       &mut self.h,
                       x)
    }

    fn combine_h(&mut self, nu: &[f64]) -> () {

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

impl ProblemSol {
    pub fn new(nx: usize, na: usize, nf: usize) -> Self {
        Self {
            x: vec![0.;nx],
            lam: vec![0.;na],
            nu: vec![0.;nf],
            mu: vec![0.;nx],
            pi: vec![0.;nx]
        }
    }
}

impl Debug for ProblemSol {

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