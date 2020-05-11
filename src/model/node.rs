use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;
use simple_error::SimpleError;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use num_traits::cast::ToPrimitive;
use num_traits::identities::{Zero, One};
use std::ops::{Add, Mul, Neg, Sub, Div};

use crate::model::constant::ConstantScalar;
use crate::model::variable::VariableScalar;
use crate::model::function::add::FunctionAdd;
use crate::model::function::mul::FunctionMul;
use crate::model::function::div::FunctionDiv;
use crate::model::function::cos::FunctionCos;
use crate::model::function::sin::FunctionSin;

pub enum NodeRef {
    ConstantScalar(Rc<RefCell<ConstantScalar>>),
    VariableScalar(Rc<RefCell<VariableScalar>>),
    FunctionAdd(Rc<RefCell<FunctionAdd>>),
    FunctionCos(Rc<RefCell<FunctionCos>>),
    FunctionDiv(Rc<RefCell<FunctionDiv>>),
    FunctionMul(Rc<RefCell<FunctionMul>>),
    FunctionSin(Rc<RefCell<FunctionSin>>),
}

pub trait NodeBase {

    fn arguments(&self) -> Vec<NodeRef> { Vec::new() }
    fn partial(&self, arg: &NodeRef) -> NodeRef;
    fn update_value(&mut self, _value: f64) -> Result<(), SimpleError> { 
        panic!("can only update value of variables")
    }
    fn value(&self) -> f64;
}

impl NodeRef {

    pub fn is_constant(&self) -> bool {
        match self {
            NodeRef::ConstantScalar(_) => true,
            _ => false
        }
    }

    pub fn is_constant_with_value(&self, val: f64) -> bool {
        match self {
            NodeRef::ConstantScalar(x) => {
                (**x).borrow().value() == val
            },
            _ => false
        }
    }
}

impl NodeBase for NodeRef {
    
    fn arguments(&self) -> Vec<NodeRef> {
        match self {
            NodeRef::ConstantScalar(x) => (**x).borrow().arguments(),
            NodeRef::VariableScalar(x) => (**x).borrow().arguments(),
            NodeRef::FunctionAdd(x) => (**x).borrow().arguments(),
            NodeRef::FunctionCos(x) => (**x).borrow().arguments(),
            NodeRef::FunctionDiv(x) => (**x).borrow().arguments(),
            NodeRef::FunctionMul(x) => (**x).borrow().arguments(),
            NodeRef::FunctionSin(x) => (**x).borrow().arguments(),
        }
    }
    
    fn partial(&self, arg: &NodeRef) -> NodeRef { 
        match self {
            NodeRef::ConstantScalar(x) => (**x).borrow().partial(arg),
            NodeRef::VariableScalar(x) => (**x).borrow().partial(arg),
            NodeRef::FunctionAdd(x) => (**x).borrow().partial(arg),
            NodeRef::FunctionCos(x) => (**x).borrow().partial(arg),
            NodeRef::FunctionDiv(x) => (**x).borrow().partial(arg),
            NodeRef::FunctionMul(x) => (**x).borrow().partial(arg),
            NodeRef::FunctionSin(x) => (**x).borrow().partial(arg),
        }
    }

    fn update_value(&mut self, value: f64) -> Result<(), SimpleError> {
        match self {
            NodeRef::VariableScalar(x) => (**x).borrow_mut().update_value(value),
            _ => panic!("can only update value of variables")
        }
    }

    fn value(&self) -> f64 {
        match self {
            NodeRef::ConstantScalar(x) => (**x).borrow().value(),
            NodeRef::VariableScalar(x) => (**x).borrow().value(),
            NodeRef::FunctionAdd(x) => (**x).borrow().value(),
            NodeRef::FunctionCos(x) => (**x).borrow().value(),
            NodeRef::FunctionDiv(x) => (**x).borrow().value(),
            NodeRef::FunctionMul(x) => (**x).borrow().value(),
            NodeRef::FunctionSin(x) => (**x).borrow().value(),            
        }
    }
}

