use crate::matrix::CooMat;
use crate::problem::{Problem,
                     ProblemBase,
                     ProblemFloat,
                     ProblemMilp,
                     ProblemMilpBase};

pub struct ProblemLp<T: ProblemFloat> {
    base: ProblemMilp<T>,
}

pub trait ProblemLpBase {
    type N: ProblemFloat;
    fn x(&self) -> &[Self::N];
    fn c(&self) -> &[Self::N];
    fn a(&self) -> &CooMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn base(&self) -> &ProblemMilp<Self::N>;
    fn base_mut(&mut self) -> &mut ProblemMilp<Self::N>;
}

impl<T: 'static + ProblemFloat> ProblemLp<T> {
    pub fn new(c: Vec<T>,
               a: CooMat<T>,
               b: Vec<T>,  
               l: Vec<T>,
               u: Vec<T>) -> Self {
        let nx = c.len();
        let base = ProblemMilp::new(c, a, b, l, u, vec![false;nx]);
        Self {
            base: base,
        }
    }
}

impl<N: ProblemFloat> ProblemLpBase for ProblemLp<N> {
    type N = N;
    fn x(&self) -> &[N] { ProblemMilpBase::x(&self.base) }
    fn c(&self) -> &[N] { ProblemMilpBase::c(&self.base) }
    fn a(&self) -> &CooMat<N> { ProblemMilpBase::a(&self.base) } 
    fn b(&self) -> &[N] { ProblemMilpBase::b(&self.base) }
    fn l(&self) -> &[N] { ProblemMilpBase::l(&self.base) }
    fn u(&self) -> &[N] { ProblemMilpBase::u(&self.base) }
    fn base(&self) -> &ProblemMilp<Self::N> { &self.base }
    fn base_mut(&mut self) -> &mut ProblemMilp<Self::N> { &mut self.base }
}

impl<N: ProblemFloat> ProblemMilpBase for ProblemLp<N> {
    type N = N;
    fn x(&self) -> &[N] { ProblemMilpBase::x(&self.base) }
    fn c(&self) -> &[N] { ProblemMilpBase::c(&self.base) }
    fn a(&self) -> &CooMat<N> { ProblemMilpBase::a(&self.base) }
    fn b(&self) -> &[N] { ProblemMilpBase::b(&self.base) }
    fn l(&self) -> &[N] { ProblemMilpBase::l(&self.base) }
    fn u(&self) -> &[N] { ProblemMilpBase::u(&self.base) }
    fn p(&self) -> &[bool] { ProblemMilpBase::p(&self.base) }
    fn base(&self) -> &Problem<N> { self.base.base() }
    fn base_mut(&mut self) -> &mut Problem<N> { self.base.base_mut() }
}

impl<N: ProblemFloat> ProblemBase for ProblemLp<N> {
    type N = N;
    fn x(&self) -> &[N] { ProblemBase::x(&self.base) }
    fn phi(&self) -> N { ProblemBase::phi(&self.base) }
    fn gphi(&self) -> &[N] { ProblemBase::gphi(&self.base) }
    fn hphi(&self) -> &CooMat<N> { ProblemBase::hphi(&self.base) }
    fn a(&self) -> &CooMat<N> { ProblemBase::a(&self.base) }
    fn b(&self) -> &[N] { ProblemBase::b(&self.base) }
    fn f(&self) -> &[N] { ProblemBase::f(&self.base) }
    fn j(&self) -> &CooMat<N> { ProblemBase::j(&self.base) }
    fn h(&self) -> &Vec<CooMat<N>> { ProblemBase::h(&self.base) }
    fn hcomb(&self) -> &CooMat<N> { ProblemBase::hcomb(&self.base) }
    fn l(&self) -> &[N] { ProblemBase::l(&self.base) }
    fn u(&self) -> &[N] { ProblemBase::u(&self.base) }
    fn p(&self) -> &[bool] { ProblemBase::p(&self.base) }
    fn evaluate(&mut self, x: &[N]) -> () { ProblemBase::evaluate(&mut self.base, x) }
    fn combine_h(&mut self, _nu: &[N]) -> () {}
}