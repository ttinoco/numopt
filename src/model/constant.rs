
use super::node::{Node,
                  NodeType};

pub struct ConstantScalar {
    value: f64,
}

impl ConstantScalar {

    pub fn new(value: f64) -> NodeType {
        NodeType::ConstantScalarType(
            Self {
                value: value,
            }
        )
    }
}

impl Node for ConstantScalar {

    fn get_value(&self) -> f64 { self.value }
}

#[cfg(test)]
mod tests {


}