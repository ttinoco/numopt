use std::collections::HashSet;

use crate::model::node::Node;
use crate::model::node_std::{NodeStd, NodeStdComp};
use crate::model::constant::ConstantScalar;
use crate::model::constraint_std::{ConstraintStd, ConstraintStdComp};
use crate::model::problem::{Problem, Objective};

pub struct ProblemStdComp {
    pub obj: NodeStdComp,
    pub constr: ConstraintStdComp,
}

pub trait ProblemStd {
    fn std_components(&self) -> ProblemStdComp;
    fn std_problem(&self) -> ();
}

impl ProblemStd for Problem {

    fn std_components(&self) -> ProblemStdComp {

        // Objective std comp
        let obj = match self.objective() {
            Objective::Maximize(f) => f.std_components(),
            Objective::Minimize(f) => f.std_components(),
            Objective::Empty => ConstantScalar::new(0.).std_components(),
        };

        // Constraint std comp
        let mut arow: usize = 0;
        let mut jrow: usize = 0;
        let mut constr = ConstraintStdComp::new();
        for c in self.constraints().iter() {
            constr += c.std_components(&mut arow, &mut jrow);
        }

        // Return
        ProblemStdComp {
            obj: obj,
            constr: constr,
        }
    }

    fn std_problem(&self) -> () {

        // Components
        let comp = self.std_components();

        // Variables
        let varset: HashSet<Node> = comp.obj.prop.a.keys()
                                                   .map(|x| x.clone())
                                                   .collect();
        
    }
}