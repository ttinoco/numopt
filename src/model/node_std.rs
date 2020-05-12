use std::collections::HashMap;

use crate::model::node::Node;
use crate::model::node_base::NodeBase;
use crate::model::constant::ConstantScalar;

pub struct NodeStdProp {
    pub affine: bool,
    pub a: HashMap<Node, f64>,
    pub b: f64,
}

pub struct NodeStdComp {
    pub phi: Node,
    pub gphi: Vec<(Node, Node)>,
    pub Hphi: Vec<(Node, Node, Node)>,
    pub prop: NodeStdProp,
}

pub trait NodeStd {
    fn properties(&self) -> NodeStdProp {
        NodeStdProp {
            affine: false,
            a: HashMap::new(),
            b: 0.,
        }
    }
    fn components(&self) -> NodeStdComp {
        NodeStdComp {
            phi: ConstantScalar::new(0.),
            gphi: Vec::new(),
            Hphi: Vec::new(),
            prop: self.properties(),
        }
    }
}

impl NodeStd for Node {

    fn properties(&self) -> NodeStdProp {
        match self {
            Node::ConstantScalar(_x) => {
                NodeStdProp {
                    affine: true,
                    a: HashMap::new(),
                    b: self.value(),
                }
            },
            Node::VariableScalar(_x) => {
                let mut a: HashMap<Node, f64> = HashMap::new();
                a.insert(self.clone(), 1.);
                NodeStdProp {
                    affine: true,
                    a: a,
                    b: 0.,
                }
            },
            Node::FunctionAdd(x) => (**x).borrow().properties(),
            Node::FunctionCos(x) => (**x).borrow().properties(),
            Node::FunctionDiv(x) => (**x).borrow().properties(),
            Node::FunctionMul(x) => (**x).borrow().properties(),
            Node::FunctionSin(x) => (**x).borrow().properties(),
        }
    }
}