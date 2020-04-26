
use std::fmt;
use std::rc::Rc;

use crate::model::node::{Node,
                         NodeRc};

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

    fn get_value(&self) -> f64 { 
        self.args.iter().map(|x| x.get_value()).sum()
    }

    fn get_arguments(&self) -> Vec<NodeRc> {
        self.args.iter().map(|x| x.clone()).collect()
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



