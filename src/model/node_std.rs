use std::collections::HashMap;

use crate::model::node::Node;
use crate::model::node_base::NodeBase;
use crate::model::node_diff::NodeDiff;
use crate::model::constant::ConstantScalar;

pub struct NodeStdProp {
    pub affine: bool,
    pub a: HashMap<Node, f64>,
    pub b: f64,
}

pub struct NodeStdComp {
    pub phi: Node,
    pub gphi: Vec<(Node, Node)>,
    pub hphi: Vec<(Node, Node, Node)>,
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
            hphi: Vec::new(),
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

    fn components(&self) -> NodeStdComp {

        let phi = self.clone();
        let mut gphi: Vec<(Node, Node)> = Vec::new();
        let mut hphi: Vec<(Node, Node, Node)> = Vec::new();
        let prop = self.properties();

        // Affine
        if prop.affine {
            for (key, val) in prop.a.iter() {
                gphi.push((key.clone(), ConstantScalar::new(*val)));
            }
        }

        // Not affine
        else {
            let vars: Vec<&Node> = prop.a.keys().collect();
            let derivs = self.derivatives(&vars);
            for (i, var1) in vars.iter().enumerate() {
                let d = derivs.get(var1).unwrap();
                gphi.push(((*var1).clone(), d.clone()));
                let dvars: Vec<&Node> = vars.iter()
                                            .enumerate()
                                            .filter(|&(k,_)| k >= i)
                                            .map(|(_,v)| *v)
                                            .collect();
                let dderivs = d.derivatives(&dvars);
                for var2 in dvars.iter() {
                    let dd = dderivs.get(&var2).unwrap();
                    if !dd.is_constant_with_value(0.) {
                        hphi.push(((*var1).clone(), (*var2).clone(), dd.clone()));
                    }
                }                     
            }
        }

        // Return
        NodeStdComp {
            phi: phi,
            gphi: gphi,
            hphi: hphi,
            prop: prop
        }
    }
}