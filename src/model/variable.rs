use super::node::{Node,
                  NodeType};

pub enum VariableKind {
    VarContinuous,
    VarInteger,
}

pub struct VariableScalar {
    name: String,
    value: f64,
    kind: VariableKind,
}

impl VariableScalar {

    pub fn new(name: &str, value: f64, kind: VariableKind) -> NodeType {
        NodeType::VariableScalarType(
            Self {
                name: name.to_string(),
                value: value,
                kind: kind,
            }
        )
    }

    pub fn new_continuous(name: &str, value: f64) -> NodeType {
        VariableScalar::new(name, value, VariableKind::VarContinuous)
    }

    pub fn new_integer(name: &str, value: f64) -> NodeType {
        VariableScalar::new(name, value, VariableKind::VarInteger)
    }
}

impl Node for VariableScalar {

    fn get_value(&self) -> f64 { self.value }
}


#[cfg(test)]
mod tests {


}