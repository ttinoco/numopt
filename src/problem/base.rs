use std::ops::Mul;
use std::str::FromStr;
use sprs::{TriMat, TriMatBase};
use num_traits::{Float, NumCast};
use std::fmt::{self, LowerExp, Debug};

pub type ProblemEval<T> = Box< dyn Fn(&mut T,              // phi
                                      &mut Vec<T>,         // gphi
                                      &mut TriMat<T>,      // Hphi
                                      &mut Vec<T>,         // f
                                      &mut TriMat<T>,      // J
                                      &mut Vec<TriMat<T>>, // H
                                      &[T]                 // x
                                    ) -> () >;

pub struct Problem<T> 
{
    x: Vec<T>,
    
    phi: T,
    gphi: Vec<T>,
    Hphi: TriMat<T>,
    
    A: TriMat<T>,
    b: Vec<T>,
    
    f: Vec<T>,
    J: TriMat<T>,
    H: Vec<TriMat<T>>,
    Hcomb: TriMat<T>,
    
    l: Vec<T>,
    u: Vec<T>,
    
    P: Option<Vec<bool>>,
    
    eval_fn: ProblemEval<T>,
}

pub trait ProblemBase {
    type N: Float + FromStr + LowerExp + Debug + Mul;
    fn x(&self) -> &[Self::N];
    fn phi(&self) -> Self::N;
    fn gphi(&self) -> &[Self::N];
    fn Hphi(&self) -> &TriMat<Self::N>;
    fn A(&self) -> &TriMat<Self::N>;
    fn b(&self) -> &[Self::N];
    fn f(&self) -> &[Self::N];
    fn J(&self) -> &TriMat<Self::N>;
    fn H(&self) -> &Vec<TriMat<Self::N>>;
    fn Hcomb(&self) -> &TriMat<Self::N>; 
    fn l(&self) -> &[Self::N];
    fn u(&self) -> &[Self::N];
    fn P(&self) -> Option<&[bool]>;
    fn evaluate(&mut self, x: &[Self::N]) -> ();
    fn combine_H(&mut self, nu: &[Self::N]) -> ();
}

pub trait ProblemDims {
    fn nx(&self) -> usize;
    fn na(&self) -> usize;
    fn nf(&self) -> usize;
}

pub struct ProblemSol<T: ProblemBase> {
    pub x: Vec<T::N>,
    pub lam: Vec<T::N>,
    pub nu: Vec<T::N>,
    pub mu: Vec<T::N>,
    pub pi: Vec<T::N>,
}

impl<T: Float + FromStr + LowerExp + Debug + Mul> Problem<T> 
{
    pub fn new(Hphi: TriMat<T>, 
               A: TriMat<T>, 
               b: Vec<T>,
               J: TriMat<T>,
               H: Vec<TriMat<T>>,  
               l: Vec<T>, 
               u: Vec<T>, 
               P: Option<Vec<bool>>,
               eval_fn: ProblemEval<T>) -> Self {

        let z: T = NumCast::from(0.).unwrap();

        let nx = A.cols();
        let na = A.rows();
        let nf = J.rows();

        assert_eq!(Hphi.cols(), nx);
        assert_eq!(Hphi.rows(), nx);

        assert_eq!(A.cols(), nx);
        assert_eq!(A.rows(), na);
        assert_eq!(b.len(), na);

        assert_eq!(J.cols(), nx);
        assert_eq!(J.rows(), nf);
        assert_eq!(H.len(), nf);
        for Hi in H.iter() {
            assert_eq!(Hi.rows(), nx);
            assert_eq!(Hi.cols(), nx);
        }

        assert_eq!(l.len(), nx);
        assert_eq!(u.len(), na);

        match &P {
            Some(p) => assert_eq!(p.len(), nx),
            None => (),
        }

        let Hcomb_nnz = H.iter().map(|h| h.nnz()).sum();
        let Hcomb: TriMat<T> = TriMatBase::with_capacity((nx, nx), Hcomb_nnz);
        for HH in H.iter() {
            for (val, (row, col)) in HH.triplet_iter() {
                Hcomb.add_triplet(row, col, z);
            }
        }
        
        Self {
            x: vec![NumCast::from(0.).unwrap();nx],
            phi: NumCast::from(0.).unwrap(),
            gphi: vec![NumCast::from(0.).unwrap();nx],
            Hphi: Hphi,
            A: A,
            b: b,
            f: vec![NumCast::from(0.).unwrap();nf],
            J: J,
            H: H,
            Hcomb: Hcomb,
            l: l,
            u: u,
            P: P,
            eval_fn: eval_fn
        }
    }
}

impl<N: Float + FromStr + LowerExp + Debug + Mul > ProblemBase for Problem<N> {

    type N = N;
    fn x(&self) -> &[N] { &self.x }
    fn phi(&self) -> N { self.phi }
    fn gphi(&self) -> &[N] { &self.gphi }
    fn Hphi(&self) -> &TriMat<N> { &self.Hphi }
    fn A(&self) -> &TriMat<N> { &self.A } 
    fn b(&self) -> &[N] { &self.b }
    fn f(&self) -> &[N] { &self.f }
    fn J(&self) -> &TriMat<N> { &self.J } 
    fn H(&self) -> &Vec<TriMat<N>> { &self.H } 
    fn Hcomb(&self) -> &TriMat<N> { &self.Hcomb }
    fn l(&self) -> &[N] { &self.l }
    fn u(&self) -> &[N] { &self.u }
    fn P(&self) -> Option<&[bool]> { 
        match self.P.as_ref() {
            Some(p) => Some(p),
            None => None
        }
    }
    
    fn evaluate(&mut self, x: &[N]) -> () {
        (self.eval_fn)(&mut self.phi, 
                       &mut self.gphi,
                       &mut self.Hphi,
                       &mut self.f,
                       &mut self.J,
                       &mut self.H,
                       x)
    }

    fn combine_H(&mut self, nu: &[N]) -> () {
        assert_eq!(self.nf(), nu.len());
        let k: usize = 0;
        let data = self.Hcomb.data();
        for (HH, nuval) in self.H.iter().zip(nu.iter()) {
            for val in HH.triplet_iter().into_data() {
                data[k] = (*nuval)*(*val);
                k += 1;
            }
        }    
    }
}

impl<T: ProblemBase> ProblemDims for T {
    fn nx(&self) -> usize { self.x().len() }
    fn na(&self) -> usize { self.b().len() }
    fn nf(&self) -> usize { self.f().len() }
}

impl<T: ProblemBase> ProblemSol<T> {
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

impl<T: ProblemBase> Debug for ProblemSol<T> {

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