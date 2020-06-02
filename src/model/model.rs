use std::fmt;
use std::collections::HashMap;
use simple_error::SimpleError;

use crate::solver::base::{Solver, SolverStatus};

use crate::model::node::Node;
use crate::model::constraint::Constraint;
use crate::model::model_std::ModelStd;

pub enum Objective {
    Minimize(Node),
    Maximize(Node),
    Empty,
}

pub struct Model {

    objective: Objective,
    constraints: Vec<Constraint>,
    init_primals: HashMap<Node, f64>,
    solver_status: Option<SolverStatus>,
    final_primals: HashMap<Node, f64>,
    final_duals: HashMap<Constraint, f64>,
}

impl Objective {

    pub fn minimize(f: &Node) -> Objective {
        Objective::Minimize(f.clone())
    }

    pub fn maximize(f: &Node) -> Objective {
        Objective::Maximize(f.clone())
    }

    pub fn empty() -> Objective {
        Objective::Empty
    }
}

impl Model {

    pub fn add_constraint(&mut self, c: &Constraint) -> () {
        self.constraints.push(c.clone())
    }

    pub fn add_constraints(&mut self, c: &[&Constraint]) -> () {
        self.constraints.extend(c.iter()
                                 .map(|cc| (*cc).clone())
                                 .collect::<Vec<Constraint>>());
    }

    pub fn constraints(&self) -> &Vec<Constraint> { &self.constraints }

    pub fn final_primals(&self) -> HashMap<&Node, f64> { 
         self.final_primals.iter().map(|(var, val)| (var, *val)).collect()
    }

    pub fn final_duals(&self) -> HashMap<&Constraint, f64> { 
        self.final_duals.iter().map(|(c, val)| (c, *val)).collect()
    }

    pub fn init_primals(&self) -> HashMap<&Node, f64> { 
        self.init_primals.iter().map(|(var, val)| (var, *val)).collect()
    }

    pub fn new() -> Model {
        Model {
            objective: Objective::empty(),
            constraints: Vec::new(),
            init_primals: HashMap::new(),
            solver_status: None,
            final_primals: HashMap::new(),
            final_duals: HashMap::new(),
        }
    }

    pub fn objective(&self) -> &Objective { &self.objective }

    pub fn set_objective(&mut self, obj: Objective) -> () {
        self.objective = obj;
    }

    pub fn set_init_primals(&mut self, values: &HashMap<&Node, f64>) -> () {
        self.init_primals.clear();
        for (key, val) in values.iter() {
            self.init_primals.insert((*key).clone(), *val);
        }
    }

    pub fn solve(&mut self, solver: &dyn Solver) -> Result<(), SimpleError> {

        // Reset
        self.final_primals.clear();
        self.final_duals.clear();
        self.solver_status = None;

        // Construct
        let mut std_prob = self.std_problem();
        
        // Solve
        let (status, solution) = solver.solve(&mut std_prob.prob)?;
       
        // Status
        self.solver_status = Some(status);

        // Final var values
        for (var, index) in std_prob.var2index.iter() {
            self.final_primals.insert(var.clone(), solution.x[*index]);
        }

        // Final constr duals
        for (index, constr) in std_prob.aindex2constr.iter() {
            self.final_duals.insert(constr.clone(), solution.lam[*index]);
        }
        for (index, constr) in std_prob.jindex2constr.iter() {
            self.final_duals.insert(constr.clone(), solution.nu[*index]);
        }
        for (index, constr) in std_prob.uindex2constr.iter() {
            self.final_duals.insert(constr.clone(), solution.mu[*index]);
        }
        for (index, constr) in std_prob.lindex2constr.iter() {
            self.final_duals.insert(constr.clone(), solution.pi[*index]);
        }

        // Done
        Ok(())
    }

    pub fn solver_status(&self) -> Option<&SolverStatus> {
        match &self.solver_status {
            Some(x) => Some(&x),
            None => None,
        }
    }
}

