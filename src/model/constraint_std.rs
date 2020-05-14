use crate::model::node::Node;
use crate::model::node_std::NodeStdProp;
use crate::model::constraint::Constraint;

pub struct ConstraintStdComp {
    pub cA: Vec<usize>,
    pub cJ: Vec<usize>,
    pub A: Vec<(usize, Node, f64)>,
    pub b: Vec<f64>,
    pub f: Vec<Node>,
    pub J: Vec<(usize, Node, Node)>,
    pub H: Vec<(Node, Node, Node)>,
    pub u: Vec<(Node, f64, usize)>,
    pub l: Vec<(Node, f64, usize)>,
    pub prop: Vec<NodeStdProp>,
}

pub trait ConstraintStd {
    fn components(&self) -> ConstraintStdComp;
}

// impl ConstraintStd for Constraint {

//     fn components(&self) -> ConstraintStdComp {

//     }
// }