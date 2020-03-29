
use crate::matrix::CooMat;

use crate::problem::{Problem, 
                     ProblemEval,
                     ProblemFloat,
                     ProblemBase};

pub struct ProblemNlp<T: ProblemFloat> {
    base: Problem<T>,
}

pub trait ProblemNlpBase {
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
    fn evaluate(&mut self, x: &[Self::N]) -> ();
    fn combine_h(&mut self, nu: &[Self::N]) -> ();
    fn base(&self) -> &Problem<Self::N>;
}

impl<T: ProblemFloat> ProblemNlp<T> {
    pub fn new(hphi: CooMat<T>, 
               a: CooMat<T>, 
               b: Vec<T>,
               j: CooMat<T>,
               h: Vec<CooMat<T>>,  
               l: Vec<T>, 
               u: Vec<T>, 
               eval_fn: ProblemEval<T>) -> Self {
        let p = vec![false;a.cols()];
        let base = Problem::new(hphi, a, b, j, h, l, u, p, eval_fn);
        Self {
            base: base,
        }       
    }
}

impl<N: ProblemFloat> ProblemNlpBase for ProblemNlp<N> {
    type N = N;
    fn x(&self) -> &[N] { &self.base.x() }
    fn phi(&self) -> N { self.base.phi() }
    fn gphi(&self) -> &[N] { &self.base.gphi() }
    fn hphi(&self) -> &CooMat<N> { &self.base.hphi() }
    fn a(&self) -> &CooMat<N> { &self.base.a() } 
    fn b(&self) -> &[N] { &self.base.b() }
    fn f(&self) -> &[N] { &self.base.f() }
    fn j(&self) -> &CooMat<N> { &self.base.j() } 
    fn h(&self) -> &Vec<CooMat<N>> { &self.base.h() } 
    fn hcomb(&self) -> &CooMat<N> { &self.base.hcomb() }
    fn l(&self) -> &[N] { &self.base.l() }
    fn u(&self) -> &[N] { &self.base.u() }
    fn evaluate(&mut self, x: &[N]) -> () { self.base.evaluate(x) }
    fn combine_h(&mut self, nu: &[N]) -> () { self.base.combine_h(nu) }
    fn base(&self) -> &Problem<Self::N> { &self.base }
}

impl<N: ProblemFloat> ProblemBase for ProblemNlp<N> {
    type N = N;
    fn x(&self) -> &[N] { &self.base.x() }
    fn phi(&self) -> N { self.base.phi() }
    fn gphi(&self) -> &[N] { &self.base.gphi() }
    fn hphi(&self) -> &CooMat<N> { &self.base.hphi() }
    fn a(&self) -> &CooMat<N> { &self.base.a() } 
    fn b(&self) -> &[N] { &self.base.b() }
    fn f(&self) -> &[N] { &self.base.f() }
    fn j(&self) -> &CooMat<N> { &self.base.j() } 
    fn h(&self) -> &Vec<CooMat<N>> { &self.base.h() } 
    fn hcomb(&self) -> &CooMat<N> { &self.base.hcomb() }
    fn l(&self) -> &[N] { &self.base.l() }
    fn u(&self) -> &[N] { &self.base.u() }
    fn p(&self) -> &[bool] { self.base.p() }
    fn evaluate(&mut self, x: &[N]) -> () { self.base.evaluate(x) }
    fn combine_h(&mut self, nu: &[N]) -> () { self.base.combine_h(nu) }
}