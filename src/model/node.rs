use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use num_traits::cast::ToPrimitive;
use std::ops::{Add, Mul, Neg, Sub, Div};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::model::constant::ConstantScalar;
use crate::model::variable::VariableScalar;
use crate::model::function::add::FunctionAdd;
use crate::model::function::mul::FunctionMul;
use crate::model::function::div::FunctionDiv;

pub enum NodeRc {
    ConstantScalarRc(Rc<ConstantScalar>),
    VariableScalarRc(Rc<VariableScalar>),
    FunctionAddRc(Rc<FunctionAdd>),
    FunctionMulRc(Rc<FunctionMul>),
    FunctionDivRc(Rc<FunctionDiv>),
}

pub trait Node {

    fn get_value(&self) -> f64;
    fn get_arguments(&self) -> Vec<NodeRc> { Vec::new() }
}

impl NodeRc {

    fn all_simple_paths(&self, vars: &[&NodeRc]) -> HashMap<NodeRc, Vec<Vec<NodeRc>>> {

        // Check inputs
        for v in vars {
            match v {
                NodeRc::VariableScalarRc(_x) => (),
                _ => panic!("variable expected")
            }
        }

        // Vars
        let varset: HashSet<&NodeRc> = HashSet::from_iter(vars.iter().map(|x| x.clone()));

        // Workqueue
        let mut wq: VecDeque<Vec<NodeRc>> = VecDeque::new();
        wq.push_front(vec![self.clone()]);

        // Paths
        let mut paths: HashMap<NodeRc, Vec<Vec<NodeRc>>> = HashMap::new();

        // Process
        loop {

            // Pop path
            let path = match wq.pop_front() {
                Some(p) => p,
                None => break
            };
            let node = path.last().unwrap();

            // Add paths
            match node {
                NodeRc::VariableScalarRc(_x) => {
                    for v in &varset {
                        if node == *v {
                            let new_path = path.iter().map(|x| x.clone()).collect();
                            match paths.get_mut(node) {
                                Some(p) => { p.push(new_path); },
                                None => { paths.insert(node.clone(), vec![new_path]); },
                            };
                        }
                    } 
                }
                _ => (),
            };

            // Process arguments
            for n in node.get_arguments() {
                let mut new_path: Vec<NodeRc> = path.iter().map(|x| x.clone()).collect();
                new_path.push(n.clone());
                wq.push_front(new_path);
            }
        }

        // Return paths
        paths
    }

}

impl Node for NodeRc {
    
    fn get_value(&self) -> f64 {
        match self {
            NodeRc::ConstantScalarRc(x) => x.get_value(),
            NodeRc::VariableScalarRc(x) => x.get_value(),
            NodeRc::FunctionAddRc(x) => x.get_value(),
            NodeRc::FunctionMulRc(x) => x.get_value(),
            NodeRc::FunctionDivRc(x) => x.get_value(),
        }
    }

    fn get_arguments(&self) -> Vec<NodeRc> {
        match self {
            NodeRc::ConstantScalarRc(x) => x.get_arguments(),
            NodeRc::VariableScalarRc(x) => x.get_arguments(),
            NodeRc::FunctionAddRc(x) => x.get_arguments(),
            NodeRc::FunctionMulRc(x) => x.get_arguments(),
            NodeRc::FunctionDivRc(x) => x.get_arguments(),
        }
    }   
}

impl Hash for NodeRc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NodeRc::ConstantScalarRc(x) => ptr::hash(&**x, state),
            NodeRc::VariableScalarRc(x) => ptr::hash(&**x, state),
            NodeRc::FunctionAddRc(x) => ptr::hash(&**x, state),
            NodeRc::FunctionMulRc(x) => ptr::hash(&**x, state),
            NodeRc::FunctionDivRc(x) => ptr::hash(&**x, state),
        };
    }
}

impl PartialEq for NodeRc {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeRc::ConstantScalarRc(x), NodeRc::ConstantScalarRc(y)) => Rc::ptr_eq(x, y),
            (NodeRc::VariableScalarRc(x), NodeRc::VariableScalarRc(y)) => Rc::ptr_eq(x, y),
            (NodeRc::FunctionAddRc(x), NodeRc::FunctionAddRc(y)) => Rc::ptr_eq(x, y),
            (NodeRc::FunctionMulRc(x), NodeRc::FunctionMulRc(y)) => Rc::ptr_eq(x, y),
            (NodeRc::FunctionDivRc(x), NodeRc::FunctionDivRc(y)) => Rc::ptr_eq(x, y),
            _ => false,
        }
    }
}

