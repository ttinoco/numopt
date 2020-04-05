use crate::matrix::CooMat;
use crate::problem::{Problem,
                     ProblemBase,
                     ProblemMilp,
                     ProblemMilpBase};

pub struct ProblemLp {
    base: ProblemMilp,
}

pub trait ProblemLpBase {
    fn x0(&self) -> Option<&[f64]>;
    fn c(&self) -> &[f64];
    fn a(&self) -> &CooMat;
    fn b(&self) -> &[f64];
    fn l(&self) -> &[f64];
    fn u(&self) -> &[f64];
    fn base(&self) -> &ProblemMilp;
    fn base_mut(&mut self) -> &mut ProblemMilp;
    fn nx(&self) -> usize { self.c().len() }
    fn na(&self) -> usize { self.b().len() }
}

impl ProblemLp {
    pub fn new(c: Vec<f64>,
               a: CooMat,
               b: Vec<f64>,  
               l: Vec<f64>,
               u: Vec<f64>,
               x0: Option<Vec<f64>>) -> Self {
        let nx = c.len();
        let base = ProblemMilp::new(c, a, b, l, u, vec![false;nx], x0);
        Self {
            base: base,
        }
    }
}

impl ProblemLpBase for ProblemLp {
    fn x0(&self) -> Option<&[f64]> { ProblemMilpBase::x0(&self.base) }
    fn c(&self) -> &[f64] { ProblemMilpBase::c(&self.base) }
    fn a(&self) -> &CooMat { ProblemMilpBase::a(&self.base) } 
    fn b(&self) -> &[f64] { ProblemMilpBase::b(&self.base) }
    fn l(&self) -> &[f64] { ProblemMilpBase::l(&self.base) }
    fn u(&self) -> &[f64] { ProblemMilpBase::u(&self.base) }
    fn base(&self) -> &ProblemMilp { &self.base }
    fn base_mut(&mut self) -> &mut ProblemMilp { &mut self.base }
}

impl ProblemMilpBase for ProblemLp {
    fn x0(&self) -> Option<&[f64]> { ProblemMilpBase::x0(&self.base) }
    fn c(&self) -> &[f64] { ProblemMilpBase::c(&self.base) }
    fn a(&self) -> &CooMat { ProblemMilpBase::a(&self.base) }
    fn b(&self) -> &[f64] { ProblemMilpBase::b(&self.base) }
    fn l(&self) -> &[f64] { ProblemMilpBase::l(&self.base) }
    fn u(&self) -> &[f64] { ProblemMilpBase::u(&self.base) }
    fn p(&self) -> &[bool] { ProblemMilpBase::p(&self.base) }
    fn base(&self) -> &Problem { self.base.base() }
    fn base_mut(&mut self) -> &mut Problem { self.base.base_mut() }
}

impl ProblemBase for ProblemLp {
    fn x0(&self) -> Option<&[f64]> { ProblemBase::x0(&self.base) }
    fn phi(&self) -> f64 { ProblemBase::phi(&self.base) }
    fn gphi(&self) -> &[f64] { ProblemBase::gphi(&self.base) }
    fn hphi(&self) -> &CooMat { ProblemBase::hphi(&self.base) }
    fn a(&self) -> &CooMat { ProblemBase::a(&self.base) }
    fn b(&self) -> &[f64] { ProblemBase::b(&self.base) }
    fn f(&self) -> &[f64] { ProblemBase::f(&self.base) }
    fn j(&self) -> &CooMat { ProblemBase::j(&self.base) }
    fn h(&self) -> &Vec<CooMat> { ProblemBase::h(&self.base) }
    fn hcomb(&self) -> &CooMat { ProblemBase::hcomb(&self.base) }
    fn l(&self) -> &[f64] { ProblemBase::l(&self.base) }
    fn u(&self) -> &[f64] { ProblemBase::u(&self.base) }
    fn p(&self) -> &[bool] { ProblemBase::p(&self.base) }
    fn evaluate(&mut self, x: &[f64]) -> () { ProblemBase::evaluate(&mut self.base, x) }
    fn combine_h(&mut self, _nu: &[f64]) -> () {}
}