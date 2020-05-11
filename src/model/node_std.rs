use std::collections::HashMap;

use crate::model::node::{NodeBase, NodeRef};

pub struct NodeStdProp {
    pub affine: bool,
    pub a: HashMap<NodeRef, f64>,
    pub b: f64,
}

pub struct NodeStdComp {
    pub phi: NodeRef,
    pub gphi: Vec<(NodeRef, NodeRef)>,
    pub Hphi: Vec<(NodeRef, NodeRef, NodeRef)>,
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
    //fn components(&self) -> NodeStdProp;
}

impl NodeStd for NodeRef {

    fn properties(&self) -> NodeStdProp {
        match self {
            NodeRef::ConstantScalar(_x) => {
                NodeStdProp {
                    affine: true,
                    a: HashMap::new(),
                    b: self.value(),
                }
            },
            NodeRef::VariableScalar(_x) => {
                let mut a: HashMap<NodeRef, f64> = HashMap::new();
                a.insert(self.clone(), 1.);
                NodeStdProp {
                    affine: true,
                    a: a,
                    b: 0.,
                }
            },
            NodeRef::FunctionAdd(x) => (**x).borrow().properties(),
            NodeRef::FunctionCos(x) => (**x).borrow().properties(),
            NodeRef::FunctionDiv(x) => (**x).borrow().properties(),
            NodeRef::FunctionMul(x) => (**x).borrow().properties(),
            NodeRef::FunctionSin(x) => (**x).borrow().properties(),
        }
    }
}