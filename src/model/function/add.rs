
use std::fmt;
use std::rc::Rc;

use crate::model::node::{Node,
                         NodeType};

pub struct FunctionAdd {
    arguments: Vec<NodeType>,
}

impl FunctionAdd {

    pub fn new(args: Vec<NodeType>) -> NodeType {

        assert!(args.len() >= 2);
        NodeType::FunctionAddType(Rc::new(
            Self {
                arguments: args,
            }
        ))
    }
}

impl Node for FunctionAdd {

    fn get_value(&self) -> f64 { 
        self.arguments.iter().map(|x| x.get_value()).sum()
    }
}

impl<'a> fmt::Display for FunctionAdd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.arguments.len();
        for i in 0..n {
            if i < n-1 {
                write!(f, "{} + ", self.arguments[i]).unwrap();
            }
            else {
                write!(f, "{}", self.arguments[i]).unwrap();
            }
        };
        Ok(())
    }
}



