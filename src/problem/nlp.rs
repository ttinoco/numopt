
use crate::matrix::CooMat;

use crate::problem::{Problem, 
                     ProblemEval,
                     ProblemBase};

pub struct ProblemNlp {
    base: Problem,
}

pub trait ProblemNlpBase {
    fn x0(&self) -> Option<&[f64]>;
    fn phi(&self) -> f64;
    fn gphi(&self) -> &[f64];
    fn hphi(&self) -> &CooMat<f64>;
    fn a(&self) -> &CooMat<f64>;
    fn b(&self) -> &[f64];
    fn f(&self) -> &[f64];
    fn j(&self) -> &CooMat<f64>;
    fn h(&self) -> &Vec<CooMat<f64>>;
    fn hcomb(&self) -> &CooMat<f64>; 
    fn l(&self) -> &[f64];
    fn u(&self) -> &[f64];
    fn evaluate(&mut self, x: &[f64]) -> ();
    fn combine_h(&mut self, nu: &[f64]) -> ();
    fn base(&self) -> &Problem;
    fn nx(&self) -> usize { self.gphi().len() }
    fn na(&self) -> usize { self.b().len() }
    fn nf(&self) -> usize { self.f().len() }
}

impl ProblemNlp {
    pub fn new(hphi: CooMat<f64>, 
               a: CooMat<f64>, 
               b: Vec<f64>,
               j: CooMat<f64>,
               h: Vec<CooMat<f64>>,  
               l: Vec<f64>, 
               u: Vec<f64>, 
               x0: Option<Vec<f64>>,
               eval_fn: ProblemEval) -> Self {
        let p = vec![false;a.cols()];
        let base = Problem::new(hphi, a, b, j, h, l, u, p, x0, eval_fn);
        Self {
            base: base,
        }       
    }
}

impl ProblemNlpBase for ProblemNlp {
    fn x0(&self) -> Option<&[f64]> { self.base.x0() }
    fn phi(&self) -> f64 { self.base.phi() }
    fn gphi(&self) -> &[f64] { &self.base.gphi() }
    fn hphi(&self) -> &CooMat<f64> { &self.base.hphi() }
    fn a(&self) -> &CooMat<f64> { &self.base.a() } 
    fn b(&self) -> &[f64] { &self.base.b() }
    fn f(&self) -> &[f64] { &self.base.f() }
    fn j(&self) -> &CooMat<f64> { &self.base.j() } 
    fn h(&self) -> &Vec<CooMat<f64>> { &self.base.h() } 
    fn hcomb(&self) -> &CooMat<f64> { &self.base.hcomb() }
    fn l(&self) -> &[f64] { &self.base.l() }
    fn u(&self) -> &[f64] { &self.base.u() }
    fn evaluate(&mut self, x: &[f64]) -> () { self.base.evaluate(x) }
    fn combine_h(&mut self, nu: &[f64]) -> () { self.base.combine_h(nu) }
    fn base(&self) -> &Problem { &self.base }
}

impl ProblemBase for ProblemNlp {
    fn x0(&self) -> Option<&[f64]> { self.base.x0() }
    fn phi(&self) -> f64 { self.base.phi() }
    fn gphi(&self) -> &[f64] { &self.base.gphi() }
    fn hphi(&self) -> &CooMat<f64> { &self.base.hphi() }
    fn a(&self) -> &CooMat<f64> { &self.base.a() } 
    fn b(&self) -> &[f64] { &self.base.b() }
    fn f(&self) -> &[f64] { &self.base.f() }
    fn j(&self) -> &CooMat<f64> { &self.base.j() } 
    fn h(&self) -> &Vec<CooMat<f64>> { &self.base.h() } 
    fn hcomb(&self) -> &CooMat<f64> { &self.base.hcomb() }
    fn l(&self) -> &[f64] { &self.base.l() }
    fn u(&self) -> &[f64] { &self.base.u() }
    fn p(&self) -> &[bool] { self.base.p() }
    fn evaluate(&mut self, x: &[f64]) -> () { self.base.evaluate(x) }
    fn combine_h(&mut self, nu: &[f64]) -> () { self.base.combine_h(nu) }
}

