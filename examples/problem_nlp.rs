use approx::assert_abs_diff_eq;

use optrs;
use optrs::matrix::CooMat;
use optrs::assert_vec_approx_eq;
use optrs::problem::{ProblemNlp,
                     ProblemNlpBase};
use optrs::solver::{Solver,
                    SolverIpopt}; 

fn main () {

    println!("optrs example Nlp problem and solution");

    // Sample problem 
    // min        x0*x3*(x0+x1+x2) + x2 = x0*x3*x0 + x0*x3*x1 + x0*x3*x2 + x2
    // subject to x0*x1*x2*x3 - x4 == 0
    //            x0*x0 + x1*x1 + x2*x2 + x3*x3 - 40 == 0
    //             1 <= x0 <= 5
    //             1 <= x1 <= 5
    //             1 <= x2 <= 5
    //             1 <= x3 <= 5
    //            25 <= x4 <= inf

    // hphi
    // 2*x3         x3  x3 (2*x0+x1+x2) 0
    // x3           0   0  x0           0
    // x3           0   0  x0           0
    // (2*x0+x1+x2) x0  x0 0            0
    // 0            0   0  0            0
    let hphi: CooMat<f64> = CooMat::new(
        (5, 5),
        vec![0, 1, 2, 3, 3, 3],
        vec![0, 0, 0, 0, 1, 2],
        vec![0.; 6]
    );

    let a: CooMat<f64> = CooMat::from_nnz((0, 5), 0);
    let b: Vec<f64> = Vec::new();

    // j
    // x1*x2*x3 x0*x2*x3 x0*x1*x3 x0*x1*x2 -1 
    // 2*x0     2*x1     2*x2     2*x3      0
    let j: CooMat<f64> = CooMat::new(
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
    let h: Vec<CooMat<f64>> = vec![
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

    let mut p = ProblemNlp::new(
        hphi, 
        a,
        b,
        j,
        h,
        l,
        u,
        eval_fn
    );

    let x = vec![1., 2., 3., 4., 5.];

    p.evaluate(&x);

    println!("phi = {}", p.phi()); 
    println!("gphi = {:?}", p.gphi());
    println!("hphi = {:?}", p.hphi());
    println!("f = {:?}", p.f());
    println!("j = {:?}", p.j());
    println!("h[0] = {:?}", p.h()[0]);
    println!("h[1] = {:?}", p.h()[1]);

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

    let nu = vec![3., 5.];

    p.combine_h(&nu);

    println!("hcomb = {:?}", p.hcomb());

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

    let mut s = SolverIpopt::new();
    s.solve(&mut p).unwrap();
}