
use crate::model::node::{Node,
                         NodeType};

pub struct FunctionAdd {
    arguments: Vec<NodeType>,
}

impl FunctionAdd {

    pub fn new(args: Vec<NodeType>) -> NodeType {

        assert!(args.len() >= 2);
        NodeType::FunctionAddType(
            Self {
                arguments: args,
            }
        )
    }
}

impl Node for FunctionAdd {

    fn get_value(&self) -> f64 { 
        self.arguments.iter().map(|x| x.get_value()).sum()
    }
}



