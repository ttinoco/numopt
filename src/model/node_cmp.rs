use num_traits::cast::ToPrimitive;

use crate::model::node::NodeRef;
use crate::model::constant::ConstantScalar;
use crate::model::constraint::{Constraint,
                               ConstraintKind};

const DEFAULT_LABEL: &str = "";

pub trait NodeCmp<T> {

    fn equal_and_tag(&self, other: T, tag: &str) -> Constraint;
    fn equal(&self, other: T) -> Constraint { self.equal_and_tag(other, DEFAULT_LABEL) }
    fn geq_and_tag(&self, other: T, tag: &str) -> Constraint;
    fn geq(&self, other: T) -> Constraint { self.geq_and_tag(other, DEFAULT_LABEL) }
    fn leq_and_tag(&self, other: T, tag: &str) -> Constraint;
    fn leq(&self, other: T) -> Constraint { self.leq_and_tag(other, DEFAULT_LABEL) }
}

macro_rules! impl_node_cmp_scalar {
    ($x: ty, $y: ty) => {
        impl NodeCmp<$y> for $x {
    
            fn equal_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(self.clone(),
                                ConstraintKind::Equal,
                                ConstantScalar::new(other.to_f64().unwrap()),
                                tag,
                                0.)
            }

            fn geq_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(self.clone(),
                                ConstraintKind::GreaterEqual,
                                ConstantScalar::new(other.to_f64().unwrap()),
                                tag,
                                0.)
            }

            fn leq_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(self.clone(),
                                ConstraintKind::LessEqual,
                                ConstantScalar::new(other.to_f64().unwrap()),
                                tag,
                                0.)
            }
        }
    };
}

impl_node_cmp_scalar!(NodeRef, f64);

macro_rules! impl_node_cmp_node {
    ($x: ty, $y: ty) => {
        impl NodeCmp<$y> for $x {
    
            fn equal_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(self.clone(),
                                ConstraintKind::Equal,
                                other.clone(),
                                tag,
                                0.)
            }

            fn geq_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(self.clone(),
                                ConstraintKind::GreaterEqual,
                                other.clone(),
                                tag,
                                0.)
            }

            fn leq_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(self.clone(),
                                ConstraintKind::LessEqual,
                                other.clone(),
                                tag,
                                0.)
            }
        }
    };
}

impl_node_cmp_node!(NodeRef, NodeRef);
impl_node_cmp_node!(NodeRef, &NodeRef);

macro_rules! impl_scalar_cmp_node {
    ($x: ty, $y: ty) => {
        impl NodeCmp<$y> for $x {
    
            fn equal_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(ConstantScalar::new(self.to_f64().unwrap()),
                                ConstraintKind::Equal,
                                other.clone(),
                                tag,
                                0.)
            }

            fn geq_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(ConstantScalar::new(self.to_f64().unwrap()),
                                ConstraintKind::GreaterEqual,
                                other.clone(),
                                tag,
                                0.)
            }

            fn leq_and_tag(&self, other: $y, tag: &str) -> Constraint {
                Constraint::new(ConstantScalar::new(self.to_f64().unwrap()),
                                ConstraintKind::LessEqual,
                                other.clone(),
                                tag,
                                0.)
            }
        }
    };
}

impl_scalar_cmp_node!(f64, NodeRef);
impl_scalar_cmp_node!(f64, &NodeRef);

#[cfg(test)]
mod tests {

    use crate::model::node_cmp::NodeCmp;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

    #[test]
    fn node_cmp_node() {

        let x = VariableScalar::new_continuous("x", 1.);
        let c = ConstantScalar::new(5.);

        let z1 = x.equal(&c);
        assert_eq!(format!("{}", z1), "x == 5");

        let z2 = &x.leq(&c);
        assert_eq!(format!("{}", z2), "x <= 5");

        let z3 = x.geq(&x + 3.);
        assert_eq!(format!("{}", z3), "x >= x + 3");

        let z4 = &x.leq(5.*&x);
        assert_eq!(format!("{}", z4), "x <= 5*x");
    }

    #[test]
    fn node_cmp_scalar() {

        let x = VariableScalar::new_continuous("x", 5.);

        let z1 = x.equal(6.);
        assert_eq!(format!("{}", z1), "x == 6");

        let z2 = &x.equal(10.);
        assert_eq!(format!("{}", z2), "x == 10");

        let z3 = (&x + 11.).equal(12.);
        assert_eq!(format!("{}", z3), "x + 11 == 12");
    }

    #[test]
    fn scalar_cmp_node() {

        let x = VariableScalar::new_continuous("x", 3.);

        let z1 = 4_f64.equal(&x);
        assert_eq!(format!("{}", z1), "4 == x");

        let z2 = 4_f64.leq(&x + 3.);
        assert_eq!(format!("{}", z2), "4 <= x + 3");

        let z3 = 5_f64.geq(&x*5.);
        assert_eq!(format!("{}", z3), "5 >= x*5");
    }
}