use std::collections::HashMap;

use crate::model::node::NodeRc;

pub struct NodePropData {
    pub affine: bool,
    pub a: HashMap<NodeRc, f64>,
    pub b: f64,
}

pub struct NodeCompData {
    pub phi: NodeRc,
    pub gphi: Vec<(NodeRc, NodeRc)>,
    pub Hphi: Vec<(NodeRc, NodeRc, NodeRc)>,
    pub prop: NodePropData,
}

// pub trait NodeProp {
//     fn reduce_props(&self, props: Vec<NodePropData>) -> NodePropData;
// }

// pub trait NodeStd {
//     fn properties(&self) -> NodePropData;
//     //fn components(&self) -> NodeCompData;
// }

// impl NodeProp for NodeRc {

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

// impl NodeStd for NodeRc {
    
//     fn properties(&self) -> NodePropData {


//     }
// }

