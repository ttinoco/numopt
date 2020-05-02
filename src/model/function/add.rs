
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

    fn get_arguments(&self) -> Vec<NodeRc> {
        self.args.iter().map(|x| x.clone()).collect()
    }

    fn get_partial(&self, arg: &NodeRc) -> NodeRc { 
        for a in &self.args {
            if *a == *arg {
                return ConstantScalar::new(1.);
            } 
        }
        ConstantScalar::new(0.)
    }

    fn get_value(&self) -> f64 { 
        self.args.iter().map(|x| x.get_value()).sum()
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



