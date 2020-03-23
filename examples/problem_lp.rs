
#[macro_use]
extern crate optrs;

#[macro_use]
extern crate approx;

use sprs::{TriMat, TriMatBase};
use optrs::{Problem,
            ProblemLp,
            Solver,
            SolverClpCMD};

fn main () {

    println!("optrs example LP problem");

    // Sample problem 
    // min        180*x0 + 160*x1 
    // subject to 6*x0 +   x1 + x2 == 12
    //            3*x0 +   x1 + x3 ==  8
    //            4*x0 + 6*x1 + x4 == 24
    //            0 <= x0 <= 5
    //            0 <= x1 <= 5
    //            x2 <= 0
    //            x3 <= 0
    //            x4 <= 0

    struct P {
        x: Vec<f64>,
        c: Vec<f64>,
        a: TriMat<f64>,
        b: Vec<f64>,
        l: Vec<f64>,
        u: Vec<f64>,
    };

    impl ProblemLp for P {
        type N = f64;
        fn x(&self) -> &[f64] { &self.x }
        fn c(&self) -> &[f64] { &self.c }
        fn a(&self) -> &TriMat<f64> { &self.a }
        fn b(&self) -> &[f64] { &self.b }
        fn l(&self) -> &[f64] { &self.l }
        fn u(&self) -> &[f64] { &self.u }
        fn setx(&mut self, x: &[f64]) -> () {
            self.x = x.to_vec();
        }
    }

    let mut p = P {
        x: vec![0.,0.,0.,0.,0.],
        c: vec![180.,160., 0., 0., 0.],
        a: TriMatBase::from_triplets(
            (3, 5),
            vec![0,0,0,1,1,1,2,2,2],
            vec![0,1,2,0,1,3,0,1,4],
            vec![6.,1.,1.,3.,1.,1.,4.,6.,1.]),
        b: vec![12.,8.,24.],
        l: vec![0.,0.,-1e8,-1e8,-1e8],
        u: vec![5.,5.,0.,0.,0.],
    };

    let x = vec![0.5, 2., 1., 2., 3.];

    p.eval(&x);
    
    println!("x = {:?}", ProblemLp::x(&p));
    println!("phi = {}", p.phi());
    println!("gphi = {:?}", p.gphi());
    println!("c = {:?}", p.c());
    println!("a = {:?}", ProblemLp::a(&p));
    println!("b = {:?}", ProblemLp::b(&p));
    println!("l = {:?}", ProblemLp::l(&p));
    println!("u = {:?}", ProblemLp::u(&p));

    let mut s = SolverClpCMD::new();
    s.solve(p).unwrap();

    println!("solver status = {}", s.status());
    println!("solution = {:?}", s.solution());

    assert!(s.status().is_solved());
    assert!(s.solution().is_some());
    assert_vec_approx_eq!(s.solution().as_ref().unwrap().x, 
                          &vec![1.7142857, 2.8571429, -1.1428571, 0., 0.], 
                          epsilon=1e-8);
    assert_vec_approx_eq!(s.solution().as_ref().unwrap().lam, 
                          &vec![0., 31.428571, 21.428571], 
                          epsilon=1e-8);
    assert_vec_approx_eq!(s.solution().as_ref().unwrap().mu, 
                          &vec![1.4210855e-14, 0., 0., 3.1428571e+01, 2.1428571e+01], 
                          epsilon=1e-8);
    assert_vec_approx_eq!(s.solution().as_ref().unwrap().pi, 
                          &vec![0.;5], 
                          epsilon=1e-8);
}
