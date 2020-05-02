
use std::fmt;
use std::rc::Rc;

use super::node::{Node,
                  NodeRc};

pub struct ConstantScalar {
    value: f64,
}

impl ConstantScalar {

    pub fn new(value: f64) -> NodeRc {
        NodeRc::ConstantScalarRc(Rc::new(
            Self {
                value: value,
            }
        ))
    }
}

impl Node for ConstantScalar {

    fn get_partial(&self, arg: &NodeRc) -> NodeRc { ConstantScalar::new(0.) }
    fn get_value(&self) -> f64 { self.value }
}

impl fmt::Display for ConstantScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {

    use crate::model::node::Node;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

    #[test]
    fn get_partial() {

        let x = VariableScalar::new_continuous("x", 2.);
        let c = ConstantScalar::new(3.);


        let z1 = c.get_partial(&x);
        assert!(z1.is_constant_with_value(0.));

        let z2 = c.get_partial(&c);
        assert!(z2.is_constant_with_value(0.));
    }

}