impl Hash for NodeRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            NodeRef::ConstantScalar(x) => ptr::hash(&**x, state),
            NodeRef::VariableScalar(x) => ptr::hash(&**x, state),
            NodeRef::FunctionAdd(x) => ptr::hash(&**x, state),
            NodeRef::FunctionCos(x) => ptr::hash(&**x, state),
            NodeRef::FunctionDiv(x) => ptr::hash(&**x, state),
            NodeRef::FunctionMul(x) => ptr::hash(&**x, state),
            NodeRef::FunctionSin(x) => ptr::hash(&**x, state),
        };
    }
}

impl PartialEq for NodeRef {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeRef::ConstantScalar(x), NodeRef::ConstantScalar(y)) => Rc::ptr_eq(x, y),
            (NodeRef::VariableScalar(x), NodeRef::VariableScalar(y)) => Rc::ptr_eq(x, y),
            (NodeRef::FunctionAdd(x), NodeRef::FunctionAdd(y)) => Rc::ptr_eq(x, y),
            (NodeRef::FunctionCos(x), NodeRef::FunctionCos(y)) => Rc::ptr_eq(x, y),
            (NodeRef::FunctionDiv(x), NodeRef::FunctionDiv(y)) => Rc::ptr_eq(x, y),
            (NodeRef::FunctionMul(x), NodeRef::FunctionMul(y)) => Rc::ptr_eq(x, y),
            (NodeRef::FunctionSin(x), NodeRef::FunctionSin(y)) => Rc::ptr_eq(x, y),
            _ => false,
        }
    }
}

impl Eq for NodeRef {}

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        match self {
            NodeRef::ConstantScalar(x) => NodeRef::ConstantScalar(Rc::clone(&x)),
            NodeRef::VariableScalar(x) => NodeRef::VariableScalar(Rc::clone(&x)),
            NodeRef::FunctionAdd(x) => NodeRef::FunctionAdd(Rc::clone(&x)),
            NodeRef::FunctionCos(x) => NodeRef::FunctionCos(Rc::clone(&x)),
            NodeRef::FunctionDiv(x) => NodeRef::FunctionDiv(Rc::clone(&x)),
            NodeRef::FunctionMul(x) => NodeRef::FunctionMul(Rc::clone(&x)),
            NodeRef::FunctionSin(x) => NodeRef::FunctionSin(Rc::clone(&x)), 
        }
    }
}

