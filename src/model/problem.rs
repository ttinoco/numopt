use std::fmt;
use std::collections::HashMap;

use crate::model::node::Node;
use crate::model::constraint::Constraint;

pub enum Objective {
    Minimize(Node),
    Maximize(Node),
    Empty,
}

pub struct Problem {

    objective: Objective,
    constraints: Vec<Constraint>,
    init_values: HashMap<Node, f64>,
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

impl Problem {

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

    pub fn new() -> Problem {
        Problem {
            objective: Objective::empty(),
            constraints: Vec::new(),
            init_values: HashMap::new(),
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
}

impl<'a> fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.objective {
            Objective::Minimize(x) => write!(f, "\nMinimize {}\n\n", x).unwrap(),
            Objective::Maximize(x) => write!(f, "\nMaximize {}\n\n", x).unwrap(),
            Objective::Empty => write!(f, "\nFind point\n\n").unwrap(),
        };
        if self.constraints.len() > 0 {
            write!(f, "Subject to\n").unwrap();
            for c in self.constraints.iter() {
                write!(f, "{} : {}\n", c, c.label()).unwrap();
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

        let mut p = Problem::new();
        p.set_objective(Objective::minimize(&f));
        p.add_constraints(&vec!(&c1, &c2, &c3));

        let refstr = "\nMinimize 4*cos(x) + y\n\n\
                      Subject to\n\
                      x + y >= 0 : comb limit\n\
                      x >= 0 : x limit\n\
                      y >= 0 : y limit\n";

        assert_eq!(refstr, format!("{}", p));
    }
}