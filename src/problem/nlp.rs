
use crate::matrix::coo::CooMat;
use crate::problem::base::ProblemEval;
use crate::problem::minlp::ProblemMinlp;

/// Smooth nonlinear optimization problem (Nlp).
pub struct ProblemNlp {
    base: ProblemMinlp,
}

/// A trait for smooth nonlinear optimization problems
/// (Nlp) of the form
/// ```ignore
/// minimize   phi(x)
/// subject to a*x = b
///            f(x) = 0
///            l <= x <= u
/// ```
// pub trait ProblemNlpBase {

//     /// Initial point.
//     fn x0(&self) -> Option<&[f64]>;

//     /// Objective function value.
//     fn phi(&self) -> f64;

//     /// Objective function gradient value.
//     fn gphi(&self) -> &[f64];

//     /// Objective function Hessian value (lower triangular part)
//     fn hphi(&self) -> &CooMat<f64>;

//     /// Jacobian matrix of linear equality constraints.
//     fn a(&self) -> &CooMat<f64>;

//     /// Right-hand-side vector of linear equality constraints.
//     fn b(&self) -> &[f64];

//     /// Nonlinear equality constraint function value.
//     fn f(&self) -> &[f64];

//     /// Nonlinear equality constraint function Jacobian value.
//     fn j(&self) -> &CooMat<f64>;

//     /// Vector of nonlinear equality constraint function Hessian values
//     /// (lower triangular parts).
//     fn h(&self) -> &Vec<CooMat<f64>>;

//     /// Linear combination of nonlinear equality constraint function Hessian values
//     /// (lower triangular part).
//     fn hcomb(&self) -> &CooMat<f64>;
    
//     /// Vector of optimization variable lower limits.
//     fn l(&self) -> &[f64];

//     /// Vector of optimization variable upper limits.
//     fn u(&self) -> &[f64];

//     /// Function that evaluates objective function, nonlinear equality constraint
//     /// functions, and their derivaties for a given vector of optimization variable values.
//     fn evaluate(&mut self, x: &[f64]) -> ();

//     /// Function that forms a linear combination of nonlinear equality constraint
//     /// function Hessians.
//     fn combine_h(&mut self, nu: &[f64]) -> ();

//     /// A reference to the problem as an Minlp problem.
//     fn base(&self) -> &ProblemMinlp;

//     /// Number of optimization variables.
//     fn nx(&self) -> usize { self.gphi().len() }

//     /// Number of linear equality constraints.
//     fn na(&self) -> usize { self.b().len() }

//     /// Number of nonlinear equality constraints.
//     fn nf(&self) -> usize { self.f().len() }
// }

impl ProblemNlp {

    /// Creates new smooth nonlinear optimization problem (Nlp).
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
        let base = ProblemMinlp::new(hphi, a, b, j, h, l, u, p, x0, eval_fn);
        Self {
            base: base,
        }       
    }
 
    pub fn x0(&self) -> Option<&[f64]> { self.base.x0() }
    pub fn phi(&self) -> f64 { self.base.phi() }
    pub fn gphi(&self) -> &[f64] { &self.base.gphi() }
    pub fn hphi(&self) -> &CooMat<f64> { &self.base.hphi() }
    pub fn a(&self) -> &CooMat<f64> { &self.base.a() } 
    pub fn b(&self) -> &[f64] { &self.base.b() }
    pub fn f(&self) -> &[f64] { &self.base.f() }
    pub fn j(&self) -> &CooMat<f64> { &self.base.j() } 
    pub fn h(&self) -> &Vec<CooMat<f64>> { &self.base.h() } 
    pub fn hcomb(&self) -> &CooMat<f64> { &self.base.hcomb() }
    pub fn l(&self) -> &[f64] { &self.base.l() }
    pub fn u(&self) -> &[f64] { &self.base.u() }
    pub fn nx(&self) -> usize { self.a().cols() }
    pub fn na(&self) -> usize { self.b().len() }
    pub fn nf(&self) -> usize { self.f().len() }
    pub fn evaluate(&mut self, x: &[f64]) -> () { self.base.evaluate(x) }
    pub fn combine_h(&mut self, nu: &[f64]) -> () { self.base.combine_h(nu) }
    pub fn as_mut_minlp(&mut self) -> &mut ProblemMinlp { &mut self.base }
}

#[cfg(test)]
mod tests {

    use approx::assert_abs_diff_eq;
    
    use crate::matrix::coo::CooMat;
    use crate::problem::nlp::ProblemNlp;
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
            let hphi_data = hphi.data_mut();
            hphi_data[0] = 2.*x3;       // x0, x0
            hphi_data[1] = x3;          // x1, x0
            hphi_data[2] = x3;          // x2, x0
            hphi_data[3] = 2.*x0+x1+x2; // x3, x0
            hphi_data[4] = x0;          // x3, x1
            hphi_data[5] = x0;          // x3, x2

            // f
            f[0] = x0*x1*x2*x3 - x4;
            f[1] = x0*x0 + x1*x1 + x2*x2 + x3*x3 - 40.;

            // j
            let j_data = j.data_mut();
            j_data[0] = x1*x2*x3; // 0, x0
            j_data[1] = x0*x2*x3; // 0, x1
            j_data[2] = x0*x1*x3; // 0, x2
            j_data[3] = x0*x1*x2; // 0, x3
            j_data[4] = -1.;      // 0, x4
            j_data[5] = 2.*x0;    // 1, x0
            j_data[6] = 2.*x1;    // 1, x1
            j_data[7] = 2.*x2;    // 1, x2
            j_data[8] = 2.*x3;    // 1, x3

            // h0
            let h0_data = h[0].data_mut();
            h0_data[0] = x2*x3;
            h0_data[1] = x1*x3;
            h0_data[2] = x0*x3;
            h0_data[3] = x1*x2;
            h0_data[4] = x0*x2;
            h0_data[5] = x0*x1;

            // h1
            let h1_data = h[1].data_mut();
            h1_data[0] = 2.;
            h1_data[1] = 2.;
            h1_data[2] = 2.;
            h1_data[3] = 2.;
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