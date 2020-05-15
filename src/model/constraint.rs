use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;

use super::node::Node;
use super::node_base::NodeBase;

pub enum ConstraintKind {
    Equal,
    LessEqual,
    GreaterEqual,
}

struct ConstraintData {
    lhs: Node,
    kind: ConstraintKind,
    rhs: Node,
    label: String,
}

pub struct Constraint(Rc<ConstraintData>);

impl Constraint {

    pub fn label(&self) -> &str { self.0.label.as_ref() }

    pub fn new(lhs: Node, kind: ConstraintKind, rhs: Node, label: &str) -> Constraint {
        Constraint(Rc::new(
            ConstraintData{
                lhs: lhs,
                kind: kind,
                rhs: rhs,
                label: String::from(label),
            }
        ))
    }

    pub fn violation(&self, var_values: &HashMap<&Node, f64>) -> f64 {
        match self.0.kind {
            ConstraintKind::Equal => { 
                (self.0.lhs.eval(var_values)-self.0.rhs.eval(var_values)).abs()
            },
            ConstraintKind::LessEqual => { 
                0_f64.max(self.0.lhs.eval(var_values)-self.0.rhs.eval(var_values))
            },
            ConstraintKind::GreaterEqual => {
                0_f64.max(self.0.rhs.eval(var_values)-self.0.lhs.eval(var_values))
            },
        }
    }
}

impl Clone for Constraint {
    fn clone(&self) -> Self {
        Constraint(Rc::clone(&self.0))
    }
}

impl PartialEq for Constraint {

    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Constraint {}

impl<'a> fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.kind {
            ConstraintKind::Equal => write!(f, "{} == {}", self.0.lhs, self.0.rhs),
            ConstraintKind::LessEqual => write!(f, "{} <= {}", self.0.lhs, self.0.rhs),
            ConstraintKind::GreaterEqual => write!(f, "{} >= {}", self.0.lhs, self.0.rhs),
        }
    }
}

impl fmt::Debug for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {

    use maplit::hashmap;

    use super::*;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

    #[test]
    fn clone_eq() {

        let x = VariableScalar::new_continuous("x");
        let c = ConstantScalar::new(4.);

        let c1 = Constraint::new(x.clone(), ConstraintKind::Equal, c.clone(), "foo");
        let c2 = Constraint::new(x.clone(), ConstraintKind::LessEqual, c.clone(), "foo");
        let c3 = c1.clone();

        assert_ne!(c1, c2);
        assert_eq!(c1, c3);
    }

    #[test]
    fn label() {

        let x = VariableScalar::new_continuous("x");
        let c = ConstantScalar::new(4.);

        let z = Constraint::new(x, ConstraintKind::Equal, c, "foo");
        assert_eq!(z.label(), "foo");
    }

    #[test]
    fn violation() {

        let x = VariableScalar::new_continuous("x");
        let c4 = ConstantScalar::new(4.);

        let var_values = hashmap!{ &x => 3. };

        let z1 = Constraint::new(x.clone(), ConstraintKind::Equal, c4.clone(), "foo");
        assert_eq!(z1.violation(&var_values), 1.);

        let z2 = Constraint::new(x.clone(), ConstraintKind::LessEqual, c4.clone(), "foo");
        assert_eq!(z2.violation(&var_values), 0.);

        let z3 = Constraint::new(x.clone(), ConstraintKind::LessEqual, -c4.clone(), "foo");
        assert_eq!(z3.violation(&var_values), 7.);

        let z4 = Constraint::new(x.clone(), ConstraintKind::GreaterEqual, c4.clone(), "foo");
        assert_eq!(z4.violation(&var_values), 1.);

        let z5 = Constraint::new(x.clone(), ConstraintKind::GreaterEqual, -c4.clone(), "foo");
        assert_eq!(z5.violation(&var_values), 0.);
    }
}


