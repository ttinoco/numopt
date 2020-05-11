use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::model::node::{NodeBase, NodeRef};
use crate::model::node_std::{NodeStd, NodeStdProp};
use crate::model::node_func::NodeFunc;
use crate::model::constant::ConstantScalar;

pub struct FunctionSin {
    arg: NodeRef
}

impl FunctionSin {

    pub fn new(arg: NodeRef) -> NodeRef {
        NodeRef::FunctionSin(Rc::new(RefCell::new(
            Self {
                arg: arg,
            }
        )))
    }
}

impl NodeBase for FunctionSin {

    fn arguments(&self) -> Vec<NodeRef> {
        vec![self.arg.clone()]
    }

    fn partial(&self, arg: &NodeRef) -> NodeRef { 
        if self.arg == *arg {
            return self.arg.cos();
        }
        else {
            return ConstantScalar::new(0.);
        }
    }

    fn value(&self) -> f64 {
        self.arg.value().sin()
    }
}

impl NodeStd for FunctionSin {

    fn properties(&self) -> NodeStdProp {
        let mut p = self.arg.properties();
        p.affine = false;
        p
    }
}

impl<'a> fmt::Display for FunctionSin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sin({})", self.arg)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::node::NodeBase;
    use crate::model::node_diff::NodeDiff;
    use crate::model::variable::VariableScalar;

    #[test]
    fn partial() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("x", 4.);

        let z = x.cos();
        
        let z1 = z.partial(&x);
        assert_eq!(format!("{}", z1), "-1*sin(x)");
        
        let z2 = z.partial(&y);
        assert!(z2.is_constant_with_value(0.));
    }

    #[test]
    fn derivative() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = x.sin();
        let z1x = z1.derivative(&x);
        let z1y = z1.derivative(&y);
        assert_eq!(format!("{}", z1x), "cos(x)");
        assert!(z1y.is_constant_with_value(0.));

        let z2 = (5.*&x + 3.*&y).sin();
        let z2x = z2.derivative(&x);
        let z2y = z2.derivative(&y);
        assert_eq!(format!("{}", z2x), "cos(5*x + 3*y)*5");
        assert_eq!(format!("{}", z2y), "cos(5*x + 3*y)*3");
    }

    #[test]
    fn properties() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 5.);

        let z1 = &x.sin();
        let p1 = z1.properties();
        assert!(!p1.affine);
        assert_eq!(p1.a.len(), 1);
        assert!(p1.a.contains_key(&x));

        let z2 = 3.*(&x + &y).sin();
        let p2 = z2.properties();
        assert!(!p2.affine);
        assert_eq!(p2.a.len(), 2);
        assert!(p2.a.contains_key(&x));
        assert!(p2.a.contains_key(&y));
    }
}