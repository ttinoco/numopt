use std::fmt;
use std::collections::HashMap;
use simple_error::SimpleError;

use crate::solver::{Solver, SolverStatus};
use crate::problem::{Problem, ProblemSol};

use crate::model::node::Node;
use crate::model::constraint::Constraint;
use crate::model::model_std::{ModelStd, ModelStdProb, ModelStdMaps};

pub enum Objective {
    Minimize(Node),
    Maximize(Node),
    Empty,
}

pub struct Model {

    objective: Objective,
    constraints: Vec<Constraint>,
    init_values: HashMap<Node, f64>,
    std_prob: Option<ModelStdProb>,
    std_maps: Option<ModelStdMaps>,
    solver_status: Option<SolverStatus>,
    solution: Option<ProblemSol>,
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

    pub fn init_values(&self) -> &HashMap<Node, f64> { &self.init_values }

    pub fn new() -> Model {
        Model {
            objective: Objective::empty(),
            constraints: Vec::new(),
            init_values: HashMap::new(),
            std_maps: None,
            std_prob: None,
            solver_status: None,
            solution: None,
        }
    }

    pub fn objective(&self) -> &Objective { &self.objective }

    pub fn set_objective(&mut self, obj: Objective) -> () {
        self.objective = obj;
    }

    pub fn set_init_values(&mut self, init_values: &HashMap<&Node, f64>) -> () {
        self.init_values.clear();
        for (key, val) in init_values.iter() {
            self.init_values.insert((*key).clone(), *val);
        }
    }

    pub fn solve(&mut self, solver: &dyn Solver) -> Result<(), SimpleError> {

        // Reset
        self.std_prob = None;
        self.std_maps = None;
        self.solver_status = None;
        self.solution = None;

        // Construct
        let (std_prob, std_maps) = self.std_problem();
        
        // Solve
        let (status, solution) = match std_prob {
             ModelStdProb::Minlp(x) => solver.solve(&mut Problem::Minlp(x))?,
             ModelStdProb::Lp(x) => solver.solve(&mut Problem::Lp(x))?,
             ModelStdProb::Milp(x) => solver.solve(&mut Problem::Milp(x))?,
             ModelStdProb::Nlp(x) => solver.solve(&mut Problem::Nlp(x))?,
        };

        // Done
        Ok(())
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

    use super::*;
    use crate::model::node_cmp::NodeCmp;
    use crate::model::node_func::NodeFunc;
    use crate::model::variable::VariableScalar;

    #[test]
    fn display() {

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
}