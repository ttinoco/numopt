use std::fmt;
use std::collections::HashMap;

use super::node::Node;
use super::node_base::NodeBase;

pub enum ConstraintKind {
    Equal,
    LessEqual,
    GreaterEqual,
}

pub struct Constraint {
    lhs: Node,
    kind: ConstraintKind,
    rhs: Node,
    label: String,
}

impl Constraint {

    pub fn label(&self) -> &str { self.label.as_ref() }

    pub fn new(lhs: Node, kind: ConstraintKind, rhs: Node, label: &str) -> Constraint {
        Constraint {
            lhs: lhs,
            kind: kind,
            rhs: rhs,
            label: String::from(label),
        }
    }

    pub fn violation(&self, var_values: &HashMap<&Node, f64>) -> f64 {
        match self.kind {
            ConstraintKind::Equal => { 
                (self.lhs.eval(var_values)-self.rhs.eval(var_values)).abs()
            },
            ConstraintKind::LessEqual => { 
                0_f64.max(self.lhs.eval(var_values)-self.rhs.eval(var_values))
            },
            ConstraintKind::GreaterEqual => {
                0_f64.max(self.rhs.eval(var_values)-self.lhs.eval(var_values))
            },
        }
    }
}

impl<'a> fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ConstraintKind::Equal => write!(f, "{} == {}", self.lhs, self.rhs),
            ConstraintKind::LessEqual => write!(f, "{} <= {}", self.lhs, self.rhs),
            ConstraintKind::GreaterEqual => write!(f, "{} >= {}", self.lhs, self.rhs),
        }
    }
}

#[cfg(test)]
mod tests {

    use maplit::hashmap;

    use super::*;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

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