#[cfg(test)]
mod tests {

    use approx::assert_abs_diff_eq;
    
    use crate::matrix::CooMat;
    use crate::problem::{ProblemNlp, ProblemNlpBase};
    use crate::assert_vec_approx_eq;

    fn nlp_construct() -> ProblemNlp {

        // Sample NLP problem 
        // min        x0*x3*(x0+x1+x2) + x2 = x0*x3*x0 + x0*x3*x1 + x0*x3*x2 + x2
        // subject to x0*x1*x2*x3 - x4 == 0
        //            x0*x0 + x1*x1 + x2*x2 + x3*x3 - 40 == 0
        //             1 <= x0 <= 5
        //             1 <= x1 <= 5
        //             1 <= x2 <= 5
        //             1 <= x3 <= 5
        //            25 <= x4 <= inf

        // x0
        let x0 = vec![1., 5., 5., 1., 0.];

        // hphi
        // 2*x3         x3  x3 (2*x0+x1+x2) 0
        // x3           0   0  x0           0
        // x3           0   0  x0           0
        // (2*x0+x1+x2) x0  x0 0            0
        // 0            0   0  0            0
        let hphi = CooMat::new(
            (5, 5),
            vec![0, 1, 2, 3, 3, 3],
            vec![0, 0, 0, 0, 1, 2],
            vec![0.; 6]
        );

        let a = CooMat::from_nnz((0, 5), 0);
        let b = Vec::new();

        // j
        // x1*x2*x3 x0*x2*x3 x0*x1*x3 x0*x1*x2 -1 
        // 2*x0     2*x1     2*x2     2*x3      0
        let j = CooMat::new(
            (2, 5),
            vec![0, 0, 0, 0, 0, 1, 1, 1, 1],
            vec![0, 1, 2, 3, 4, 0, 1, 2, 3],
            vec![0.;9]
        );

        // h0
        // 0     x2*x3 x1*x3 x1*x2 0 
        // x2*x3 0     x0*x3 x0*x2 0
        // x1*x3 x0*x3 0     x0*x1 0
        // x1*x2 x0*x2 x0*x1 0     0
        // 0     0     0     0     0
        // h1
        // 2 0 0 0 0 
        // 0 2 0 0 0
        // 0 0 2 0 0
        // 0 0 0 2 0
        // 0 0 0 0 0
        let h = vec![
            CooMat::new(
                (5, 5),
                vec![1, 2, 2, 3, 3, 3],
                vec![0, 0, 1, 0, 1, 2],
                vec![0.;6]
            ),
            CooMat::new(
                (5, 5),
                vec![0, 1, 2, 3],
                vec![0, 1, 2, 3],
                vec![0.;4]
            )
        ];

        // l
        let l = vec![1., 1., 1., 1., 25.];

        // u
        let u = vec![5., 5., 5., 5., 1e8];
        
        // eval_fn
        let eval_fn = Box::new(move | phi: &mut f64, 
                                      gphi: &mut Vec<f64>, 
                                      hphi: &mut CooMat<f64>,
                                      f: &mut Vec<f64>,
                                      j: &mut CooMat<f64>,
                                      h: &mut Vec<CooMat<f64>>,
                                      x: &[f64] | {

            assert_eq!(gphi.len(), x.len());

            let x0 = x[0];
            let x1 = x[1];
            let x2 = x[2];
            let x3 = x[3];
            let x4 = x[4];

            // phi
            *phi = x0*x3*(x0+x1+x2) + x2;

            // gphi
            gphi[0] = 2.*x0*x3 + x1*x3 + x2*x3;
            gphi[1] = x0*x3;
            gphi[2] = x0*x3 + 1.;
            gphi[3] = x0*(x0+x1+x2);
            gphi[4] = 0.;

            // hphi
            hphi.set_data(0, 2.*x3);       // x0, x0
            hphi.set_data(1, x3);          // x1, x0
            hphi.set_data(2, x3);          // x2, x0
            hphi.set_data(3, 2.*x0+x1+x2); // x3, x0
            hphi.set_data(4, x0);          // x3, x1
            hphi.set_data(5, x0);          // x3, x2

            // f
            f[0] = x0*x1*x2*x3 - x4;
            f[1] = x0*x0 + x1*x1 + x2*x2 + x3*x3 - 40.;

            // j
            j.set_data(0, x1*x2*x3); // 0, x0
            j.set_data(1, x0*x2*x3); // 0, x1
            j.set_data(2, x0*x1*x3); // 0, x2
            j.set_data(3, x0*x1*x2); // 0, x3
            j.set_data(4, -1.);      // 0, x4
            j.set_data(5, 2.*x0);    // 1, x0
            j.set_data(6, 2.*x1);    // 1, x1
            j.set_data(7, 2.*x2);    // 1, x2
            j.set_data(8, 2.*x3);    // 1, x3

            // h0
            h[0].set_data(0, x2*x3);
            h[0].set_data(1, x1*x3);
            h[0].set_data(2, x0*x3);
            h[0].set_data(3, x1*x2);
            h[0].set_data(4, x0*x2);
            h[0].set_data(5, x0*x1);

            // h1
            h[1].set_data(0, 2.);
            h[1].set_data(1, 2.);
            h[1].set_data(2, 2.);
            h[1].set_data(3, 2.);
        });

        let p = ProblemNlp::new(
            hphi, 
            a,
            b,
            j,
            h,
            l,
            u,
            Some(x0),
            eval_fn
        );        

        // Return
        p
    }

