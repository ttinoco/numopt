use std::collections::HashMap;

use crate::model::node::NodeRef;

pub struct NodePropData {
    pub affine: bool,
    pub a: HashMap<NodeRef, f64>,
    pub b: f64,
}

pub struct NodeCompData {
    pub phi: NodeRef,
    pub gphi: Vec<(NodeRef, NodeRef)>,
    pub Hphi: Vec<(NodeRef, NodeRef, NodeRef)>,
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

