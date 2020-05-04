
use std::fmt;
use std::rc::Rc;

use crate::model::node::{Node,
                         NodeRc};
use crate::model::constant::ConstantScalar;

pub struct FunctionAdd {
    args: Vec<NodeRc>,
}

impl FunctionAdd {

    pub fn new(args: Vec<NodeRc>) -> NodeRc {

        assert!(args.len() >= 2);
        NodeRc::FunctionAddRc(Rc::new(
            Self {
                args: args,
            }
        ))
    }
}

impl Node for FunctionAdd {

    fn arguments(&self) -> Vec<NodeRc> {
        self.args.iter().map(|x| x.clone()).collect()
    }

    fn partial(&self, arg: &NodeRc) -> NodeRc { 
        for a in &self.args {
            if *a == *arg {
                return ConstantScalar::new(1.);
            } 
        }
        ConstantScalar::new(0.)
    }

    fn value(&self) -> f64 { 
        self.args.iter().map(|x| x.value()).sum()
    }
}

impl<'a> fmt::Display for FunctionAdd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.args.len();
        for i in 0..n {
            if i < n-1 {
                write!(f, "{} + ", self.args[i]).unwrap();
            }
            else {
                write!(f, "{}", self.args[i]).unwrap();
            }
        };
        Ok(())
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

        let z = &x + &y; 

        let z1 = z.partial(&x);
        assert!(z1.is_constant_with_value(1.));

        let z2 = z.partial(&y);
        assert!(z2.is_constant_with_value(1.));

        let z3 = z.partial(&w);
        assert!(z3.is_constant_with_value(0.));

        let zz = &x + 2.;
        let f = &y + &zz;

        let z4 = f.partial(&x);
        assert!(z4.is_constant_with_value(1.));

        let z5 = f.partial(&y);
        assert!(z5.is_constant_with_value(1.));

        let z6 = f.partial(&zz);
        assert!(z6.is_constant_with_value(0.));
    }
}