    #[test]
    fn nlp_evaluate() {

        let mut p = nlp_construct();
        let x = vec![1., 2., 3., 4., 5.];

        p.evaluate(&x);

        assert_abs_diff_eq!(p.phi(), 27., epsilon=1e-8);
        assert_vec_approx_eq!(p.gphi(), vec![28., 4., 5., 6., 0.], epsilon=1e-8);
        assert_vec_approx_eq!(p.hphi().data(), 
                              vec![8., 4., 4., 7., 1., 1.],
                              epsilon=1e-8);
        assert_vec_approx_eq!(p.f(), vec![19., -10.], epsilon=1e-8);
        assert_vec_approx_eq!(p.j().data(),
                              vec![24., 12., 8., 6., -1., 2., 4., 6., 8.],
                              epsilon=1e-8);
        assert_vec_approx_eq!(p.h()[0].data(),
                              vec![12., 8., 4., 6., 3., 2.],
                              epsilon=1e-8);
        assert_vec_approx_eq!(p.h()[1].data(),
                              vec![2., 2., 2., 2.],
                              epsilon=1e-8);
    }

    #[test]
    fn nlp_combine_h() {

        let mut p = nlp_construct();
        let x = vec![1., 2., 3., 4., 5.];
        let nu = vec![3., 5.];

        p.evaluate(&x);
        p.combine_h(&nu);

        assert_vec_approx_eq!(p.hcomb().row_inds(), 
                              [p.h()[0].row_inds(), p.h()[1].row_inds()].concat(),
                              epsilon=0);
        assert_vec_approx_eq!(p.hcomb().col_inds(), 
                              [p.h()[0].col_inds(), p.h()[1].col_inds()].concat(),
                              epsilon=0);

        let data_manual: Vec<f64> = [
            p.h()[0].data().iter().map(|xx| nu[0]*xx).collect::<Vec<f64>>(),
            p.h()[1].data().iter().map(|xx| nu[1]*xx).collect::<Vec<f64>>()
        ].concat();

        assert_vec_approx_eq!(p.hcomb().data(),
                              data_manual,
                              epsilon=1e-8);

    }
}