// Solve a simple LP using Clp
extern crate numopt;
use numopt::model::*;
use numopt::solver::*;

fn main () { 
    println!("Solving: \n min 2x-5y \n subject to \n\tx >= 100 \n \tx < 200\n\ty >= 80 \n \ty <= 170 \n \ty + 2x = x + 200\n");
    println!("-----------------------------------------------------");


    // Variables
    let x = VariableScalar::new_continuous("x");
    let y = VariableScalar::new_continuous("y");

    // Objective function
    let f = 2.*&x - 5.*&y;
     
    // Constraints
    let c1 = x.geq(100.);
    let c2 = x.leq(200.);
    let c3 = y.geq(80.);
    let c4 = y.leq(170.);
    let c5 = (&y + 2.*&x).equal(&x + 200.);
    let constraints = [&c1, &c2, &c3, &c4, &c5];
     
    // Model
    let mut m = Model::new();
    m.set_objective(Objective::minimize(&f));
    m.add_constraint(&c1);
    m.add_constraint(&c2);
    m.add_constraint(&c3);
    m.add_constraint(&c4);
    m.add_constraint(&c5);
     
    // Solver
    let mut s = SolverClpCmd::new();
    s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
    m.solve(&s).unwrap();
     
    // Status
    assert_eq!(*m.solver_status().unwrap(), SolverStatus::Solved);
    println!("Solver Status: {}\n", *m.solver_status().unwrap());
     
    // Primal results
    let final_primals = m.final_primals();
    assert_eq!(*final_primals.get(&x).unwrap(), 100.);
    println!("Primal Results:");
    println!("x* = {}", *final_primals.get(&x).unwrap());
    println!("y* = {}", *final_primals.get(&y).unwrap());
    assert_eq!(f.evaluate(&final_primals), -300.);
    println!("f(x*) = {}\n", f.evaluate(&final_primals));

    // Dual results
    let final_duals = m.final_duals();
    assert_eq!(*final_duals.get(&c1).unwrap(), 7.);
    assert_eq!(*final_duals.get(&c5).unwrap(), -5.);

    println!("Dual Results:");
    for (i, constraint) in constraints.iter().enumerate() {
        println!("Dual associated with c{}", i + 1);
        println!("{}", *final_duals.get(constraint).unwrap());
    }
}
