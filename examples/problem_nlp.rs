use sprs::{TriMat,
           TriMatI,
           TriMatBase};
use optrs::{self,
            assert_vec_approx_eq,
            ProblemNlp,
            ProblemNlpBase};

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
    let hphi: TriMatI<f64, usize> = TriMatBase::from_triplets(
        (5, 5),
        vec![0, 1, 2, 3, 3, 3],
        vec![0, 0, 0, 0, 1, 1],
        vec![0.; 6]
    );

    let a: TriMatI<f64, usize> = TriMatBase::new((0, 5));
    let b: Vec<f64> = Vec::new();

    // j
    // x1*x2*x3 x0*x2*x3 x0*x1*x3 x0*x1*x2 -1 
    // 2*x0     2*x1     2*x2     2*x3      0
    let j: TriMatI<f64, usize> = TriMatBase::from_triplets(
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
    let h: Vec<TriMatI<f64, usize>> = vec![
        TriMatBase::from_triplets(
            (5, 5),
            vec![1, 2, 2, 3, 3, 3],
            vec![0, 0, 1, 0, 1, 2],
            vec![0.;6]
        ),
        TriMatBase::from_triplets(
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
    let eval_fn = | phi: &mut f64, 
                    gphi: &mut Vec<f64>, 
                    hphi: &mut TriMat<f64>,
                    f: &mut Vec<f64>,
                    j: &mut TriMat<f64>,
                    h: &mut Vec<TriMat<f64>>,
                    x: &[f64] | {

        let x0 = x[0];
        let x1 = x[1];
        let x2 = x[2];
        let x3 = x[3];
        let x4 = x[4];

        // phi
        *phi = x0*x3*(x0+x1+x2) + x2;

        // gphi
        gphi[0] = 2.*x0*x3;
        gphi[1] = x0*x3;
        gphi[2] = x0*x3 + 1.;
        gphi[3] = x0*(x0+x1+x2);
        gphi[4] = 0.;

        // hphi
        

    };

}