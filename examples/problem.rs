use optrs::matrix::CooMat;
use optrs::problem::{Problem, 
                     ProblemBase};

fn main () {

    println!("optrs example problem");

    // Sample problem
    // min        180*x1 + 160*x2 
    // subject to 6*x1 +   x2 + x3 == 12
    //            3*x1 +   x2 + x4 ==  8
    //            4*x1 + 6*x2 + x5 == 24
    //            0 <= x1 <= 5
    //            0 <= x2 <= 5
    //            x3 <= 0
    //            x4 <= 0
    //            x5 <= 0

    let mut p = Problem::new(
        CooMat::from_nnz((5, 5), 0),
        CooMat::new(
            (3, 5),
            vec![0,0,0,1,1,1,1,1,1],
            vec![0,1,2,0,1,3,0,1,4],
            vec![6.,1.,1.,3.,1.,1.,4.,6.,1.]),
        vec![12.,8.,24.],
        CooMat::from_nnz((0, 5), 0),
        Vec::new(),
        vec![0.,0.,-1e8,-1e8,-1e8],
        vec![5.,5.,0.,0.,0.],
        vec![false;5],
        Box::new(| phi: &mut f64, 
                   gphi: &mut Vec<f64>, 
                   _hphi: &mut CooMat<f64>,
                   _f: &mut Vec<f64>,
                   _j: &mut CooMat<f64>,
                   _h: &mut Vec<CooMat<f64>>,
                   x: &[f64] | {
            *phi = 180.*x[0] + 160.*x[1];
            gphi[0] = 180.;
            gphi[1] = 160.;
        })
    );

    let x = vec![0.5, 2., 1., 2., 3.];

    p.evaluate(&x);
    
    println!("x = {:?}", p.x());
    println!("phi = {}", p.phi());
    println!("gphi = {:?}", p.gphi());
    println!("a = {:?}", p.a());
    println!("b = {:?}", p.b());
    println!("l = {:?}", p.l());
    println!("u = {:?}", p.u());
}
