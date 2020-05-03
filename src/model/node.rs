use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use num_traits::cast::ToPrimitive;
use num_traits::identities::{Zero, One};
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

    fn arguments(&self) -> Vec<NodeRc> { Vec::new() }
    fn partial(&self, arg: &NodeRc) -> NodeRc;
    fn value(&self) -> f64;

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

        // Vars set
        let varset: HashSet<&NodeRc> = HashSet::from_iter(vars.iter().map(|x| x.clone()));

        // Workqueue
        let mut wq: VecDeque<Vec<NodeRc>> = VecDeque::new();
        wq.push_front(vec![self.clone()]);

        // Paths
        let mut paths: HashMap<NodeRc, Vec<Vec<NodeRc>>> = HashMap::new();
        for v in &varset {
            paths.insert((*v).clone(), Vec::new());
        }

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
                            paths.get_mut(node).unwrap().push(new_path); 
                        }
                    } 
                }
                _ => (),
            };

            // Process arguments
            for n in node.arguments() {
                let mut new_path: Vec<NodeRc> = path.iter().map(|x| x.clone()).collect();
                new_path.push(n.clone());
                wq.push_front(new_path);
            }
        }

        // Return paths
        paths
    }

    pub fn derivative(&self, var: &NodeRc) -> NodeRc {
        let derivs = self.derivatives(&vec![var]);
        derivs.get(var).unwrap().clone()
    }

    pub fn derivatives(&self, vars: &[&NodeRc]) -> HashMap<NodeRc, NodeRc> {

        // Check inputs
        for v in vars {
            match v {
                NodeRc::VariableScalarRc(_x) => (),
                _ => panic!("variable expected")
            }
        }

        // Vars set
        let varset: HashSet<&NodeRc> = HashSet::from_iter(vars.iter().map(|x| x.clone()));

        // Derivatives
        let paths = self.all_simple_paths(vars);
        let mut derivs: HashMap<NodeRc, NodeRc> = HashMap::new();
        for v in varset.iter() {
            let mut d = ConstantScalar::new(0.);
            for path in paths.get(v).unwrap() {
                let mut prod = ConstantScalar::new(1.);
                for pair in path.as_slice().windows(2) {
                    prod = prod*pair[0].partial(&pair[1]);
                }
                d = d + prod;
            }
            derivs.insert((**v).clone(), d);
        }
        derivs
    }

    pub fn is_constant_with_value(&self, val: f64) -> bool {
        match self {
            NodeRc::ConstantScalarRc(x) => x.value() == val,
            _ => false
        }
    }
}

impl Node for NodeRc {
    
    fn arguments(&self) -> Vec<NodeRc> {
        match self {
            NodeRc::ConstantScalarRc(x) => x.arguments(),
            NodeRc::VariableScalarRc(x) => x.arguments(),
            NodeRc::FunctionAddRc(x) => x.arguments(),
            NodeRc::FunctionMulRc(x) => x.arguments(),
            NodeRc::FunctionDivRc(x) => x.arguments(),
        }
    }
    
    fn partial(&self, arg: &NodeRc) -> NodeRc { 
        match self {
            NodeRc::ConstantScalarRc(x) => x.partial(arg),
            NodeRc::VariableScalarRc(x) => x.partial(arg),
            NodeRc::FunctionAddRc(x) => x.partial(arg),
            NodeRc::FunctionMulRc(x) => x.partial(arg),
            NodeRc::FunctionDivRc(x) => x.partial(arg),
        }
    }

    fn value(&self) -> f64 {
        match self {
            NodeRc::ConstantScalarRc(x) => x.value(),
            NodeRc::VariableScalarRc(x) => x.value(),
            NodeRc::FunctionAddRc(x) => x.value(),
            NodeRc::FunctionMulRc(x) => x.value(),
            NodeRc::FunctionDivRc(x) => x.value(),
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

impl fmt::Debug for NodeRc {
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
                if self.is_constant_with_value(0.) {
                    rhs.clone()
                }
                else if rhs.is_constant_with_value(0.) {
                    self.clone()
                }
                else {
                    FunctionAdd::new(vec![self.clone(), rhs.clone()])
                }
            }        
        }
    };
}

