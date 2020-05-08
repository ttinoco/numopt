use std::fmt;

use super::node::{Node, NodeRc};

pub enum ConstraintKind {
    Equal,
    LessEqual,
    GreaterEqual,
}

pub struct Constraint {
    lhs: NodeRc,
    kind: ConstraintKind,
    rhs: NodeRc,
    dual: f64,
    label: String,
}

impl Constraint {

    pub fn dual(&self) -> f64 { self.dual }
    pub fn label(&self) -> &str { self.label.as_ref() }

    pub fn new(lhs: NodeRc, kind: ConstraintKind, rhs: NodeRc, label: &str, dual: f64) -> Constraint {
        Constraint {
            lhs: lhs,
            kind: kind,
            rhs: rhs,
            label: String::from(label),
            dual: dual,
        }
    }

    pub fn violation(&self) -> f64 {
        match self.kind {
            ConstraintKind::Equal => (self.lhs.value()-self.rhs.value()).abs(),
            ConstraintKind::LessEqual => 0_f64.max(self.lhs.value()-self.rhs.value()),
            ConstraintKind::GreaterEqual => 0_f64.max(self.rhs.value()-self.lhs.value()),
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

    use super::*;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

    #[test]
    fn label() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c = ConstantScalar::new(4.);

        let z = Constraint::new(x, ConstraintKind::Equal, c, "foo", 3.);
        assert_eq!(z.label(), "foo");
    }

    #[test]
    fn dual() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c = ConstantScalar::new(4.);

        let z = Constraint::new(x, ConstraintKind::Equal, c, "foo", 3.);
        assert_eq!(z.dual(), 3.);
    }

    #[test]
    fn violation() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c4 = ConstantScalar::new(4.);

        let z1 = Constraint::new(x.clone(), ConstraintKind::Equal, c4.clone(), "foo", 0.);
        assert_eq!(z1.violation(), 1.);

        let z2 = Constraint::new(x.clone(), ConstraintKind::LessEqual, c4.clone(), "foo", 0.);
        assert_eq!(z2.violation(), 0.);

        let z3 = Constraint::new(x.clone(), ConstraintKind::LessEqual, -c4.clone(), "foo", 0.);
        assert_eq!(z3.violation(), 7.);

        let z4 = Constraint::new(x.clone(), ConstraintKind::GreaterEqual, c4.clone(), "foo", 0.);
        assert_eq!(z4.violation(), 1.);

        let z5 = Constraint::new(x.clone(), ConstraintKind::GreaterEqual, -c4.clone(), "foo", 0.);
        assert_eq!(z5.violation(), 0.);
    }
}


