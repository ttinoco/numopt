use std::fmt;
use std::rc::Rc;

use crate::model::node::{Node,
                         NodeRc};
use crate::model::constant::ConstantScalar;

pub struct FunctionCos {
    value: f64,
    arg: NodeRc
}

impl FunctionCos {

    pub fn new(arg: NodeRc) -> NodeRc {
        NodeRc::FunctionCosRc(Rc::new(
            Self {
                value: 0.,
                arg: arg,
            }
        ))
    }
}

impl Node for FunctionCos {

    fn arguments(&self) -> Vec<NodeRc> {
        vec![self.arg.clone()]
    }

    fn partial(&self, arg: &NodeRc) -> NodeRc { 
        if self.arg == *arg {
            return -&self.arg.sin();
        }
        else {
            return ConstantScalar::new(0.);
        }
    }

    fn value(&self) -> f64 { 
        self.arg.value().cos()
    }
}

impl<'a> fmt::Display for FunctionCos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cos({})", self.arg)
    }
}

#[cfg(test)]
mod tests {

    use crate::model::node::Node;
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

        let z1 = x.cos();
        let z1x = z1.derivative(&x);
        let z1y = z1.derivative(&y);
        assert_eq!(format!("{}", z1x), "-1*sin(x)");
        assert!(z1y.is_constant_with_value(0.));

        let z2 = (5.*&x + 3.*&y).cos();
        let z2x = z2.derivative(&x);
        let z2y = z2.derivative(&y);
        assert_eq!(format!("{}", z2x), "-1*sin(5*x + 3*y)*5");
        assert_eq!(format!("{}", z2y), "-1*sin(5*x + 3*y)*3");
    }
}