macro_rules! impl_node_add_scalar {
    ($x: ty, $y: ty) => {
        impl Add<$y> for $x {
            type Output = NodeRc;
            fn add(self, rhs: $y) -> NodeRc {
                if self.is_constant_with_value(0.) {
                    ConstantScalar::new(rhs.to_f64().unwrap())
                }
                else if rhs == <$y>::zero() {
                    self.clone()
                }
                else {
                    FunctionAdd::new(
                        vec![self.clone(), 
                        ConstantScalar::new(rhs.to_f64().unwrap())])
                }
            }           
        }
        impl Add<$x> for $y {
            type Output = NodeRc;
            fn add(self, rhs: $x) -> NodeRc {
                if self == <$y>::zero() {
                    rhs.clone()
                }
                else if rhs.is_constant_with_value(0.) {
                    ConstantScalar::new(self.to_f64().unwrap())
                }
                else {
                    FunctionAdd::new(
                        vec![ConstantScalar::new(self.to_f64().unwrap()), 
                        rhs.clone()])
                }
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
                if self.is_constant_with_value(0.) || rhs.is_constant_with_value(0.) {
                    ConstantScalar::new(0.)
                }
                else if self.is_constant_with_value(1.) {
                    rhs.clone()
                }
                else if rhs.is_constant_with_value(1.) {
                    self.clone()
                }
                else {
                    FunctionMul::new(self.clone(), rhs.clone())
                }
            }        
        }
    };
}

macro_rules! impl_node_mul_scalar {
    ($x: ty, $y: ty) => {
        impl Mul<$y> for $x {
            type Output = NodeRc;
            fn mul(self, rhs: $y) -> NodeRc {
                if self.is_constant_with_value(0.) || rhs == <$y>::zero() {
                    ConstantScalar::new(0.)
                }
                else if self.is_constant_with_value(1.) {
                    ConstantScalar::new(rhs.to_f64().unwrap())
                }
                else if rhs == <$y>::one() {
                    self.clone()
                }
                else {
                    FunctionMul::new(self.clone(), 
                                     ConstantScalar::new(rhs.to_f64().unwrap()))
                }
            }           
        }
        impl Mul<$x> for $y {
            type Output = NodeRc;
            fn mul(self, rhs: $x) -> NodeRc {
                if self == <$y>::zero() || rhs.is_constant_with_value(0.) {
                    ConstantScalar::new(0.)
                }
                else if self == <$y>::one() {
                    rhs.clone()
                }
                else if rhs.is_constant_with_value(1.) {
                    ConstantScalar::new(self.to_f64().unwrap())
                }
                else {
                    FunctionMul::new(ConstantScalar::new(self.to_f64().unwrap()), 
                                     rhs.clone())
                }
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
                match self {
                    NodeRc::ConstantScalarRc(x) => ConstantScalar::new(-1.*x.value()),
                    _ => (-1.)*self,
                }
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
                if rhs.is_constant_with_value(0.) {
                    panic!("dividion by zero constant")
                }
                else if rhs.is_constant_with_value(1.) {
                    self.clone()
                }
                else if self.is_constant_with_value(0.) {
                    ConstantScalar::new(0.)
                }
                else {
                    FunctionDiv::new(self.clone(), rhs.clone())
                }
            }        
        }
    };
}

