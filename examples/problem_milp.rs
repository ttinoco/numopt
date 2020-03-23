use sprs::{TriMat, TriMatBase};
use optrs::{assert_vec_approx_eq,
            Problem,
            ProblemMilp,
            Solver,
            SolverCbcCmd};

fn main () {

    println!("optrs example MILP problem and solution");

    // Sample problem 
    // min        -x0 - x1 
    // subject to -2*x0 +  2*x1 + x2 == 1
    //            -8*x0 + 10*x1 + x3 ==  13
    //            x2 <= 0
    //            x3 >= 0
    //            x0 integer
    //            x1 integer

    struct P {
        x: Vec<f64>,
        c: Vec<f64>,
        a: TriMat<f64>,
        b: Vec<f64>,
        l: Vec<f64>,
        u: Vec<f64>,
        p: Option<Vec<bool>>,
    };

    impl ProblemMilp for P {
        type N = f64;
        fn x(&self) -> &[f64] { &self.x }
        fn c(&self) -> &[f64] { &self.c }
        fn a(&self) -> &TriMat<f64> { &self.a }
        fn b(&self) -> &[f64] { &self.b }
        fn l(&self) -> &[f64] { &self.l }
        fn u(&self) -> &[f64] { &self.u }
        fn p(&self) -> Option<&[bool]> { 
            match self.p.as_ref() {
                Some(p) => Some(p),
                None => None
            }
        }
        fn setx(&mut self, x: &[f64]) -> () {
            self.x = x.to_vec();
        }
    }

    let mut p = P {
        x: vec![0.,0.,0.,0.,0.],
        c: vec![-1.,-1., 0., 0.],
        a: TriMatBase::from_triplets(
            (2, 4),
            vec![0,0,0,1,1,1],
            vec![0,1,2,0,1,3],
            vec![-2.,2.,1.,-8.,10.,1.]),
        b: vec![1.,13.],
        l: vec![-1e8,-1e8,-1e8,0.],
        u: vec![1e8,1e8,0.,1e8],
        p: Some(vec![true, true, false, false]),
    };

    let x = vec![0.5, 2., 1., 2.];

    p.eval(&x);
    
    println!("x = {:?}", ProblemMilp::x(&p));
    println!("phi = {}", p.phi());
    println!("gphi = {:?}", p.gphi());
    println!("c = {:?}", p.c());
    println!("a = {:?}", ProblemMilp::a(&p));
    println!("b = {:?}", ProblemMilp::b(&p));
    println!("l = {:?}", ProblemMilp::l(&p));
    println!("u = {:?}", ProblemMilp::u(&p));
    println!("p = {:?}", ProblemMilp::p(&p));

    let mut s = SolverCbcCmd::new();
    s.solve(p).unwrap();

    println!("solver status = {}", s.status());
    println!("solution = {:?}", s.solution());

    assert!(s.status().is_solved());
    assert!(s.solution().is_some());
    assert_vec_approx_eq!(s.solution().as_ref().unwrap().x, 
                          &vec![1., 2., -1., 1.0], 
                          epsilon=1e-8);
    
}