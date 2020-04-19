use std::fmt;
use std::rc::Rc;

use super::node::{Node,
                  NodeRc};

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

    pub fn new(name: &str, value: f64, kind: VariableKind) -> NodeRc {
        NodeRc::VariableScalarRc(Rc::new(
            Self {
                name: name.to_string(),
                value: value,
                kind: kind,
            }
        ))
    }

    pub fn new_continuous(name: &str, value: f64) -> NodeRc {
        VariableScalar::new(name, value, VariableKind::VarContinuous)
    }

    pub fn new_integer(name: &str, value: f64) -> NodeRc {
        VariableScalar::new(name, value, VariableKind::VarInteger)
    }
}

impl<'a> Node for VariableScalar {

    fn get_value(&self) -> f64 { self.value }
}

impl<'a> fmt::Display for VariableScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {


}