macro_rules! impl_node_div_scalar {
    ($x: ty, $y: ty) => {
        impl Div<$y> for $x {
            type Output = NodeRc;
            fn div(self, rhs: $y) -> NodeRc {
                if rhs == <$y>::zero() {
                    panic!("dividion by zero constant")
                }
                else if rhs == <$y>::one() {
                    self.clone()
                }
                else if self.is_constant_with_value(0.) {
                    ConstantScalar::new(0.)
                }
                else {
                    FunctionDiv::new(self.clone(), 
                                     ConstantScalar::new(rhs.to_f64().unwrap()))
                }
            }           
        }
        impl Div<$x> for $y {
            type Output = NodeRc;
            fn div(self, rhs: $x) -> NodeRc {
                if rhs.is_constant_with_value(0.) {
                    panic!("dividion by zero constant")
                }
                else if rhs.is_constant_with_value(1.) {
                    ConstantScalar::new(self.to_f64().unwrap())
                }
                else if self == <$y>::zero() {
                    ConstantScalar::new(0.)
                }
                else {
                    FunctionDiv::new(ConstantScalar::new(self.to_f64().unwrap()), 
                                     rhs.clone())
                }
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
    use crate::model::constant::ConstantScalar;

    #[test]
    fn node_add_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);
        let c = ConstantScalar::new(0.);

        let z1 = &x + &y;
        assert_eq!(format!("{}", z1), "x + y");
        assert_eq!(z1.value(), 7.);

        let z2 = &y + &x;
        assert_eq!(format!("{}", z2), "y + x");
        assert_eq!(z2.value(), 7.);

        let z3 = &x + (&y + &x);
        assert_eq!(format!("{}", z3), "x + y + x");
        assert_eq!(z3.value(), 10.);

        let z4 = (&x + &y) + &x;
        assert_eq!(format!("{}", z4), "x + y + x");
        assert_eq!(z4.value(), 10.);

        let z5 = &z1 + &z2 + &z3 + &z4;
        assert_eq!(format!("{}", z5), "x + y + y + x + x + y + x + x + y + x");
        assert_eq!(z5.value(), 34.);
    
        let z6 = (&x + &y) + (&y + &x);
        assert_eq!(format!("{}", z6), "x + y + y + x");
        assert_eq!(z6.value(), 14.);

        let z7 = &x + &c;
        assert_eq!(z7, x);

        let z8 = &c + &x;
        assert_eq!(z8, x);
    }

    #[test]
    fn node_add_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);

        let z1 = &x + 15.;
        assert_eq!(format!("{}", z1), "x + 15");
        assert_eq!(z1.value(), 18.);

        let z2 = 13. + &x;
        assert_eq!(format!("{}", z2), "13 + x");
        assert_eq!(z2.value(), 16.);

        let z3 = 2. + &z2 + 6.;
        assert_eq!(format!("{}", z3), "2 + 13 + x + 6");
        assert_eq!(z3.value(), 24.);

        let z4 = &x + 0.;
        assert_eq!(z4, x);

        let z5 = 0. + &x;
        assert_eq!(z5, x);
    }

    #[test]
    fn node_mul_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);
        let c0 = ConstantScalar::new(0.);
        let c1 = ConstantScalar::new(1.);

        let z1 = &x*&y;
        assert_eq!(format!("{}", z1), "x*y");
        assert_eq!(z1.value(), 12.);

        let z2 = &y*&x;
        assert_eq!(format!("{}", z2), "y*x");
        assert_eq!(z2.value(), 12.);

        let z3 = (&y*&x)*&x;
        assert_eq!(format!("{}", z3), "y*x*x");
        assert_eq!(z3.value(), 36.);

        let z4 = &y*(&x*&x);
        assert_eq!(format!("{}", z4), "y*x*x");
        assert_eq!(z4.value(), 36.);

        let z5 = &z4*(&x*&z3);
        assert_eq!(format!("{}", z5), "y*x*x*x*y*x*x");
        assert_eq!(z5.value(), (4.).pow(2.)*((3.).pow(5.)));

        let z6 = &x*&c0;
        assert!(z6.is_constant_with_value(0.));

        let z7 = &c0*&x;
        assert!(z7.is_constant_with_value(0.));

        let z8 = &c1*&x;
        assert_eq!(z8, x);

        let z9 = &x*&c1;
        assert_eq!(z9, x);

        let z10 = &c1*&c0;
        assert!(z10.is_constant_with_value(0.));

        let z11 = &c0*&c1;
        assert!(z11.is_constant_with_value(0.));
    }

    #[test]
    fn node_mul_add_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = (&x + &y*&x)*(&y*&x + &y);
        assert_eq!(format!("{}", z1), "(x + y*x)*(y*x + y)");
        assert_eq!(z1.value(), 15.*16.);
    }

    #[test]
    fn node_mul_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c1 = ConstantScalar::new(1.);

        let z1 = &x*15.;
        assert_eq!(format!("{}", z1), "x*15");
        assert_eq!(z1.value(), 45.);

        let z2 = 13.*&x;
        assert_eq!(format!("{}", z2), "13*x");
        assert_eq!(z2.value(), 39.);

        let z3 = 2.*&z2*6.;
        assert_eq!(format!("{}", z3), "2*13*x*6");
        assert_eq!(z3.value(), 2.*13.*3.*6.);

        let z4 = &x*0.;
        assert!(z4.is_constant_with_value(0.));

        let z5 = 0.*&x;
        assert!(z5.is_constant_with_value(0.));

        let z6 = &x*1.;
        assert_eq!(z6, x);

        let z7 = 1.*&x;
        assert_eq!(z7, x);

        let z8 = &c1*0.;
        assert!(z8.is_constant_with_value(0.));

        let z9 = 0.*&c1;
        assert!(z9.is_constant_with_value(0.));
    }

    #[test]
    fn node_neg() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c = ConstantScalar::new(5.);

        let z1 = -&x;
        assert_eq!(format!("{}", z1), "-1*x");
        assert_eq!(z1.value(), -3.);

        let z2 = -(&x + 3.);
        assert_eq!(format!("{}", z2), "-1*(x + 3)");
        assert_eq!(z2.value(), -6.);

        let z3 = -&c;
        assert!(z3.is_constant_with_value(-5.));
    }

    #[test]
    fn node_sub_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);

        let z1 = &x - &y;
        assert_eq!(z1.value(), -1.);
        assert_eq!(format!("{}", z1), "x + -1*y");

        let z2 = &y - &x;
        assert_eq!(z2.value(), 1.);
        assert_eq!(format!("{}", z2), "y + -1*x");

        let z3 = &x - (&x - &y);
        assert_eq!(z3.value(), 4.);
        assert_eq!(format!("{}", z3), "x + -1*(x + -1*y)");

        let z4 = (&x - &y) - &y;
        assert_eq!(z4.value(), -5.);

        let z5 = &z4 - &z3 - &x;
        assert_eq!(z5.value(), -12.);

        let z6 = (&z1 - &z2) - (&z3 - &z4);
        assert_eq!(z6.value(), -2.-9.);
    }

    #[test]
    fn node_sub_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);

        let z1 = &x - 15.;
        assert_eq!(format!("{}", z1), "x + -1*15");
        assert_eq!(z1.value(), -12.);

        let z2 = 13. - &x;
        assert_eq!(format!("{}", z2), "13 + -1*x");
        assert_eq!(z2.value(), 10.);

        let z3 = 2. - &z2 - 6.;
        assert_eq!(format!("{}", z3), "2 + -1*(13 + -1*x) + -1*6");
        assert_eq!(z3.value(), -14.);
    }

    #[test]
    fn node_div_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);
        let c0 = ConstantScalar::new(0.);
        let c1 = ConstantScalar::new(1.);

        let z1 = &x/&y;
        assert_eq!(format!("{}", z1), "x/y");
        assert_eq!(z1.value(), 3./4.);

        let z2 = (3.*&x)/(4.*&y);
        assert_eq!(format!("{}", z2), "3*x/(4*y)");
        assert_eq!(z2.value(), 9./16.);

        let z3 = (3. + &x)/(&y + 4.);
        assert_eq!(format!("{}", z3), "(3 + x)/(y + 4)");
        assert_eq!(z3.value(), 6./8.);

        let z4 = &x/(3.+&y);
        assert_eq!(format!("{}", z4), "x/(3 + y)");
        assert_eq!(z4.value(), 3./7.);

        let z5 = (2.+&x)/&y;
        assert_eq!(format!("{}", z5), "(2 + x)/y");
        assert_eq!(z5.value(), 5./4.);

        let z6 = &x/&c1;
        assert_eq!(z6, x);

        let z7 = &c0/&x;
        assert!(z7.is_constant_with_value(0.));
    }

    #[test]
    fn node_div_scalar() {

        let x = VariableScalar::new_continuous("x", 4.);
        let c1 = ConstantScalar::new(1.);

        let z1 = 3./&x;
        assert_eq!(format!("{}", z1), "3/x");
        assert_eq!(z1.value(), 3./4.);

        let z2 = 3./(&x + 1.);
        assert_eq!(format!("{}", z2), "3/(x + 1)");
        assert_eq!(z2.value(), 3./5.);

        let z3 = &x/3.;
        assert_eq!(format!("{}", z3), "x/3");
        assert_eq!(z3.value(), 4./3.);

        let z4 = (&x + 1.)/3.;
        assert_eq!(format!("{}", z4), "(x + 1)/3");
        assert_eq!(z4.value(), 5./3.);

        let z5 = &x/1.;
        assert_eq!(z5, x);

        let z6 = 0./&x;
        assert!(z6.is_constant_with_value(0.));

        let z7 = 0./&c1;
        assert!(z7.is_constant_with_value(0.));
    }

    #[test]
    fn all_simple_paths() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);
        let z = VariableScalar::new_continuous("z", 5.);

        let p1 = x.all_simple_paths(&vec![&x, &y, &z]);
        println!("p1 = {:?}", p1);
        assert_eq!(p1.get(&x).unwrap().len(), 1);
        assert_eq!(p1.get(&x).unwrap()[0].len(), 1);
        assert_eq!(p1.get(&y).unwrap().len(), 0);
        assert_eq!(p1.get(&z).unwrap().len(), 0);

        let p2 = (&x + 1.).all_simple_paths(&vec![&x, &y, &z]);
        println!("p2 = {:?}", p2);
        assert_eq!(p2.get(&x).unwrap().len(), 1);
        assert_eq!(p2.get(&x).unwrap()[0].len(), 2);
        assert_eq!(p2.get(&y).unwrap().len(), 0);
        assert_eq!(p2.get(&z).unwrap().len(), 0);

        let p3 = (4. + 3.*(&z + &x)).all_simple_paths(&vec![&x, &y, &z]);
        println!("p3 = {:?}", p3);
        assert_eq!(p3.get(&x).unwrap().len(), 1);
        assert_eq!(p3.get(&x).unwrap()[0].len(), 4);
        assert_eq!(p3.get(&y).unwrap().len(), 0);
        assert_eq!(p3.get(&z).unwrap().len(), 1);
        assert_eq!(p3.get(&z).unwrap()[0].len(), 4);

        let f4 = &x + 5.;
        let g4 = &f4*(&z + 3.);
        let p4 = (f4 + g4).all_simple_paths(&vec![&x, &y, &z]);
        println!("p4 = {:?}", p4);
        assert_eq!(p4.get(&x).unwrap().len(), 2);
        assert_eq!(p4.get(&x).unwrap()[0].len() +
                   p4.get(&x).unwrap()[1].len(), 7);
        assert_eq!(p4.get(&y).unwrap().len(), 0);
        assert_eq!(p4.get(&z).unwrap().len(), 1);
        assert_eq!(p4.get(&z).unwrap()[0].len(), 4);
    }
}