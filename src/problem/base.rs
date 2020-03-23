use sprs::TriMat;
use std::str::FromStr;
use num_traits::{Float, NumCast};
use std::fmt::{self, LowerExp, Debug};

pub type ProblemEval<T> = fn(&mut T, &mut Vec<T>, &[T])-> ();

pub struct Problem<T> {
    x: Vec<T>,
    phi: T,
    gphi: Vec<T>,
    a: TriMat<T>,
    b: Vec<T>,
    l: Vec<T>,
    u: Vec<T>,
    p: Option<Vec<bool>>,
    eval_fn: ProblemEval<T>,
}

pub trait ProblemBase {
    type T: Float + FromStr + LowerExp + Debug;
    fn x(&self) -> &[Self::T];
    fn phi(&self) -> Self::T;
    fn gphi(&self) -> &[Self::T];
    fn a(&self) -> &TriMat<Self::T>;
    fn b(&self) -> &[Self::T];
    fn l(&self) -> &[Self::T];
    fn u(&self) -> &[Self::T];
    fn p(&self) -> Option<&[bool]>;
    fn eval(&mut self, x: &[Self::T]) -> ();
}

pub trait ProblemDims {
    fn nx(&self) -> usize;
    fn na(&self) -> usize;
}

pub struct ProblemSol<T: ProblemBase> {
    pub x: Vec<T::T>,
    pub lam: Vec<T::T>,
    pub mu: Vec<T::T>,
    pub pi: Vec<T::T>,
}

impl<T: Float + FromStr + LowerExp + Debug> Problem<T> {
    pub fn new(a: TriMat<T>, 
               b: Vec<T>,  
               l: Vec<T>, 
               u: Vec<T>, 
               p: Option<Vec<bool>>,
               eval_fn: ProblemEval<T>) -> Self {
        assert_eq!(a.cols(), l.len());
        assert_eq!(a.cols(), u.len());
        match &p {
            Some(pp) => assert_eq!(a.cols(), pp.len()),
            None => (),
        }
        assert_eq!(a.rows(), b.len());
        Self {
            x: vec![NumCast::from(0.).unwrap();a.cols()],
            phi: NumCast::from(0.).unwrap(),
            gphi: vec![NumCast::from(0.).unwrap();a.cols()],
            a: a,
            b: b,
            l: l,
            u: u,
            p: p,
            eval_fn: eval_fn
        }
    }
}

impl<N: Float + FromStr + LowerExp + Debug> ProblemBase for Problem<N> {
    type T = N;
    fn x(&self) -> &[N] { &self.x }
    fn phi(&self) -> N { self.phi }
    fn gphi(&self) -> &[N] { &self.gphi }
    fn a(&self) -> &TriMat<N> { &self.a } 
    fn b(&self) -> &[N] { &self.b }
    fn l(&self) -> &[N] { &self.l }
    fn u(&self) -> &[N] { &self.u }
    fn p(&self) -> Option<&[bool]> { 
        match self.p.as_ref() {
            Some(p) => Some(p),
            None => None
        }
    }
    fn eval(&mut self, x: &[N]) -> () {
        (self.eval_fn)(&mut self.phi, 
                       &mut self.gphi,
                       x)
    }
}

impl<T: ProblemBase> ProblemDims for T {
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
}

impl<T: ProblemBase> ProblemSol<T> {
    pub fn new(nx: usize, na: usize) -> Self {
        let z = NumCast::from(0.).unwrap();
        Self {
            x: vec![z;nx],
            lam: vec![z;na],
            mu: vec![z;nx],
            pi: vec![z;nx]
        }
    }
}

impl<T: ProblemBase> Debug for ProblemSol<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProblemSol")
         .field("x", &self.x)
         .field("lam", &self.lam)
         .field("mu", &self.mu)
         .field("pi", &self.pi)
         .finish()
    }
}