extern crate numopt;
use numopt::matrix::coo::CooMat;
use numopt::problem::base::Problem;
use numopt::problem::milp::ProblemMilp;
use numopt::solver::{Solver, SolverParam, SolverStatus, SolverCbcCmd};
use numopt::model::{Model, Objective, NodeCmp, VariableScalar};
use numopt::assert_vec_approx_eq;

fn main () { 
    
    // Constructing the raw problem
    println!("\nUsing problem vectors and matrices ...");
    let a: CooMat<f64> = CooMat::new(
        (1 , 6),
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 2, 3, 4, 5],
        vec![2., 8., 4., 2., 5., -1.]
        );
    let c: Vec<f64> = vec![-5., -3., -2., -7., -4., 0.];
    let b: Vec<f64> = vec![0.];
    let u: Vec<f64> = vec![1., 1., 1., 1., 1., 10.];
    let l: Vec<f64> = vec![0., 0., 0., 0., -0., -1e8];
    let p = vec![true, true, true, true, true, false];
    let mut problem: Problem =  Problem::Milp(ProblemMilp::new(c, a, b, l, u, p, None)); 
    let mut s = SolverCbcCmd::new();
    s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
    let (status, solution) = s.solve(&mut problem).unwrap();
    println!("Solver Status: {}\n", status);
    println!("x1 = {}", solution.x[0]);
    println!("x2 = {}", solution.x[1]);
    println!("x3 = {}", solution.x[2]);
    println!("x4 = {}", solution.x[3]);
    println!("x5 = {}", solution.x[4]);
    assert_eq!(status, SolverStatus::Solved);
    assert_vec_approx_eq!(solution.x, 
                          &vec![1., 0., 0., 1., 1., 9.], 
                          epsilon=1e-8);

    // Using the modeling layer
    println!("\nUsing the modeling layer ...");
    let x1 = VariableScalar::new_integer("x1");
    let x2 = VariableScalar::new_integer("x2");
    let x3 = VariableScalar::new_integer("x3");
    let x4 = VariableScalar::new_integer("x4");
    let x5 = VariableScalar::new_integer("x5");
    let obj = 5.*&x1 + 3.*&x2 + 2.*&x3 + 7.*&x4 + 4.*&x5;
    let constraints = [
        &(2.*&x1 + 8.*&x2 + 4.*&x3 + 2.*&x4 + 5.*&x5).leq(10.), 
        &x1.leq(1.), &x1.geq(0.),
        &x2.leq(1.), &x2.geq(0.),
        &x3.leq(1.), &x3.geq(0.),
        &x4.leq(1.), &x4.geq(0.),
        &x5.leq(1.), &x5.geq(0.),
    ];
    let mut m = Model::new();
    m.set_objective(Objective::maximize(&obj));
    m.add_constraints(&constraints);
    m.solve(&s).unwrap();
    let final_primals = m.final_primals();
    println!("Solver Status: {}\n", *m.solver_status().unwrap());
    println!("x1 = {}", *final_primals.get(&x1).unwrap());
    println!("x2 = {}", *final_primals.get(&x2).unwrap());
    println!("x3 = {}", *final_primals.get(&x3).unwrap());
    println!("x4 = {}", *final_primals.get(&x4).unwrap());
    println!("x5 = {}", *final_primals.get(&x5).unwrap());
    assert_eq!(*m.solver_status().unwrap(), SolverStatus::Solved);
    assert_eq!(*final_primals.get(&x1).unwrap(), 1.);
    assert_eq!(*final_primals.get(&x2).unwrap(), 0.);
    assert_eq!(*final_primals.get(&x3).unwrap(), 0.);
    assert_eq!(*final_primals.get(&x4).unwrap(), 1.);
    assert_eq!(*final_primals.get(&x5).unwrap(), 1.);
    
}
