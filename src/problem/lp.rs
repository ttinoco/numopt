use sprs::TriMat; 

use crate::problem::{Problem,
                     ProblemFloat,
                     ProblemMilp,
                     ProblemMilpBase};

pub struct ProblemLp<T> {
    base: ProblemMilp<T>,
}

pub trait ProblemLpBase {
    type N: ProblemFloat;
    fn x(&self) -> &[Self::N];
    fn c(&self) -> &[Self::N];
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn base(&self) -> &ProblemMilp<Self::N>;
    fn base_mut(&mut self) -> &mut ProblemMilp<Self::N>;
}

impl<T: 'static + ProblemFloat> ProblemLp<T> {
    pub fn new(c: Vec<T>,
               a: TriMat<T>,
               b: Vec<T>,  
               l: Vec<T>,
               u: Vec<T>) -> Self {
        let base = ProblemMilp::new(c, a, b, l, u, None);
        Self {
            base: base,
        }
    }
}

impl<N: ProblemFloat> ProblemLpBase for ProblemLp<N> {
    type N = N;
    fn x(&self) -> &[N] { &self.base.x() }
    fn c(&self) -> &[N] { &self.base.c() }
    fn a(&self) -> &TriMat<N> { &self.base.a() } 
    fn b(&self) -> &[N] { &self.base.b() }
    fn l(&self) -> &[N] { &self.base.l() }
    fn u(&self) -> &[N] { &self.base.u() }
    fn base(&self) -> &ProblemMilp<Self::N> { &self.base }
    fn base_mut(&mut self) -> &mut ProblemMilp<Self::N> { &mut self.base }
}

impl<T: ProblemLpBase> ProblemMilpBase for T {
    type N = T::N;
    fn x(&self) -> &[Self::N] { self.x() }
    fn c(&self) -> &[Self::N] { self.c() }
    fn a(&self) -> &TriMat<Self::N> { self.a() }
    fn b(&self) -> &[Self::N] { self.b() }
    fn l(&self) -> &[Self::N] { self.l() }
    fn u(&self) -> &[Self::N] { self.u() }
    fn p(&self) -> Option<&[bool]> { None }
    fn base(&self) -> &Problem<Self::N> { self.base().base() }
    fn base_mut(&mut self) -> &mut Problem<Self::N> { self.base_mut().base_mut() }
}