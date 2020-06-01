use ndarray::ArrayView1;

use crate::matrix::coo::CooMat;
use crate::problem::minlp::ProblemMinlp;
use crate::problem::milp::ProblemMilp;
use crate::problem::nlp::ProblemNlp;

/// Linear optimization problem (Lp).                     
pub struct ProblemLp {
    base_milp: ProblemMilp,
    base_nlp: ProblemNlp,
}

/// A trait for linear optimization 
/// problems (Lp) of the form
/// ```ignore
/// minimize   c^T*x
/// subject to a*x = b
///            l <= x <= u
/// ```
// pub trait ProblemLpBase {

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

//     /// A reference to the problem as an Milp problem.
//     fn base(&self) -> &ProblemMilp;

//     /// A mutable reference to the problem as an Milp problem.
//     fn base_mut(&mut self) -> &mut ProblemMilp;

//     /// Number of optimization variables.
//     fn nx(&self) -> usize { self.c().len() }

//     /// Number of linear equality cosntraints.
//     fn na(&self) -> usize { self.b().len() }
// }

impl ProblemLp {

    /// Creates a new linear optimization problem (Lp).
    pub fn new(c: Vec<f64>,
               a: CooMat<f64>,
               b: Vec<f64>,  
               l: Vec<f64>,
               u: Vec<f64>,
               x0: Option<Vec<f64>>) -> Self {
        let nx = c.len();
        let base_milp = ProblemMilp::new(c.clone(), 
                                         a.clone(), 
                                         b.clone(), 
                                         l.clone(), 
                                         u.clone(), 
                                         vec![false; nx], 
                                         x0.clone());
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
        let base_nlp = ProblemNlp::new(CooMat::from_nnz((nx, nx), 0),
                                       a.clone(), 
                                       b.clone(),
                                       CooMat::from_nnz((0, nx), 0),
                                       Vec::new(), 
                                       l.clone(), 
                                       u.clone(), 
                                       x0.clone(),
                                       eval_fn);
        Self {
            base_milp: base_milp,
            base_nlp: base_nlp,
        }
    }

    pub fn x0(&self) -> Option<&[f64]> { self.base_milp.x0() }
    pub fn c(&self) -> &[f64] { self.base_milp.c() }
    pub fn a(&self) -> &CooMat<f64> { self.base_milp.a() } 
    pub fn b(&self) -> &[f64] { self.base_milp.b() }
    pub fn l(&self) -> &[f64] { self.base_milp.l() }
    pub fn u(&self) -> &[f64] { self.base_milp.u() }
    pub fn nx(&self) -> usize { self.c().len() }
    pub fn na(&self) -> usize { self.b().len() }
    
    pub fn as_mut_milp(&mut self) -> &mut ProblemMilp { &mut self.base_milp }
    pub fn as_mut_minlp(&mut self) -> &mut ProblemMinlp { self.base_milp.as_mut_minlp() }   
    pub fn as_mut_nlp(&mut self) -> &mut ProblemNlp { &mut self.base_nlp }      
}