impl<'a> fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.objective {
            Objective::Minimize(_) => write!(f, "\nMinimize\n")?,
            Objective::Maximize(_) => write!(f, "\nMaximize\n")?,
            Objective::Empty => write!(f, "\nFind point\n")?,
        };
        match &self.objective {
            Objective::Minimize(x) => write!(f, "{}\n", x)?,
            Objective::Maximize(x) => write!(f, "{}\n", x)?,
            Objective::Empty => write!(f, "\n")?,
        };
        if self.constraints.len() > 0 {
            write!(f, "\nSubject to\n")?;
            for c in self.constraints.iter() {
                if c.label() != "" {
                    write!(f, "{} : {}\n", c, c.label())?;
                }
                else {
                    write!(f, "{}\n", c)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use serial_test::serial;
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::solver::base::SolverParam;
    use crate::solver::clp_cmd::SolverClpCmd;
    use crate::solver::cbc_cmd::SolverCbcCmd;
    use crate::model::node_cmp::NodeCmp;
    use crate::model::node_func::NodeFunc;
    use crate::model::variable::VariableScalar;

    #[test]
    fn model_display() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let f = 4.*x.cos() + &y;
        let c1 = (&x + &y).geq_and_tag(0., "comb limit");
        let c2 = (&x).geq_and_tag(0., "x limit");
        let c3 = (&y).geq_and_tag(0., "y limit");

        let mut m = Model::new();
        m.set_objective(Objective::minimize(&f));
        m.add_constraints(&vec!(&c1, &c2, &c3));

        let refstr = "\nMinimize\n\
                      4*cos(x) + y\n\n\
                      Subject to\n\
                      x + y >= 0 : comb limit\n\
                      x >= 0 : x limit\n\
                      y >= 0 : y limit\n";

        assert_eq!(refstr, format!("{}", m));
    }

    #[test]
    #[serial]
    fn model_solve_lp_clp_cmd() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let mut m = Model::new();
        m.set_objective(Objective::maximize(&(-2.*&x + 5.*&y)));
        m.add_constraint(&(100_f64.leq(&x)));
        m.add_constraint(&(&x).leq(200.));
        m.add_constraint(&(80_f64.leq(&y)));
        m.add_constraint(&(&y).leq(170.));
        m.add_constraint(&(&y).geq(-&x + 200.));

        let mut solver = SolverClpCmd::new();
        solver.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&solver).unwrap();

        let status = m.solver_status().unwrap();
        assert_eq!(*status, SolverStatus::Solved);
    
        let final_primals = m.final_primals();
        assert_abs_diff_eq!(*final_primals.get(&x).unwrap(), 100., epsilon = 1e-5);
        assert_abs_diff_eq!(*final_primals.get(&y).unwrap(), 170., epsilon = 1e-5);

        // Solve again
        m.solve(&solver).unwrap();

        let status = m.solver_status().unwrap();
        assert_eq!(*status, SolverStatus::Solved);
    
        let final_primals = m.final_primals();
        assert_abs_diff_eq!(*final_primals.get(&x).unwrap(), 100., epsilon = 1e-5);
        assert_abs_diff_eq!(*final_primals.get(&y).unwrap(), 170., epsilon = 1e-5);
    }

    #[test]
    #[serial]
    fn model_solve_lp_cbc_cmd() {

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let mut m = Model::new();
        m.set_objective(Objective::maximize(&(-2.*&x + 5.*&y)));
        m.add_constraint(&(100_f64.leq(&x)));
        m.add_constraint(&(&x).leq(200.));
        m.add_constraint(&(80_f64.leq(&y)));
        m.add_constraint(&(&y).leq(170.));
        m.add_constraint(&(&y).geq(-&x + 200.));

        let mut solver = SolverCbcCmd::new();
        solver.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&solver).unwrap();

        let status = m.solver_status().unwrap();
        assert_eq!(*status, SolverStatus::Solved);
    
        let final_primals = m.final_primals();
        assert_abs_diff_eq!(*final_primals.get(&x).unwrap(), 100., epsilon = 1e-5);
        assert_abs_diff_eq!(*final_primals.get(&y).unwrap(), 170., epsilon = 1e-5);
    }

    #[cfg(feature = "ipopt")] 
    #[test]
    #[serial]
    fn model_solve_lp_ipopt() {

        use crate::solver::ipopt::SolverIpopt;

        let x = VariableScalar::new_continuous("x");
        let y = VariableScalar::new_continuous("y");

        let mut m = Model::new();
        m.set_objective(Objective::maximize(&(-2.*&x + 5.*&y)));
        m.add_constraint(&(100_f64.leq(&x)));
        m.add_constraint(&(&x).leq(200.));
        m.add_constraint(&(80_f64.leq(&y)));
        m.add_constraint(&(&y).leq(170.));
        m.add_constraint(&(&y).geq(-&x + 200.));

        let mut solver = SolverIpopt::new();
        solver.set_param("print_level", SolverParam::IntParam(0)).unwrap();
        solver.set_param("sb", SolverParam::StrParam("yes".to_string())).unwrap();
        m.solve(&solver).unwrap();

        let status = m.solver_status().unwrap();
        assert_eq!(*status, SolverStatus::Solved);
    
        let final_primals = m.final_primals();
        assert_abs_diff_eq!(*final_primals.get(&x).unwrap(), 100., epsilon = 1e-5);
        assert_abs_diff_eq!(*final_primals.get(&y).unwrap(), 170., epsilon = 1e-5);
    }

    #[test]
    #[serial]
    fn model_infeas_lp_clc_cmd() {

        let x = VariableScalar::new_continuous("x");
        
        let f = &x + 4.;
        let c1 = &x.geq(4.);
        let c2 = &x.leq(3.);

        let mut m = Model::new();
        m.set_objective(Objective::minimize(&f));
        m.add_constraint(&c1);
        m.add_constraint(&c2);

        let mut s = SolverClpCmd::new();
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&s).unwrap();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Infeasible);

    }

    #[test]
    #[serial]
    fn model_infeas_lp_cbc_cmd() {

        let x = VariableScalar::new_continuous("x");
        
        let f = &x + 4.;
        let c1 = &x.geq(4.);
        let c2 = &x.leq(3.);

        let mut m = Model::new();
        m.set_objective(Objective::minimize(&f));
        m.add_constraint(&c1);
        m.add_constraint(&c2);

        let mut s = SolverCbcCmd::new();
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&s).unwrap();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Infeasible);        
    }

    #[cfg(feature = "ipopt")] 
    #[test]
    #[serial]
    fn model_infeas_lp_ipopt() {

        use crate::solver::ipopt::SolverIpopt;

        let x = VariableScalar::new_continuous("x");
        
        let f = &x + 4.;
        let c1 = &x.geq(4.);
        let c2 = &x.leq(3.);

        let mut m = Model::new();
        m.set_objective(Objective::minimize(&f));
        m.add_constraint(&c1);
        m.add_constraint(&c2);

        let mut s = SolverIpopt::new();
        s.set_param("print_level", SolverParam::IntParam(0)).unwrap();
        s.set_param("sb", SolverParam::StrParam("yes".to_string())).unwrap();
        m.solve(&s).unwrap();
        
        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Error); // bad bounds
        
        let c3 = (2.*&x + 10.).leq(4.);
        let c4 = (2.*&x + 10.).geq(5.);

        let mut m = Model::new();
        m.set_objective(Objective::minimize(&f));
        m.add_constraint(&c3);
        m.add_constraint(&c4);

        m.solve(&s).unwrap();
        
        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Infeasible);        
    }

    #[test]
    #[serial]
    fn model_noobj_lp_clp_cmd() {

        let x = VariableScalar::new_continuous("x");
        
        let c1 = &x.geq(2.);
        let c2 = &x.leq(5.);

        let mut m = Model::new();
        m.add_constraint(&c1);
        m.add_constraint(&c2);

        let mut s = SolverClpCmd::new();
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&s).unwrap();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Error);
    }

    #[test]
    #[serial]
    fn model_noobj_lp_cbc_cmd() {

        let x = VariableScalar::new_continuous("x");
        
        let c1 = &x.geq(2.);
        let c2 = &x.leq(5.);

        let mut m = Model::new();
        m.add_constraint(&c1);
        m.add_constraint(&c2);

        let mut s = SolverCbcCmd::new();
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&s).unwrap();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Error);
    }

    #[cfg(feature = "ipopt")] 
    #[test]
    #[serial]
    fn model_noobj_lp_ipopt() {

        use crate::solver::ipopt::SolverIpopt;

        let x = VariableScalar::new_continuous("x");
        
        let c1 = &x.geq(2.);
        let c2 = &x.leq(5.);

        let mut m = Model::new();
        m.add_constraint(&c1);
        m.add_constraint(&c2);

        let mut s = SolverIpopt::new();
        s.set_param("print_level", SolverParam::IntParam(0)).unwrap();
        s.set_param("sb", SolverParam::StrParam("yes".to_string())).unwrap();
        m.solve(&s).unwrap();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Solved);
    }

    #[test]
    #[serial]
    fn model_solve_milp_cbc_cmd() {

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

    #[test]
    #[serial]
    fn model_infeas_milp_cbc_cmd() {

        let x1 = VariableScalar::new_integer("x1");
        let x2 = VariableScalar::new_integer("x2");
        let x3 = VariableScalar::new_continuous("x3");
        let x4 = VariableScalar::new_continuous("x4");

        let f = -&x1 - &x2;
        let c1 = (-2.*&x1+ 2.*&x2 + &x3).equal(1.);
        let c2 = (-8.*&x1 + 10.*&x2 + &x4).equal(13.);
        let c3 = &x4.geq(0.);
        let c4 = &x3.leq(0.);
        let c5 = (-2.*&x1+ 2.*&x2 + &x3).equal(3.);

        let mut m = Model::new();
        m.set_objective(Objective::minimize(&f));
        m.add_constraint(&c1);
        m.add_constraint(&c2);
        m.add_constraint(&c3);
        m.add_constraint(&c4);
        m.add_constraint(&c5);

        let mut s = SolverCbcCmd::new();
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&s).unwrap();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Infeasible);
    }

    #[test]
    #[serial]
    fn model_noobj_milp_cbc_cmd() {

        let x1 = VariableScalar::new_integer("x1");
        let x2 = VariableScalar::new_integer("x2");
        let x3 = VariableScalar::new_continuous("x3");
        let x4 = VariableScalar::new_continuous("x4");

        let c1 = (-2.*&x1+ 2.*&x2 + &x3).equal(1.);
        let c2 = (-8.*&x1 + 10.*&x2 + &x4).equal(13.);
        let c3 = &x4.geq(0.);
        let c4 = &x3.leq(0.);
        let c5 = &x1.leq(10.);
        let c6 = &x2.leq(10.);
        let c7 = &x1.geq(1.);
        let c8 = &x2.geq(1.);

        let mut m = Model::new();
        m.add_constraint(&c1);
        m.add_constraint(&c2);
        m.add_constraint(&c3);
        m.add_constraint(&c4);
        m.add_constraint(&c5);
        m.add_constraint(&c6);
        m.add_constraint(&c7);
        m.add_constraint(&c8);

        let mut s = SolverCbcCmd::new();
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        m.solve(&s).unwrap();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Solved);

        let final_primals = m.final_primals();
        assert_eq!(*final_primals.get(&x1).unwrap(), 1.);
        assert_eq!(*final_primals.get(&x2).unwrap(), 2.);
        assert_eq!(*final_primals.get(&x3).unwrap(), -1.);
        assert_eq!(*final_primals.get(&x4).unwrap(), 1.);
        assert_eq!(c1.violation(&final_primals), 0.);
        assert_eq!(c2.violation(&final_primals), 0.);
        assert_eq!(c3.violation(&final_primals), 0.);
        assert_eq!(c4.violation(&final_primals), 0.);
        assert_eq!(c5.violation(&final_primals), 0.);
        assert_eq!(c6.violation(&final_primals), 0.);
        assert_eq!(c7.violation(&final_primals), 0.);
        assert_eq!(c8.violation(&final_primals), 0.);
    }

    #[cfg(feature = "ipopt")] 
    #[test]
    #[serial]
    fn model_solve_nlp_ipopt() {

        use maplit::hashmap;
        use crate::model::node_base::NodeBase;
        use crate::solver::ipopt::SolverIpopt;

        // Hock-Schittkowski
        // Problem 71

        let x1 = VariableScalar::new_continuous("x1");
        let x2 = VariableScalar::new_continuous("x2");
        let x3 = VariableScalar::new_continuous("x3");
        let x4 = VariableScalar::new_continuous("x4");

        let f = &x1*&x4*(&x1+&x2+&x3) + &x3;

        let mut m = Model::new();
        m.set_objective(Objective::minimize(&f));
        m.add_constraints(&vec![
            &(&x1*&x2*&x3*&x4).geq(25.),
            &(&x1*&x1 + &x2*&x2 + &x3*&x3 + &x4*&x4).equal(40.),
            &x1.geq(1.), &x1.leq(5.),
            &x2.geq(1.), &x2.leq(5.),
            &x3.geq(1.), &x3.leq(5.),
            &x4.geq(1.), &x4.leq(5.),
        ]);
        m.set_init_primals(&hashmap!{
            &x1 => 1.,
            &x2 => 5.,
            &x3 => 5.,
            &x4 => 1.,
        });

        let mut s = SolverIpopt::new();
        s.set_param("print_level", SolverParam::IntParam(0)).unwrap();
        s.set_param("sb", SolverParam::StrParam("yes".to_string())).unwrap();
        m.solve(&s).unwrap();

        let final_primals = m.final_primals();

        assert_eq!(*m.solver_status().unwrap(), SolverStatus::Solved);
        assert_abs_diff_eq!(f.evaluate(&final_primals), 17.0140173, epsilon = 1e-4);
        assert_abs_diff_eq!(*final_primals.get(&x1).unwrap(), 1., epsilon = 1e-4);
        assert_abs_diff_eq!(*final_primals.get(&x2).unwrap(), 4.7429994, epsilon = 1e-4);
        assert_abs_diff_eq!(*final_primals.get(&x3).unwrap(), 3.8211503, epsilon = 1e-4);
        assert_abs_diff_eq!(*final_primals.get(&x4).unwrap(), 1.3794082, epsilon = 1e-4);
    }

    #[cfg(feature = "ipopt")] 
    #[test]
    #[serial]
    fn model_infeas_nlp_ipopt() {


    }

    #[cfg(feature = "ipopt")] 
    #[test]
    #[serial]
    fn model_noobj_nlp_ipopt() {

    }
}