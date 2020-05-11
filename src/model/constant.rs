
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::node::{NodeBase, NodeRef};

pub struct ConstantScalar {
    value: f64,
}

impl ConstantScalar {

    pub fn new(value: f64) -> NodeRef {
        NodeRef::ConstantScalar(Rc::new(RefCell::new(
            Self {
                value: value,
            }
        )))
    }
}

impl NodeBase for ConstantScalar {

    fn partial(&self, _arg: &NodeRef) -> NodeRef { ConstantScalar::new(0.) }
    fn value(&self) -> f64 { self.value }
}

impl fmt::Display for ConstantScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {

    use crate::model::node::NodeBase;
    use crate::model::node_std::NodeStd;
    use crate::model::node_diff::NodeDiff;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

    #[test]
    fn partial() {

        let x = VariableScalar::new_continuous("x", 2.);
        let c = ConstantScalar::new(3.);

        let z1 = c.partial(&x);
        assert!(z1.is_constant_with_value(0.));

        let z2 = c.partial(&c);
        assert!(z2.is_constant_with_value(0.));
    }

    #[test]
    fn derivative() {

        let x = VariableScalar::new_continuous("x", 2.);
        let c = ConstantScalar::new(3.);

        let z1 = c.derivative(&x);
        assert!(z1.is_constant_with_value(0.));
    }

    #[test]
    fn properties() {

        let c = ConstantScalar::new(5.);
        let p = c.properties();
        assert!(p.affine);
        assert_eq!(p.b, 5.);
        assert!(p.a.is_empty());
    }
}