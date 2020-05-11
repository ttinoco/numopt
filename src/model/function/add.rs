
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::model::node::{NodeBase, NodeRef};
use crate::model::node_std::{NodeStd, NodeStdProp};
use crate::model::constant::ConstantScalar;

pub struct FunctionAdd {
    args: Vec<NodeRef>,
}

impl FunctionAdd {

    pub fn new(args: Vec<NodeRef>) -> NodeRef {

        assert!(args.len() >= 2);
        NodeRef::FunctionAdd(Rc::new(RefCell::new(
            Self {
                args: args,
            }
        )))
    }
}

impl NodeBase for FunctionAdd {

    fn arguments(&self) -> Vec<NodeRef> {
        self.args.iter().map(|x| x.clone()).collect()
    }

    fn partial(&self, arg: &NodeRef) -> NodeRef { 
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

impl NodeStd for FunctionAdd {

    fn properties(&self) -> NodeStdProp {
        
        let mut affine = true;
        let mut a: HashMap<NodeRef, f64> = HashMap::new();
        let mut b = 0_f64;
        for arg in self.arguments().iter() {
            let p = arg.properties();
            affine &= p.affine;
            b += p.b;
            for (key, val) in p.a.iter() {
                if let Some(x) = a.get_mut(key) {
                    *x += *val;
                }
                else {
                    a.insert(key.clone(), *val);
                }
            }
        }
        NodeStdProp {
            affine: affine,
            a: a,
            b: b,
        }
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

    use crate::model::node::NodeBase;
    use crate::model::node_diff::NodeDiff;
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

    #[test]
    fn derivative() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = &x + 1.;
        let z1x = z1.derivative(&x);
        let z1y = z1.derivative(&y);
        assert!(z1x.is_constant_with_value(1.));
        assert!(z1y.is_constant_with_value(0.));

        let z2 = &x + &y;
        let z2x = z2.derivative(&x);
        let z2y = z2.derivative(&y);
        assert!(z2x.is_constant_with_value(1.));
        assert!(z2y.is_constant_with_value(1.));

        let z3 = (&x + 1.) + (&x + 3.) + (&y + (&x + 5.));
        let z3x = z3.derivative(&x);
        let z3y = z3.derivative(&y);
        assert!(z3x.is_constant_with_value(3.));
        assert!(z3y.is_constant_with_value(1.));

        let z4 = &x + &x;
        let z4x = z4.derivative(&x);
        let z4y = z4.derivative(&y);
        assert!(z4x.is_constant_with_value(2.));
        assert!(z4y.is_constant_with_value(0.));

        let f1 = &x + 1. + &y;
        let z5 = &f1 + &f1;
        let z5x = z5.derivative(&x);
        let z5y = z5.derivative(&y);
        assert_eq!(z5.value(), 2.*(3.+1.+4.));
        assert!(z5x.is_constant_with_value(2.));
        assert!(z5y.is_constant_with_value(2.));
    }
}

