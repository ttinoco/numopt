use std::fmt::{self, Debug};

use crate::matrix::CooMat; 

/// Type that represents the evaluation function
/// of an optimization problem.
pub type ProblemEval = Box<dyn Fn(&mut f64,              // phi
                                  &mut Vec<f64>,         // gphi
                                  &mut CooMat<f64>,      // Hphi
                                  &mut Vec<f64>,         // f
                                  &mut CooMat<f64>,      // J
                                  &mut Vec<CooMat<f64>>, // H
                                  &[f64]                 // x
                                 ) -> ()>;

/// "General" mixed-integer nonlinear optimization problem (Minlp).
pub struct Problem 
{
    x0: Option<Vec<f64>>,

    phi: f64,
    gphi: Vec<f64>,
    hphi: CooMat<f64>,  // lower triangular
    
    a: CooMat<f64>,
    b: Vec<f64>,
    
    f: Vec<f64>,
    j: CooMat<f64>,
    h: Vec<CooMat<f64>>,
    hcomb: CooMat<f64>, // lower triangular
    
    l: Vec<f64>,
    u: Vec<f64>,
    
    p: Vec<bool>,
    
    eval_fn: ProblemEval,
}

/// A trait for "general" mixed-integer nonlinear optimization 
/// problems (Minlp) of the form
/// ```ignore
/// minimize   phi(x)
/// subject to a*x = b
///            f(x) = 0
///            l <= x <= u
///            p*x in integers
/// ```
pub trait ProblemBase {

    /// Initial point.
    fn x0(&self) -> Option<&[f64]>;

    /// Objective function value.
    fn phi(&self) -> f64;

    /// Objective function gradient value.
    fn gphi(&self) -> &[f64];

    /// Objective function Hessian value (lower triangular part)
    fn hphi(&self) -> &CooMat<f64>;

    /// Jacobian matrix of linear equality constraints.
    fn a(&self) -> &CooMat<f64>;

    /// Right-hand-side vector of linear equality constraints.
    fn b(&self) -> &[f64];

    /// Nonlinear equality constraint function value.
    fn f(&self) -> &[f64];
    
    /// Nonlinear equality constraint function Jacobian value.
    fn j(&self) -> &CooMat<f64>;

    /// Vector of nonlinear equality constraint function Hessian values
    /// (lower triangular parts).
    fn h(&self) -> &Vec<CooMat<f64>>;

    /// Linear combination of nonlinear equality constraint function Hessian values
    /// (lower triangular part).
    fn hcomb(&self) -> &CooMat<f64>;
    
    /// Vector of optimization variable lower limits.
    fn l(&self) -> &[f64];

    /// Vector of optimization variable upper limits.
    fn u(&self) -> &[f64];

    /// Vector of boolean values indicating optimization variables that are constrained
    /// to be integers.
    fn p(&self) -> &[bool];

    /// Function that evaluates objective function, nonlinear equality constraint
    /// functions, and their derivatives for a given vector of optimization variable values.
    fn evaluate(&mut self, x: &[f64]) -> ();

    /// Function that forms a linear combination of nonlinear equality constraint
    /// function Hessians.
    fn combine_h(&mut self, nu: &[f64]) -> ();

    /// Number of optimization variables.
    fn nx(&self) -> usize { self.gphi().len() }

    /// Number of linear equality constraints.
    fn na(&self) -> usize { self.b().len() }

    /// Number of nonlinear equality constraints.
    fn nf(&self) -> usize { self.f().len() }
}

/// Optimization problem solution.
pub struct ProblemSol {

    /// Primal variable values.
    pub x: Vec<f64>,

    /// Dual variable values corresponding to linear equality constraints.
    pub lam: Vec<f64>,

    /// Dual variable values corresponding to nonlinear equality constraints.
    pub nu: Vec<f64>,

    /// Dual variable values corresponding to variable upper limits.
    pub mu: Vec<f64>,

    /// Dual variable values corresponding to variable lower limits.
    pub pi: Vec<f64>,
}

impl Problem {

    /// Creates new "general" mixed-integer nonlinear optimization 
    /// problem (Minlp).
    pub fn new(hphi: CooMat<f64>, 
               a: CooMat<f64>, 
               b: Vec<f64>,
               j: CooMat<f64>,
               h: Vec<CooMat<f64>>,  
               l: Vec<f64>, 
               u: Vec<f64>, 
               p: Vec<bool>,
               x0: Option<Vec<f64>>,
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
            x0: x0,
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

    fn x0(&self) -> Option<&[f64]> { 
        match &self.x0 { 
            Some(xx) => Some(&xx),
            None => None
        }
    }
    fn phi(&self) -> f64 { self.phi }
    fn gphi(&self) -> &[f64] { &self.gphi }
    fn hphi(&self) -> &CooMat<f64> { &self.hphi }
    fn a(&self) -> &CooMat<f64> { &self.a } 
    fn b(&self) -> &[f64] { &self.b }
    fn f(&self) -> &[f64] { &self.f }
    fn j(&self) -> &CooMat<f64> { &self.j } 
    fn h(&self) -> &Vec<CooMat<f64>> { &self.h } 
    fn hcomb(&self) -> &CooMat<f64> { &self.hcomb }
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

    /// Creates new (empty) container for solution associated with 
    /// optimization problem of specific dimensions.
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