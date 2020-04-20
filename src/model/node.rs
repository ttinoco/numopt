use std::fmt;
use std::rc::Rc;
use std::ops::{Add, Mul};
use num_traits::cast::ToPrimitive;

use crate::model::constant::ConstantScalar;
use crate::model::variable::VariableScalar;
use crate::model::function::add::FunctionAdd;
use crate::model::function::mul::FunctionMul;

pub enum NodeRc {
    ConstantScalarRc(Rc<ConstantScalar>),
    VariableScalarRc(Rc<VariableScalar>),
    FunctionAddRc(Rc<FunctionAdd>),
    FunctionMulRc(Rc<FunctionMul>),
}

pub trait Node {

    fn get_value(&self) -> f64;
    
}

impl Node for NodeRc {
    
    fn get_value(&self) -> f64 {
        match self {
            NodeRc::ConstantScalarRc(x) => x.get_value(),
            NodeRc::VariableScalarRc(x) => x.get_value(),
            NodeRc::FunctionAddRc(x) => x.get_value(),
            NodeRc::FunctionMulRc(x) => x.get_value(),
        }
    }
}

impl Clone for NodeRc {
    fn clone(&self) -> Self {
        match self {
            NodeRc::ConstantScalarRc(x) => NodeRc::ConstantScalarRc(Rc::clone(&x)),
            NodeRc::VariableScalarRc(x) => NodeRc::VariableScalarRc(Rc::clone(&x)),
            NodeRc::FunctionAddRc(x) => NodeRc::FunctionAddRc(Rc::clone(&x)),
            NodeRc::FunctionMulRc(x) => NodeRc::FunctionMulRc(Rc::clone(&x)),
        }
    }
}

impl fmt::Display for NodeRc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRc::ConstantScalarRc(x) => write!(f, "{}", x),
            NodeRc::VariableScalarRc(x) => write!(f, "{}", x),
            NodeRc::FunctionAddRc(x) => write!(f, "{}", x),
            NodeRc::FunctionMulRc(x) => write!(f, "{}", x),
        }
    }
}

macro_rules! impl_node_add_node {
    ($x: ty, $y: ty) => {
        impl Add<$y> for $x {
            type Output = NodeRc;
            fn add(self, rhs: $y) -> NodeRc {
                FunctionAdd::new(vec![self.clone(), rhs.clone()])
            }        
        }
    };
}

macro_rules! impl_node_add_scalar {
    ($x: ty, $y: ty) => {
        impl Add<$y> for $x {
            type Output = NodeRc;
            fn add(self, rhs: $y) -> NodeRc {
                FunctionAdd::new(
                    vec![self.clone(), 
                         ConstantScalar::new(rhs.to_f64().unwrap())])
            }           
        }
        impl Add<$x> for $y {
            type Output = NodeRc;
            fn add(self, rhs: $x) -> NodeRc {
                FunctionAdd::new(
                    vec![ConstantScalar::new(self.to_f64().unwrap()), 
                    rhs.clone()])
            }           
        }
    };
}

impl_node_add_node!(&NodeRc, &NodeRc);
impl_node_add_node!(&NodeRc, NodeRc);
impl_node_add_node!(NodeRc, &NodeRc);
impl_node_add_node!(NodeRc, NodeRc);
impl_node_add_scalar!(&NodeRc, f64);
impl_node_add_scalar!(NodeRc, f64);

macro_rules! impl_node_mul_node {
    ($x: ty, $y: ty) => {
        impl Mul<$y> for $x {
            type Output = NodeRc;
            fn mul(self, rhs: $y) -> NodeRc {
                FunctionMul::new(self.clone(), rhs.clone())
            }        
        }
    };
}

impl_node_mul_node!(&NodeRc, &NodeRc);
impl_node_mul_node!(&NodeRc, NodeRc);
impl_node_mul_node!(NodeRc, &NodeRc);
impl_node_mul_node!(NodeRc, NodeRc);

#[cfg(test)]
mod tests {

    use num_traits::pow::Pow;

    use crate::model::node::Node;
    use crate::model::variable::VariableScalar;

    #[test]
    fn node_add_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = &x + &y;
        assert_eq!(format!("{}", z1), "x + y");
        assert_eq!(z1.get_value(), 7.);

        let z2 = &y + &x;
        assert_eq!(format!("{}", z2), "y + x");
        assert_eq!(z2.get_value(), 7.);

        let z3 = &x + (&y + &x);
        assert_eq!(format!("{}", z3), "x + y + x");
        assert_eq!(z3.get_value(), 10.);

        let z4 = (&x + &y) + &x;
        assert_eq!(format!("{}", z4), "x + y + x");
        assert_eq!(z4.get_value(), 10.);

        let z5 = &z1 + &z2 + &z3 + &z4;
        assert_eq!(format!("{}", z5), "x + y + y + x + x + y + x + x + y + x");
        assert_eq!(z5.get_value(), 34.);
    
        let z6 = (&x + &y) + (&y + &x);
        assert_eq!(format!("{}", z6), "x + y + y + x");
        assert_eq!(z6.get_value(), 14.);
    }

    #[test]
    fn node_add_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);

        let z1 = &x + 15.;
        assert_eq!(format!("{}", z1), "x + 15");
        assert_eq!(z1.get_value(), 18.);

        let z2 = 13. + &x;
        assert_eq!(format!("{}", z2), "13 + x");
        assert_eq!(z2.get_value(), 16.);

        let z3 = 2. + &z2 + 6.;
        assert_eq!(format!("{}", z3), "2 + 13 + x + 6");
        assert_eq!(z3.get_value(), 24.);
    }

    #[test]
    fn node_mul_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = &x*&y;

        assert_eq!(format!("{}", z1), "x*y");
        assert_eq!(z1.get_value(), 12.);

        let z2 = &y*&x;

        assert_eq!(format!("{}", z2), "y*x");
        assert_eq!(z2.get_value(), 12.);

        let z3 = (&y*&x)*&x;

        assert_eq!(format!("{}", z3), "y*x*x");
        assert_eq!(z3.get_value(), 36.);

        let z4 = &y*(&x*&x);

        assert_eq!(format!("{}", z4), "y*x*x");
        assert_eq!(z4.get_value(), 36.);

        let z5 = &z4*(&x*&z3);

        assert_eq!(format!("{}", z5), "y*x*x*x*y*x*x");
        assert_eq!(z5.get_value(), (4.).pow(2.)*((3.).pow(5.)));
    }

    #[test]
    fn node_mul_add_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = (&x + &y*&x)*(&y*&x + &y);
        assert_eq!(format!("{}", z1), "(x + y*x)*(y*x + y)");
        assert_eq!(z1.get_value(), 15.*16.);
    }
}