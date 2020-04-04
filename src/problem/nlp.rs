
use crate::matrix::CooMat;

use crate::problem::{Problem, 
                     ProblemEval,
                     ProblemBase};

pub struct ProblemNlp {
    base: Problem,
}

pub trait ProblemNlpBase {
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
    fn evaluate(&mut self, x: &[f64]) -> ();
    fn combine_h(&mut self, nu: &[f64]) -> ();
    fn base(&self) -> &Problem;
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
    fn nf(&self) -> usize { self.f().len() }
}

impl ProblemNlp {
    pub fn new(hphi: CooMat, 
               a: CooMat, 
               b: Vec<f64>,
               j: CooMat,
               h: Vec<CooMat>,  
               l: Vec<f64>, 
               u: Vec<f64>, 
               eval_fn: ProblemEval) -> Self {
        let p = vec![false;a.cols()];
        let base = Problem::new(hphi, a, b, j, h, l, u, p, eval_fn);
        Self {
            base: base,
        }       
    }
}

impl ProblemNlpBase for ProblemNlp {
    fn x(&self) -> &[f64] { &self.base.x() }
    fn phi(&self) -> f64 { self.base.phi() }
    fn gphi(&self) -> &[f64] { &self.base.gphi() }
    fn hphi(&self) -> &CooMat { &self.base.hphi() }
    fn a(&self) -> &CooMat { &self.base.a() } 
    fn b(&self) -> &[f64] { &self.base.b() }
    fn f(&self) -> &[f64] { &self.base.f() }
    fn j(&self) -> &CooMat { &self.base.j() } 
    fn h(&self) -> &Vec<CooMat> { &self.base.h() } 
    fn hcomb(&self) -> &CooMat { &self.base.hcomb() }
    fn l(&self) -> &[f64] { &self.base.l() }
    fn u(&self) -> &[f64] { &self.base.u() }
    fn evaluate(&mut self, x: &[f64]) -> () { self.base.evaluate(x) }
    fn combine_h(&mut self, nu: &[f64]) -> () { self.base.combine_h(nu) }
    fn base(&self) -> &Problem { &self.base }
}

impl ProblemBase for ProblemNlp {
    fn x(&self) -> &[f64] { &self.base.x() }
    fn phi(&self) -> f64 { self.base.phi() }
    fn gphi(&self) -> &[f64] { &self.base.gphi() }
    fn hphi(&self) -> &CooMat { &self.base.hphi() }
    fn a(&self) -> &CooMat { &self.base.a() } 
    fn b(&self) -> &[f64] { &self.base.b() }
    fn f(&self) -> &[f64] { &self.base.f() }
    fn j(&self) -> &CooMat { &self.base.j() } 
    fn h(&self) -> &Vec<CooMat> { &self.base.h() } 
    fn hcomb(&self) -> &CooMat { &self.base.hcomb() }
    fn l(&self) -> &[f64] { &self.base.l() }
    fn u(&self) -> &[f64] { &self.base.u() }
    fn p(&self) -> &[bool] { self.base.p() }
    fn evaluate(&mut self, x: &[f64]) -> () { self.base.evaluate(x) }
    fn combine_h(&mut self, nu: &[f64]) -> () { self.base.combine_h(nu) }
}