impl fmt::Display for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRef::ConstantScalar(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::VariableScalar(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionAdd(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionCos(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionDiv(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionMul(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionSin(x) => write!(f, "{}", (**x).borrow()),
            
        }
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRef::ConstantScalar(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::VariableScalar(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionAdd(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionCos(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionDiv(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionMul(x) => write!(f, "{}", (**x).borrow()),
            NodeRef::FunctionSin(x) => write!(f, "{}", (**x).borrow()),
        }
    }
}

macro_rules! impl_node_add_node {
    ($x: ty, $y: ty) => {
        impl Add<$y> for $x {
            type Output = NodeRef;
            fn add(self, rhs: $y) -> NodeRef {

                // Self zero
                if self.is_constant_with_value(0.) {
                    rhs.clone()
                }

                // Rhs zero
                else if rhs.is_constant_with_value(0.) {
                    self.clone()
                }

                // Both constants
                else if self.is_constant() && rhs.is_constant() {
                    ConstantScalar::new(self.value() + rhs.value())
                }

                // Other
                else {
                    let mut args: Vec<NodeRef> = Vec::new();
                    for a in &[self.clone(), rhs.clone()] {
                        match a {
                            NodeRef::FunctionAdd(x) => {
                                args.extend((**x).borrow().arguments()) // flatten add
                            },
                            _ => args.push(a.clone()),
                        };
                    }
                    FunctionAdd::new(args)
                }
            }        
        }
    };
}

macro_rules! impl_node_add_scalar {
    ($x: ty, $y: ty) => {
        impl Add<$y> for $x {
            type Output = NodeRef;
            fn add(self, rhs: $y) -> NodeRef {

                // Self constant
                if self.is_constant() {
                    ConstantScalar::new(self.value() + rhs.to_f64().unwrap())
                }

                // Rhs zero
                else if rhs == <$y>::zero() {
                    self.clone()
                }

                // Other
                else {
                    let mut args: Vec<NodeRef> = Vec::new();
                    for a in &[self.clone(), ConstantScalar::new(rhs.to_f64().unwrap())] {
                        match a {
                            NodeRef::FunctionAdd(x) => {
                                args.extend((**x).borrow().arguments())
                            },
                            _ => args.push(a.clone()),
                        };
                    }
                    FunctionAdd::new(args)
                }
            }           
        }
        impl Add<$x> for $y {
            type Output = NodeRef;
            fn add(self, rhs: $x) -> NodeRef {

                // Self zero
                if self == <$y>::zero() {
                    rhs.clone()
                }

                // Rhs constant
                else if rhs.is_constant() {
                    ConstantScalar::new(self.to_f64().unwrap() + rhs.value())
                }

                // Other
                else {
                    let mut args: Vec<NodeRef> = Vec::new();
                    for a in &[ConstantScalar::new(self.to_f64().unwrap()), rhs.clone()] {
                        match a {
                            NodeRef::FunctionAdd(x) => {
                                args.extend((**x).borrow().arguments())
                            },
                            _ => args.push(a.clone()),
                        };
                    }
                    FunctionAdd::new(args)
                }
            }           
        }
    };
}

impl_node_add_node!(&NodeRef, &NodeRef);
impl_node_add_node!(&NodeRef, NodeRef);
impl_node_add_node!(NodeRef, &NodeRef);
impl_node_add_node!(NodeRef, NodeRef);
impl_node_add_scalar!(&NodeRef, f64);
impl_node_add_scalar!(NodeRef, f64);

macro_rules! impl_node_mul_node {
    ($x: ty, $y: ty) => {
        impl Mul<$y> for $x {
            type Output = NodeRef;
            fn mul(self, rhs: $y) -> NodeRef {

                // Self or rhs zero
                if self.is_constant_with_value(0.) || rhs.is_constant_with_value(0.) {
                    ConstantScalar::new(0.)
                }

                // Self one
                else if self.is_constant_with_value(1.) {
                    rhs.clone()
                }

                // Rhs one
                else if rhs.is_constant_with_value(1.) {
                    self.clone()
                }

                // Both constants
                else if self.is_constant() && rhs.is_constant() {
                    ConstantScalar::new(self.value()*rhs.value())
                }

                // Other
                else {
                    let s = self.clone();
                    let r = rhs.clone();
                    match (&s, &r) {

                        // Constant times add
                        (NodeRef::ConstantScalar(_x), NodeRef::FunctionAdd(_y)) => {
                            FunctionAdd::new(r.arguments().iter().map(|x| &s*x).collect())
                        },

                        // Add time constant
                        (NodeRef::FunctionAdd(_x), NodeRef::ConstantScalar(_y)) => {
                            FunctionAdd::new(s.arguments().iter().map(|x| x*&r).collect())
                        },

                        // Other
                        _ => FunctionMul::new(s, r),
                    }
                }
            }        
        }
    };
}

macro_rules! impl_node_mul_scalar {
    ($x: ty, $y: ty) => {
        impl Mul<$y> for $x {
            type Output = NodeRef;
            fn mul(self, rhs: $y) -> NodeRef {

                // Self constant or rhs zero 
                if self.is_constant() || rhs == <$y>::zero() {
                    ConstantScalar::new(self.value()*rhs.to_f64().unwrap())
                }

                // Self one
                else if self.is_constant_with_value(1.) {
                    ConstantScalar::new(rhs.to_f64().unwrap())
                }

                // Rhs one
                else if rhs == <$y>::one() {
                    self.clone()
                }

                // Other
                else {
                    let s = self.clone();
                    let r = rhs.to_f64().unwrap();
                    match &s {

                        // Add times constant
                        NodeRef::FunctionAdd(_x) => {
                            FunctionAdd::new(s.arguments().iter().map(|x| x*r).collect())
                        },

                        // Other
                        _ => FunctionMul::new(s, ConstantScalar::new(r))
                    }
                }
            }           
        }
        impl Mul<$x> for $y {
            type Output = NodeRef;
            fn mul(self, rhs: $x) -> NodeRef {

                // Self zero or rhs constant 
                if self == <$y>::zero() || rhs.is_constant() {
                    ConstantScalar::new(self.to_f64().unwrap()*rhs.value())
                }

                // Self one
                else if self == <$y>::one() {
                    rhs.clone()
                }

                // Rhs one
                else if rhs.is_constant_with_value(1.) {
                    ConstantScalar::new(self.to_f64().unwrap())
                }

                // Other
                else {
                    let s = self.to_f64().unwrap();
                    let r = rhs.clone();
                    match &r {

                        // Constant times add
                        NodeRef::FunctionAdd(_x) => {
                            FunctionAdd::new(r.arguments().iter().map(|x| s*x).collect())
                        },

                        // Other
                        _ => FunctionMul::new(ConstantScalar::new(s), r),
                    }
                }
            }           
        }
    };
}

impl_node_mul_node!(&NodeRef, &NodeRef);
impl_node_mul_node!(&NodeRef, NodeRef);
impl_node_mul_node!(NodeRef, &NodeRef);
impl_node_mul_node!(NodeRef, NodeRef);
impl_node_mul_scalar!(&NodeRef, f64);
impl_node_mul_scalar!(NodeRef, f64);

macro_rules! impl_node_neg {
    ($x: ty) => {
        impl Neg for $x {
            type Output = NodeRef;
            fn neg(self) -> NodeRef {
                match self {
                    NodeRef::ConstantScalar(x) => {
                        ConstantScalar::new(-1.*((*x).borrow().value()))
                    },
                    _ => (-1.)*self,
                }
            }        
        }
    };
}

impl_node_neg!(&NodeRef);
impl_node_neg!(NodeRef);

macro_rules! impl_node_sub_node {
    ($x: ty, $y: ty) => {
        impl Sub<$y> for $x {
            type Output = NodeRef;
            fn sub(self, rhs: $y) -> NodeRef {
                self + -1.*rhs
            }        
        }
    };
}

macro_rules! impl_node_sub_scalar {
    ($x: ty, $y: ty) => {
        impl Sub<$y> for $x {
            type Output = NodeRef;
            fn sub(self, rhs: $y) -> NodeRef {
                self + -1.*ConstantScalar::new(rhs.to_f64().unwrap())
            }           
        }
        impl Sub<$x> for $y {
            type Output = NodeRef;
            fn sub(self, rhs: $x) -> NodeRef {
                ConstantScalar::new(self.to_f64().unwrap()) + -1.*rhs
            }           
        }
    };
}

impl_node_sub_node!(&NodeRef, &NodeRef);
impl_node_sub_node!(&NodeRef, NodeRef);
impl_node_sub_node!(NodeRef, &NodeRef);
impl_node_sub_node!(NodeRef, NodeRef);
impl_node_sub_scalar!(&NodeRef, f64);
impl_node_sub_scalar!(NodeRef, f64);

macro_rules! impl_node_div_node {
    ($x: ty, $y: ty) => {
        impl Div<$y> for $x {
            type Output = NodeRef;
            fn div(self, rhs: $y) -> NodeRef {

                // Rhs is zero
                if rhs.is_constant_with_value(0.) {
                    panic!("dividion by zero constant")
                }

                // Rhs is one
                else if rhs.is_constant_with_value(1.) {
                    self.clone()
                }

                // Self is zero
                else if self.is_constant_with_value(0.) {
                    ConstantScalar::new(0.)
                }

                // Both are constants
                else if self.is_constant() && rhs.is_constant() {
                    ConstantScalar::new(self.value()/rhs.value())
                }

                // Other
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
            type Output = NodeRef;
            fn div(self, rhs: $y) -> NodeRef {

                // Rhs is zero
                if rhs == <$y>::zero() {
                    panic!("dividion by zero constant")
                }

                // Rhs is one
                else if rhs == <$y>::one() {
                    self.clone()
                }

                // Self is zero
                else if self.is_constant_with_value(0.) {
                    ConstantScalar::new(0.)
                }

                // Both are constants
                else if self.is_constant() {
                    ConstantScalar::new(self.value()/rhs.to_f64().unwrap())
                }

                // Other
                else {
                    FunctionDiv::new(self.clone(), 
                                     ConstantScalar::new(rhs.to_f64().unwrap()))
                }
            }           
        }
        impl Div<$x> for $y {
            type Output = NodeRef;
            fn div(self, rhs: $x) -> NodeRef {

                // Rhs is zero
                if rhs.is_constant_with_value(0.) {
                    panic!("dividion by zero constant")
                }

                // Rhs is oen
                else if rhs.is_constant_with_value(1.) {
                    ConstantScalar::new(self.to_f64().unwrap())
                }

                // Self is zero
                else if self == <$y>::zero() {
                    ConstantScalar::new(0.)
                }

                // Both are constants
                else if rhs.is_constant() {
                    ConstantScalar::new(self.to_f64().unwrap()/rhs.value())
                }

                // Other
                else {
                    FunctionDiv::new(ConstantScalar::new(self.to_f64().unwrap()), 
                                     rhs.clone())
                }
            }           
        }
    };
}

impl_node_div_node!(&NodeRef, &NodeRef);
impl_node_div_node!(&NodeRef, NodeRef);
impl_node_div_node!(NodeRef, &NodeRef);
impl_node_div_node!(NodeRef, NodeRef);
impl_node_div_scalar!(&NodeRef, f64);
impl_node_div_scalar!(NodeRef, f64);

#[cfg(test)]
mod tests {

    use num_traits::pow::Pow;

    use crate::model::node::NodeBase;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

    #[test]
    fn node_update_value() {

        let mut x = VariableScalar::new_continuous("x", 10.);
        assert_eq!(x.value(), 10.);

        x.update_value(11.).unwrap();
        assert_eq!(x.value(), 11.);
    }

    #[test]
    fn node_add_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);
        let c0 = ConstantScalar::new(0.);
        let c1 = ConstantScalar::new(1.);
        let c2 = ConstantScalar::new(2.);

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

        let z7 = &x + &c0;
        assert_eq!(z7, x);

        let z8 = &c0 + &x;
        assert_eq!(z8, x);

        let z9 = (&x + 1.) + &y;
        assert_eq!(format!("{:?}", z9.arguments()), "[x, 1, y]");
        assert_eq!(z9.value(), 8.);

        let z10 = &x + (&y + 5.);
        assert_eq!(format!("{:?}", z10.arguments()), "[x, y, 5]");
        assert_eq!(z10.value(), 12.);

        let z11 = (&x + 2.) + (&y + 7.);
        assert_eq!(format!("{:?}", z11.arguments()), "[x, 2, y, 7]");
        assert_eq!(z11.value(), 16.);

        let z12 = &c1 + &c2;
        assert!(z12.is_constant_with_value(3.));
    }

    #[test]
    fn node_add_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c1 = ConstantScalar::new(1.);

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

        let z6 = (&x + 1.) + 2.;
        assert_eq!(format!("{:?}", z6.arguments()), "[x, 1, 2]");
        assert_eq!(z6.value(), 6.);

        let z7 = 3. + (&x + 4.);
        assert_eq!(format!("{:?}", z7.arguments()), "[3, x, 4]");
        assert_eq!(z7.value(), 10.);

        let z8 = 4. + &c1;
        assert!(z8.is_constant_with_value(5.));

        let z9 = &c1 + 5.;
        assert!(z9.is_constant_with_value(6.));
    }

    #[test]
    fn node_mul_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);
        let c0 = ConstantScalar::new(0.);
        let c1 = ConstantScalar::new(1.);
        let c2 = ConstantScalar::new(2.);
        let c3 = ConstantScalar::new(3.);

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

        let z12 = (&x + &y*&x)*(&y*&x + &y);
        assert_eq!(format!("{}", z12), "(x + y*x)*(y*x + y)");
        assert_eq!(z12.value(), 15.*16.);

        let z13 = &c3*&c2;
        assert!(z13.is_constant_with_value(6.));

        let z14 = &c3*(&x + 3.);
        assert_eq!(format!("{}", z14), "3*x + 9");
        assert_eq!(z14.value(), 18.);

        let z15 = (&x + &y)*&c2;
        assert_eq!(format!("{}", z15), "x*2 + y*2");
        assert_eq!(z15.value(), 14.);
    }

    #[test]
    fn node_mul_scalar() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c1 = ConstantScalar::new(1.);
        let c2 = ConstantScalar::new(2.);

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

        let z10 = 3.*&c2;
        assert!(z10.is_constant_with_value(6.));

        let z11 = &c2*5.;
        assert!(z11.is_constant_with_value(10.));

        let z12 = 4.*(&x + 3.);
        assert_eq!(format!("{}", z12), "4*x + 12");
        assert_eq!(z12.value(), 24.);

        let z13 = (4. + &x)*10.;
        assert_eq!(format!("{}", z13), "40 + x*10");
        assert_eq!(z13.value(), 70.);
    }

    #[test]
    fn node_neg() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c = ConstantScalar::new(5.);

        let z1 = -&x;
        assert_eq!(format!("{}", z1), "-1*x");
        assert_eq!(z1.value(), -3.);

        let z2 = -(&x + 3.);
        assert_eq!(format!("{}", z2), "-1*x + -3");
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
        assert_eq!(format!("{}", z3), "x + -1*x + -1*-1*y");

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
        assert_eq!(format!("{}", z1), "x + -15");
        assert_eq!(z1.value(), -12.);

        let z2 = 13. - &x;
        assert_eq!(format!("{}", z2), "13 + -1*x");
        assert_eq!(z2.value(), 10.);

        let z3 = 2. - &z2 - 6.;
        assert_eq!(format!("{}", z3), "2 + -13 + -1*-1*x + -6");
        assert_eq!(z3.value(), -14.);
    }

    #[test]
    fn node_div_node() {

        let x = VariableScalar::new_continuous("x", 3.);
        let y = VariableScalar::new_continuous("y", 4.);
        let c0 = ConstantScalar::new(0.);
        let c1 = ConstantScalar::new(1.);
        let c2 = ConstantScalar::new(2.);
        let c3 = ConstantScalar::new(3.);

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

        let z8 = &c2/&c3;
        assert!(z8.is_constant_with_value(2./3.));
    }

    #[test]
    fn node_div_scalar() {

        let x = VariableScalar::new_continuous("x", 4.);
        let c1 = ConstantScalar::new(1.);
        let c2 = ConstantScalar::new(2.);

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

        let z8 = &c2/4.;
        assert!(z8.is_constant_with_value(2./4.));

        let z9 = 5./&c2;
        assert!(z9.is_constant_with_value(5./2.));
    }
}