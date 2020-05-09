use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::model::node::{NodeBase, NodeRef};
use crate::model::constant::ConstantScalar;

pub struct FunctionDiv {
    value: f64,
    args: (NodeRef, NodeRef),
}

impl FunctionDiv {

    pub fn new(arg1: NodeRef, arg2: NodeRef) -> NodeRef {
        NodeRef::FunctionDiv(Rc::new(RefCell::new(
            Self {
                value: 0.,
                args: (arg1, arg2),
            }
        )))
    }
}

impl NodeBase for FunctionDiv {

    fn arguments(&self) -> Vec<NodeRef> {
        vec![self.args.0.clone(), self.args.1.clone()]
    }

    fn partial(&self, arg: &NodeRef) -> NodeRef { 
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
            NodeRef::FunctionAdd(x) => format!("({})", (**x).borrow()),
            NodeRef::FunctionDiv(x) => format!("({})", (**x).borrow()),
            _ => format!("{}", self.args.0)
        };
        let s1 = match &self.args.1 {
            NodeRef::FunctionAdd(x) => format!("({})", (**x).borrow()),
            NodeRef::FunctionMul(x) => format!("({})", (**x).borrow()),
            NodeRef::FunctionDiv(x) => format!("({})", (**x).borrow()),
            _ => format!("{}", self.args.1)
        };
        write!(f, "{}/{}", s0, s1)
    }
}

#[cfg(test)]
mod tests {

    use crate::model::node::NodeBase;
    use crate::model::node_diff::NodeDiff;
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

    #[test]
    fn derivative() {

        let x = VariableScalar::new_continuous("x", 2.);
        let y = VariableScalar::new_continuous("y", 3.);

        let z1 = &x/3.;
        let z1x = z1.derivative(&x);
        let z1y = z1.derivative(&y);
        assert!(z1x.is_constant_with_value(1./3.));
        assert!(z1y.is_constant_with_value(0.));

        let z2 = 4./&x;
        let z2x = z2.derivative(&x);
        let z2y = z2.derivative(&y);
        assert_eq!(format!("{}", z2x), "-4/(x*x)");
        assert!(z2y.is_constant_with_value(0.));

        let z3 = 5./-&y;
        let z3x = z3.derivative(&x);
        let z3y = z3.derivative(&y);
        assert!(z3x.is_constant_with_value(0.));
        assert_eq!(format!("{}", z3y), "(-5/(-1*y*-1*y))*-1");

        let z4 = 3.*&x/(&y - &x);
        let z4x = z4.derivative(&x);
        let z4y = z4.derivative(&y);
        assert_eq!(format!("{}", z4x), 
                  "(-1*3*x/((y + -1*x)*(y + -1*x)))*-1 + (1/(y + -1*x))*3"); 
        assert_eq!(format!("{}", z4y), 
                  "-1*3*x/((y + -1*x)*(y + -1*x))");

        let f1 = &x - 2.;
        let z5 = &f1/(&f1 + 3.);
        let z5x = z5.derivative(&x);
        assert_eq!(format!("{}", z5x),
                   "(-1*x + 2)/((x + -2 + 3)*(x + -2 + 3)) + 1/(x + -2 + 3)");
    }
}


