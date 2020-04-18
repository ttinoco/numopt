
use crate::model::node::Node;

pub struct FunctionMul<T: Node> {
    arguments: [T; 2],
}

impl<T: Node> FunctionMul<T> {

    pub fn new(arg1: T, arg2: T) -> Self {

        Self {
            arguments: [arg1, arg2],
        }
    }
}



