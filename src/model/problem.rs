
use std::fmt;

use crate::model::node::NodeRef;
use crate::model::constraint::Constraint;

pub enum Objective {
    Minimize(NodeRef),
    Maximize(NodeRef),
    Empty,
}

pub struct Problem {

    objective: Objective,
    constraints: Vec<Constraint>,
    // need label -> &constraint hashmap
}

impl Objective {

    pub fn minimize(f: &NodeRef) -> Objective {
        Objective::Minimize(f.clone())
    }

    pub fn maximize(f: &NodeRef) -> Objective {
        Objective::Maximize(f.clone())
    }

    pub fn empty() -> Objective {
        Objective::Empty
    }

}

impl Problem {

    pub fn new(objective: Objective, constraints: Vec<Constraint>) -> Problem {
        Problem {
            objective: objective,
            constraints: constraints,
        }
    }

    pub fn add_constraint(&mut self, c: Constraint) -> () {
        self.constraints.push(c)
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

        let x = VariableScalar::new_continuous("x", 4.);
        let y = VariableScalar::new_continuous("y", 5.);

        let f = 4.*x.cos() + &y;
        let c1 = (&x + &y).geq_and_tag(0., "comb limit");
        let c2 = (&x).geq_and_tag(0., "x limit");
        let c3 = (&y).geq_and_tag(0., "y limit");

        let p = Problem::new(
            Objective::minimize(&f),
            vec!(c1, c2, c3),
        );

        let refstr = "\nMinimize 4*cos(x) + y\n\n\
                      Subject to\n\
                      x + y >= 0 : comb limit\n\
                      x >= 0 : x limit\n\
                      y >= 0 : y limit\n";

        assert_eq!(refstr, format!("{}", p));
    }
}