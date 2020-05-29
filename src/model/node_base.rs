use std::f64::NAN;
use std::collections::HashMap;

use crate::model::node::Node;

pub trait NodeBase {

    fn arguments(&self) -> Vec<&Node> { Vec::new() }
    fn partial(&self, arg: &Node) -> Node;
    fn evaluate(&self, _var_values: &HashMap<&Node, f64>) -> f64 { NAN }
}

impl NodeBase for Node {
    
    fn arguments(&self) -> Vec<&Node> {
        match self {
            Node::ConstantScalar(x) => x.arguments(),
            Node::VariableScalar(x) => x.arguments(),
            Node::FunctionAdd(x) => x.arguments(),
            Node::FunctionCos(x) => x.arguments(),
            Node::FunctionDiv(x) => x.arguments(),
            Node::FunctionMul(x) => x.arguments(),
            Node::FunctionSin(x) => x.arguments(),
        }
    }
    
    fn partial(&self, arg: &Node) -> Node { 
        match self {
            Node::ConstantScalar(x) => x.partial(arg),
            Node::VariableScalar(x) => x.partial(arg),
            Node::FunctionAdd(x) => x.partial(arg),
            Node::FunctionCos(x) => x.partial(arg),
            Node::FunctionDiv(x) => x.partial(arg),
            Node::FunctionMul(x) => x.partial(arg),
            Node::FunctionSin(x) => x.partial(arg),
        }
    }

    fn evaluate(&self, var_values: &HashMap<&Node, f64>) -> f64 {
        match self {
            Node::ConstantScalar(x) => x.value(),
            Node::VariableScalar(_) => {
                match var_values.get(self) {
                    Some(x) => *x,
                    None => NAN,
                }
            },
            Node::FunctionAdd(x) => x.evaluate(var_values),
            Node::FunctionCos(x) => x.evaluate(var_values),
            Node::FunctionDiv(x) => x.evaluate(var_values),
            Node::FunctionMul(x) => x.evaluate(var_values),
            Node::FunctionSin(x) => x.evaluate(var_values),            
        }
    }
}

