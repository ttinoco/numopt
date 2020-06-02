use numopt::model::*;
use numopt::solver::*;

#[test]
fn numopt_model_solve_milp_cbc_cmd() {

    let x1 = VariableScalar::new_integer("x1");
    let x2 = VariableScalar::new_integer("x2");
    let x3 = VariableScalar::new_continuous("x3");
    let x4 = VariableScalar::new_continuous("x4");

    let f = -&x1 - &x2;
    let c1 = (-2.*&x1+ 2.*&x2 + &x3).equal(1.);
    let c2 = (-8.*&x1 + 10.*&x2 + &x4).equal(13.);
    let c3 = &x4.geq(0.);
    let c4 = &x3.leq(0.);

    let mut m = Model::new();
    m.set_objective(Objective::minimize(&f));
    m.add_constraint(&c1);
    m.add_constraint(&c2);
    m.add_constraint(&c3);
    m.add_constraint(&c4);

    let mut s = SolverCbcCmd::new();
    s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
    m.solve(&s).unwrap();

    assert_eq!(*m.solver_status().unwrap(), SolverStatus::Solved);
    assert_eq!(*m.final_primals().get(&x1).unwrap(), 1.);
    assert_eq!(*m.final_primals().get(&x2).unwrap(), 2.);
}

#[cfg(feature = "ipopt")] 
#[test]
fn numopt_model_solve_nlp_ipopt() {

    use approx::assert_abs_diff_eq;

    const N: usize = 500;

    let x: Vec<Node>= (0..N).map(|i| VariableScalar::new_continuous(format!("x{}", i).as_ref()))
                            .collect();

    let mut f = ConstantScalar::zero();
    for i in 0..(N-1) {
        f = f + 100.*(&x[i+1] - &x[i]*&x[i])*(&x[i+1] - &x[i]*&x[i]) + (1. - &x[i])*(1. - &x[i]);
    }

    let mut m = Model::new();
    m.set_objective(Objective::minimize(&f));

    let mut s = SolverIpopt::new();
    s.set_param("print_level", SolverParam::IntParam(5)).unwrap();
    s.set_param("sb", SolverParam::StrParam("yes".to_string())).unwrap();
    m.solve(&s).unwrap();

    assert_eq!(*m.solver_status().unwrap(), SolverStatus::Solved);
    let final_primals = m.final_primals();
    for xx in x.iter() {
        assert_abs_diff_eq!(*final_primals.get(xx).unwrap(), 1., epsilon=1e-8);
    }
}