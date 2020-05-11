use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use simple_error::SimpleError;

use super::node::{NodeBase, NodeRef};
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

    pub fn new(name: &str, value: f64, kind: VariableKind) -> NodeRef {
        NodeRef::VariableScalar(Rc::new(RefCell::new(
            Self {
                name: name.to_string(),
                value: value,
                kind: kind,
            }
        )))
    }

    pub fn new_continuous(name: &str, value: f64) -> NodeRef {
        VariableScalar::new(name, value, VariableKind::VarContinuous)
    }

    pub fn new_integer(name: &str, value: f64) -> NodeRef {
        VariableScalar::new(name, value, VariableKind::VarInteger)
    }
}

impl NodeBase for VariableScalar {

    fn partial(&self, arg: &NodeRef) -> NodeRef { 
        match arg {
            NodeRef::VariableScalar(x) => {
                if self as *const VariableScalar == x.as_ref().as_ptr() {
                    ConstantScalar::new(1.)       
                }
                else {
                    ConstantScalar::new(0.)       
                }
            }
            _ => ConstantScalar::new(0.)  
        }
    }

    fn update_value(&mut self, value: f64) -> Result<(), SimpleError> {
        self.value = value;
        Ok(())
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

    use crate::model::node::NodeBase;
    use crate::model::node_std::NodeStd;
    use crate::model::node_diff::NodeDiff;
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

    #[test]
    fn derivative() {

        let x = VariableScalar::new_continuous("x", 2.);
        let y = VariableScalar::new_continuous("y", 3.);

        let z1 = x.derivative(&y);
        assert!(z1.is_constant_with_value(0.));

        let z2 = x.derivative(&x);
        println!("{}", z2);
        assert!(z2.is_constant_with_value(1.));
    }

    #[test]
    fn properties() {

        let x = VariableScalar::new_integer("x", 4.);
        let p = x.properties();
        assert!(p.affine);
        assert_eq!(p.b, 0.);
        assert_eq!(p.a.len(), 1);
        assert_eq!(*p.a.get(&x).unwrap(), 4.);
    }
}