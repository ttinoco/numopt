use std::fmt;
use std::rc::Rc;

use crate::model::node::{Node,
                         NodeRc};
use crate::model::constant::ConstantScalar;

pub struct FunctionDiv {
    args: (NodeRc, NodeRc),
}

impl FunctionDiv {

    pub fn new(arg1: NodeRc, arg2: NodeRc) -> NodeRc {
        NodeRc::FunctionDivRc(Rc::new(
            Self {
                args: (arg1, arg2),
            }
        ))
    }
}

impl Node for FunctionDiv {

    fn arguments(&self) -> Vec<NodeRc> {
        vec![self.args.0.clone(), self.args.1.clone()]
    }

    fn partial(&self, arg: &NodeRc) -> NodeRc { 
        if self.args.0 == *arg {
            return 1./&self.args.1;
        }
        else if self.args.1 == *arg {
            return -&self.args.0/(&self.args.1*&self.args.1);
        }
        else {
            return ConstantScalar::new(0.);
        }
    }

    fn value(&self) -> f64 { 
        self.args.0.value()/self.args.1.value()
    }
}

impl<'a> fmt::Display for FunctionDiv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s0 = match &self.args.0 {
            NodeRc::FunctionAddRc(x) => format!("({})", x),
            NodeRc::FunctionDivRc(x) => format!("({})", x),
            _ => format!("{}", self.args.0)
        };
        let s1 = match &self.args.1 {
            NodeRc::FunctionAddRc(x) => format!("({})", x),
            NodeRc::FunctionMulRc(x) => format!("({})", x),
            NodeRc::FunctionDivRc(x) => format!("({})", x),
            _ => format!("{}", self.args.1)
        };
        write!(f, "{}/{}", s0, s1)
    }
}

#[cfg(test)]
mod tests {

    use crate::model::node::Node;
    use crate::model::variable::VariableScalar;

    #[test]
    fn partial() {

        let x = VariableScalar::new_continuous("x", 2.);
        let y = VariableScalar::new_continuous("y", 3.);
        let w = VariableScalar::new_continuous("w", 4.);

        let z = &x/&y;

        let z1 = z.partial(&x);
        assert_eq!(format!("{}", z1), "1/y");

        let z2 = z.partial(&y);
        assert_eq!(format!("{}", z2), "-1*x/(y*y)");

        let z3 = z.partial(&w);
        assert!(z3.is_constant_with_value(0.));
    }
}


