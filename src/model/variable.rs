use std::fmt;
use std::rc::Rc;

use super::node::{Node,
                  NodeRc};
use super::constant::ConstantScalar;

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

    fn partial(&self, arg: &NodeRc) -> NodeRc { 
        match arg {
            NodeRc::VariableScalarRc(x) => {
                if self as *const VariableScalar == x.as_ref() {
                    ConstantScalar::new(1.)       
                }
                else {
                    ConstantScalar::new(0.)       
                }
            }
            _ => ConstantScalar::new(0.)  
        }
    }
    fn value(&self) -> f64 { self.value }
}

impl<'a> fmt::Display for VariableScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {

    use crate::model::node::Node;
    use crate::model::variable::VariableScalar;

    #[test]
    fn partial() {

        let x = VariableScalar::new_continuous("x", 2.);
        let y = VariableScalar::new_continuous("y", 3.);

        let z1 = x.partial(&x);
        assert!(z1.is_constant_with_value(1.));

        let z2 = x.partial(&y);
        assert!(z2.is_constant_with_value(0.));
    }
}