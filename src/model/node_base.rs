use simple_error::SimpleError;

use crate::model::node::Node;

pub trait NodeBase {

    fn arguments(&self) -> Vec<Node> { Vec::new() }
    fn partial(&self, arg: &Node) -> Node;
    fn update_value(&mut self, _value: f64) -> Result<(), SimpleError> { 
        panic!("can only update value of variables")
    }
    fn value(&self) -> f64;
}

impl NodeBase for Node {
    
    fn arguments(&self) -> Vec<Node> {
        match self {
            Node::ConstantScalar(x) => (**x).borrow().arguments(),
            Node::VariableScalar(x) => (**x).borrow().arguments(),
            Node::FunctionAdd(x) => (**x).borrow().arguments(),
            Node::FunctionCos(x) => (**x).borrow().arguments(),
            Node::FunctionDiv(x) => (**x).borrow().arguments(),
            Node::FunctionMul(x) => (**x).borrow().arguments(),
            Node::FunctionSin(x) => (**x).borrow().arguments(),
        }
    }
    
    fn partial(&self, arg: &Node) -> Node { 
        match self {
            Node::ConstantScalar(x) => (**x).borrow().partial(arg),
            Node::VariableScalar(x) => (**x).borrow().partial(arg),
            Node::FunctionAdd(x) => (**x).borrow().partial(arg),
            Node::FunctionCos(x) => (**x).borrow().partial(arg),
            Node::FunctionDiv(x) => (**x).borrow().partial(arg),
            Node::FunctionMul(x) => (**x).borrow().partial(arg),
            Node::FunctionSin(x) => (**x).borrow().partial(arg),
        }
    }

    fn update_value(&mut self, value: f64) -> Result<(), SimpleError> {
        match self {
            Node::VariableScalar(x) => (**x).borrow_mut().update_value(value),
            _ => panic!("can only update value of variables")
        }
    }

    fn value(&self) -> f64 {
        match self {
            Node::ConstantScalar(x) => (**x).borrow().value(),
            Node::VariableScalar(x) => (**x).borrow().value(),
            Node::FunctionAdd(x) => (**x).borrow().value(),
            Node::FunctionCos(x) => (**x).borrow().value(),
            Node::FunctionDiv(x) => (**x).borrow().value(),
            Node::FunctionMul(x) => (**x).borrow().value(),
            Node::FunctionSin(x) => (**x).borrow().value(),            
        }
    }
}

