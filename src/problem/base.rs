use std::fmt::{self, Debug};

use crate::matrix::CooMat; 
use crate::problem::minlp::ProblemMinlp;
use crate::problem::milp::ProblemMilp;
use crate::problem::lp::ProblemLp;
use crate::problem::nlp::ProblemNlp;

pub enum Problem {
    Minlp(ProblemMinlp),
    Milp(ProblemMilp),
    Lp(ProblemLp),
    Nlp(ProblemNlp),
}

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