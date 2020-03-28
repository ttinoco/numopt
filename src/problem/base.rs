use std::ops::Mul;
use std::str::FromStr;
use sprs::{TriMat, TriMatBase};
use num_traits::{Float, NumCast};
use std::fmt::{self, LowerExp, Debug};

pub trait ProblemFloat: Float + FromStr + LowerExp + Debug + Mul {}

pub type ProblemEval<T> = Box<dyn Fn(&mut T,              // phi
                                     &mut Vec<T>,         // gphi
                                     &mut TriMat<T>,      // Hphi
                                     &mut Vec<T>,         // f
                                     &mut TriMat<T>,      // J
                                     &mut Vec<TriMat<T>>, // H
                                     &[T]                 // x
                                    ) -> ()>;

pub struct Problem<T> 
{
    x: Vec<T>,
    
    phi: T,
    gphi: Vec<T>,
    hphi: TriMat<T>,  // lower triangular
    
    a: TriMat<T>,
    b: Vec<T>,
    
    f: Vec<T>,
    j: TriMat<T>,
    h: Vec<TriMat<T>>,
    hcomb: TriMat<T>, // lower triangular
    
    l: Vec<T>,
    u: Vec<T>,
    
    p: Vec<bool>,
    
    eval_fn: ProblemEval<T>,
}

pub trait ProblemBase {
    type N: ProblemFloat;
    fn x(&self) -> &[Self::N];
    fn phi(&self) -> Self::N;
    fn gphi(&self) -> &[Self::N];
    fn hphi(&self) -> &TriMat<Self::N>;
    fn a(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn f(&self) -> &[Self::N];
    fn j(&self) -> &TriMat<Self::N>;
    fn h(&self) -> &Vec<TriMat<Self::N>>;
    fn hcomb(&self) -> &TriMat<Self::N>; 
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn p(&self) -> &[bool];
    fn evaluate(&mut self, x: &[Self::N]) -> ();
    fn combine_h(&mut self, nu: &[Self::N]) -> ();
}

pub trait ProblemDims {
    fn nx(&self) -> usize;
    fn na(&self) -> usize;
    fn nf(&self) -> usize;
}

pub struct ProblemSol<T: ProblemFloat> {
    pub x: Vec<T>,
    pub lam: Vec<T>,
    pub nu: Vec<T>,
    pub mu: Vec<T>,
    pub pi: Vec<T>,
}

impl<T: Float + FromStr + LowerExp + Debug + Mul> ProblemFloat for T { }

impl<T: ProblemFloat> Problem<T> 
{
    pub fn new(hphi: TriMat<T>, 
               a: TriMat<T>, 
               b: Vec<T>,
               j: TriMat<T>,
               h: Vec<TriMat<T>>,  
               l: Vec<T>, 
               u: Vec<T>, 
               p: Vec<bool>,
               eval_fn: ProblemEval<T>) -> Self {

        let z: T = NumCast::from(0.).unwrap();

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

        let hcomb_nnz = h.iter().map(|h| h.nnz()).sum();
        let mut hcomb: TriMat<T> = TriMatBase::with_capacity((nx, nx), hcomb_nnz);
        for hh in h.iter() {
            for (_val, (row, col)) in hh.triplet_iter() {
                hcomb.add_triplet(row, col, z);
            }
        }
        
        Self {
            x: vec![NumCast::from(0.).unwrap();nx],
            phi: NumCast::from(0.).unwrap(),
            gphi: vec![NumCast::from(0.).unwrap();nx],
            hphi: hphi,
            a: a,
            b: b,
            f: vec![NumCast::from(0.).unwrap();nf],
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

impl<N: ProblemFloat> ProblemBase for Problem<N> {

    type N = N;
    fn x(&self) -> &[N] { &self.x }
    fn phi(&self) -> N { self.phi }
    fn gphi(&self) -> &[N] { &self.gphi }
    fn hphi(&self) -> &TriMat<N> { &self.hphi }
    fn a(&self) -> &TriMat<N> { &self.a } 
    fn b(&self) -> &[N] { &self.b }
    fn f(&self) -> &[N] { &self.f }
    fn j(&self) -> &TriMat<N> { &self.j } 
    fn h(&self) -> &Vec<TriMat<N>> { &self.h } 
    fn hcomb(&self) -> &TriMat<N> { &self.hcomb }
    fn l(&self) -> &[N] { &self.l }
    fn u(&self) -> &[N] { &self.u }
    fn p(&self) -> &[bool] { &self.p } 
    
    fn evaluate(&mut self, x: &[N]) -> () {
        (self.eval_fn)(&mut self.phi, 
                       &mut self.gphi,
                       &mut self.hphi,
                       &mut self.f,
                       &mut self.j,
                       &mut self.h,
                       x)
    }

    fn combine_h(&mut self, nu: &[N]) -> () {

        assert_eq!(self.nf(), nu.len());

        // Just to get a private TripletIndex!
        let mut k = TriMatBase::from_triplets((1,1), vec![0], vec![0], vec![0.])
                               .find_locations(0,0)[0];
        
        k.0 = 0;
        for (h, nuval) in self.h.iter().zip(nu.iter()) {
            for (val, (row, col)) in h.triplet_iter() {
                self.hcomb.set_triplet(k, row, col, (*nuval)*(*val));
                k.0 += 1;
            }
        }    
    }
}

impl<T: ProblemBase> ProblemDims for T {
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
    fn nf(&self) -> usize { self.f().len() }
}

impl<T: ProblemFloat> ProblemSol<T> {
    pub fn new(nx: usize, na: usize, nf: usize) -> Self {
        let z = NumCast::from(0.).unwrap();
        Self {
            x: vec![z;nx],
            lam: vec![z;na],
            nu: vec![z;nf],
            mu: vec![z;nx],
            pi: vec![z;nx]
        }
    }
}

impl<T: ProblemFloat> Debug for ProblemSol<T> {

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