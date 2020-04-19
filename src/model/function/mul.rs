
use std::fmt;
use std::rc::Rc;

use crate::model::node::{Node,
                         NodeRc};

pub struct FunctionMul {
    args: (NodeRc, NodeRc),
}

impl FunctionMul {

    pub fn new(arg1: NodeRc, arg2: NodeRc) -> NodeRc {
        NodeRc::FunctionMulRc(Rc::new(
            Self {
                args: (arg1, arg2),
            }
        ))
    }
}

impl Node for FunctionMul {

    fn get_value(&self) -> f64 { 
        self.args.0.get_value()*self.args.1.get_value()
    }
}

impl<'a> fmt::Display for FunctionMul {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s0 = match &self.args.0 {
            NodeRc::FunctionAddRc(x) => format!("({})", x),
            _ => format!("{}", self.args.0)
        };
        let s1 = match &self.args.1 {
            NodeRc::FunctionAddRc(x) => format!("({})", x),
            _ => format!("{}", self.args.1)
        };
        write!(f, "{}*{}", s0, s1)
    }
}



