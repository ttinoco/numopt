use sprs::TriMatBase;
use optrs::{self,
            assert_vec_approx_eq,
            ProblemLp,
            ProblemLpBase,
            Solver,
            SolverClpCmd};

fn main () {

    println!("optrs example LP problem and solution");

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

    let mut p = ProblemLp::new(
        vec![180.,160., 0., 0., 0.],
        TriMatBase::from_triplets(
            (3, 5),
            vec![0,0,0,1,1,1,2,2,2],
            vec![0,1,2,0,1,3,0,1,4],
            vec![6.,1.,1.,3.,1.,1.,4.,6.,1.]),
        vec![12.,8.,24.],
        vec![0.,0.,-1e8,-1e8,-1e8],
        vec![5.,5.,0.,0.,0.],
    );

    let x = vec![0.5, 2., 1., 2., 3.];

    optrs::ProblemBase::eval(&mut p, &x);
    
    println!("x = {:?}", p.x());
    println!("phi = {}", optrs::ProblemBase::phi(&p));
    println!("gphi = {:?}", optrs::ProblemBase::gphi(&p));
    println!("c = {:?}", p.c());
    println!("a = {:?}", p.a());
    println!("b = {:?}", p.b());
    println!("l = {:?}", p.l());
    println!("u = {:?}", p.u());
    println!("p = {:?}", optrs::ProblemBase::p(&p));
    println!("p = {:?}", optrs::ProblemMilpBase::p(&p));

    let mut s = SolverClpCmd::new();
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