impl Eq for NodeRc {}

impl Clone for NodeRc {
    fn clone(&self) -> Self {
        match self {
            NodeRc::ConstantScalarRc(x) => NodeRc::ConstantScalarRc(Rc::clone(&x)),
            NodeRc::VariableScalarRc(x) => NodeRc::VariableScalarRc(Rc::clone(&x)),
            NodeRc::FunctionAddRc(x) => NodeRc::FunctionAddRc(Rc::clone(&x)),
            NodeRc::FunctionMulRc(x) => NodeRc::FunctionMulRc(Rc::clone(&x)),
            NodeRc::FunctionDivRc(x) => NodeRc::FunctionDivRc(Rc::clone(&x)),
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
            NodeRc::FunctionDivRc(x) => write!(f, "{}", x),
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

macro_rules! impl_node_mul_scalar {
    ($x: ty, $y: ty) => {
        impl Mul<$y> for $x {
            type Output = NodeRc;
            fn mul(self, rhs: $y) -> NodeRc {
                FunctionMul::new(self.clone(), 
                                 ConstantScalar::new(rhs.to_f64().unwrap()))
            }           
        }
        impl Mul<$x> for $y {
            type Output = NodeRc;
            fn mul(self, rhs: $x) -> NodeRc {
                FunctionMul::new(ConstantScalar::new(self.to_f64().unwrap()), 
                                 rhs.clone())
            }           
        }
    };
}

impl_node_mul_node!(&NodeRc, &NodeRc);
impl_node_mul_node!(&NodeRc, NodeRc);
impl_node_mul_node!(NodeRc, &NodeRc);
impl_node_mul_node!(NodeRc, NodeRc);
impl_node_mul_scalar!(&NodeRc, f64);
impl_node_mul_scalar!(NodeRc, f64);

macro_rules! impl_node_neg {
    ($x: ty) => {
        impl Neg for $x {
            type Output = NodeRc;
            fn neg(self) -> NodeRc {
                (-1.)*self
            }        
        }
    };
}

impl_node_neg!(&NodeRc);
impl_node_neg!(NodeRc);

macro_rules! impl_node_sub_node {
    ($x: ty, $y: ty) => {
        impl Sub<$y> for $x {
            type Output = NodeRc;
            fn sub(self, rhs: $y) -> NodeRc {
                self + -1.*rhs
            }        
        }
    };
}

macro_rules! impl_node_sub_scalar {
    ($x: ty, $y: ty) => {
        impl Sub<$y> for $x {
            type Output = NodeRc;
            fn sub(self, rhs: $y) -> NodeRc {
                self + -1.*ConstantScalar::new(rhs.to_f64().unwrap())
            }           
        }
        impl Sub<$x> for $y {
            type Output = NodeRc;
            fn sub(self, rhs: $x) -> NodeRc {
                ConstantScalar::new(self.to_f64().unwrap()) + -1.*rhs
            }           
        }
    };
}

impl_node_sub_node!(&NodeRc, &NodeRc);
impl_node_sub_node!(&NodeRc, NodeRc);
impl_node_sub_node!(NodeRc, &NodeRc);
impl_node_sub_node!(NodeRc, NodeRc);
impl_node_sub_scalar!(&NodeRc, f64);
impl_node_sub_scalar!(NodeRc, f64);

macro_rules! impl_node_div_node {
    ($x: ty, $y: ty) => {
        impl Div<$y> for $x {
            type Output = NodeRc;
            fn div(self, rhs: $y) -> NodeRc {
                FunctionDiv::new(self.clone(), rhs.clone())
            }        
        }
    };
}

macro_rules! impl_node_div_scalar {
    ($x: ty, $y: ty) => {
        impl Div<$y> for $x {
            type Output = NodeRc;
            fn div(self, rhs: $y) -> NodeRc {
                FunctionDiv::new(self.clone(), 
                                 ConstantScalar::new(rhs.to_f64().unwrap()))
            }           
        }
        impl Div<$x> for $y {
            type Output = NodeRc;
            fn div(self, rhs: $x) -> NodeRc {
                FunctionDiv::new(ConstantScalar::new(self.to_f64().unwrap()), 
                                 rhs.clone())
            }           
        }
    };
}

impl_node_div_node!(&NodeRc, &NodeRc);
impl_node_div_node!(&NodeRc, NodeRc);
impl_node_div_node!(NodeRc, &NodeRc);
impl_node_div_node!(NodeRc, NodeRc);
impl_node_div_scalar!(&NodeRc, f64);
impl_node_div_scalar!(NodeRc, f64);

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

    #[test]
    fn node_mul_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);

        let z1 = &x*15.;
        assert_eq!(format!("{}", z1), "x*15");
        assert_eq!(z1.get_value(), 45.);

        let z2 = 13.*&x;
        assert_eq!(format!("{}", z2), "13*x");
        assert_eq!(z2.get_value(), 39.);

        let z3 = 2.*&z2*6.;
        assert_eq!(format!("{}", z3), "2*13*x*6");
        assert_eq!(z3.get_value(), 2.*13.*3.*6.);
    }

