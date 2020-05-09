use std::collections::HashMap;

use crate::model::node::{NodeRef, NodeProp};

pub struct NodeStdComp {
    pub phi: NodeRef,
    pub gphi: Vec<(NodeRef, NodeRef)>,
    pub Hphi: Vec<(NodeRef, NodeRef, NodeRef)>,
    pub prop: NodeProp,
}

pub trait NodeStd {
    fn components(&self) -> NodeStdComp;
}


// impl NodeStd for NodeRef {

//     fn reduce_props(&self, props: Vec<NodePropData>) -> NodePropData {
//         match self {
//             NodeRc::ConstantScalarRc(x) => x.reduce_props(props),
//             NodeRc::VariableScalarRc(x) => x.reduce_props(props),
//             NodeRc::FunctionAddRc(x) => x.reduce_props(props),
//             NodeRc::FunctionCosRc(x) => x.reduce_props(props),
//             NodeRc::FunctionDivRc(x) => x.reduce_props(props),
//             NodeRc::FunctionMulRc(x) => x.reduce_props(props),
//             NodeRc::FunctionSinRc(x) => x.reduce_props(props),
//         }
//     }
// }