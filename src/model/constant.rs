
use std::fmt;
use std::rc::Rc;

use super::node::{Node,
                  NodeType};

pub struct ConstantScalar {
    value: f64,
}

impl ConstantScalar {

    pub fn new(value: f64) -> NodeType {
        NodeType::ConstantScalarType(Rc::new(
            Self {
                value: value,
            }
        ))
    }
}

impl Node for ConstantScalar {

    fn get_value(&self) -> f64 { self.value }
}

impl fmt::Display for ConstantScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {


}