    #[test]
    fn node_neg() {

        let x = VariableScalar::new_continuous("x", 3.);

        let z1 = -&x;
        assert_eq!(format!("{}", z1), "-1*x");
        assert_eq!(z1.get_value(), -3.);

        let z2 = -(&x + 3.);
        assert_eq!(format!("{}", z2), "-1*(x + 3)");
        assert_eq!(z2.get_value(), -6.);
    }

    #[test]
    fn node_sub_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = &x - &y;
        assert_eq!(z1.get_value(), -1.);
        assert_eq!(format!("{}", z1), "x + -1*y");

        let z2 = &y - &x;
        assert_eq!(z2.get_value(), 1.);
        assert_eq!(format!("{}", z2), "y + -1*x");

        let z3 = &x - (&x - &y);
        assert_eq!(z3.get_value(), 4.);
        assert_eq!(format!("{}", z3), "x + -1*(x + -1*y)");

        let z4 = (&x - &y) - &y;
        assert_eq!(z4.get_value(), -5.);

        let z5 = &z4 - &z3 - &x;
        assert_eq!(z5.get_value(), -12.);

        let z6 = (&z1 - &z2) - (&z3 - &z4);
        assert_eq!(z6.get_value(), -2.-9.);
    }

    #[test]
    fn node_sub_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);

        let z1 = &x - 15.;
        assert_eq!(format!("{}", z1), "x + -1*15");
        assert_eq!(z1.get_value(), -12.);

        let z2 = 13. - &x;
        assert_eq!(format!("{}", z2), "13 + -1*x");
        assert_eq!(z2.get_value(), 10.);

        let z3 = 2. - &z2 - 6.;
        assert_eq!(format!("{}", z3), "2 + -1*(13 + -1*x) + -1*6");
        assert_eq!(z3.get_value(), -14.);
    }

    #[test]
    fn node_div_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = &x/&y;
        assert_eq!(format!("{}", z1), "x/y");
        assert_eq!(z1.get_value(), 3./4.);

        let z2 = (3.*&x)/(4.*&y);
        assert_eq!(format!("{}", z2), "3*x/(4*y)");
        assert_eq!(z2.get_value(), 9./16.);

        let z3 = (3. + &x)/(&y + 4.);
        assert_eq!(format!("{}", z3), "(3 + x)/(y + 4)");
        assert_eq!(z3.get_value(), 6./8.);

        let z4 = &x/(3.+&y);
        assert_eq!(format!("{}", z4), "x/(3 + y)");
        assert_eq!(z4.get_value(), 3./7.);

        let z5 = (2.+&x)/&y;
        assert_eq!(format!("{}", z5), "(2 + x)/y");
        assert_eq!(z5.get_value(), 5./4.);
    }

    #[test]
    fn test_node_div_scalar() {

        let x = VariableScalar::new_continuous("x", 4.);

        let z1 = 3./&x;
        assert_eq!(format!("{}", z1), "3/x");
        assert_eq!(z1.get_value(), 3./4.);

        let z2 = 3./(&x + 1.);
        assert_eq!(format!("{}", z2), "3/(x + 1)");
        assert_eq!(z2.get_value(), 3./5.);

        let z3 = &x/3.;
        assert_eq!(format!("{}", z3), "x/3");
        assert_eq!(z3.get_value(), 4./3.);

        let z4 = (&x + 1.)/3.;
        assert_eq!(format!("{}", z4), "(x + 1)/3");
        assert_eq!(z4.get_value(), 5./3.);
    }
}