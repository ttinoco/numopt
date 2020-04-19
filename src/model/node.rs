use std::fmt;
use std::rc::Rc;
use std::ops::Add;
use num_traits::cast::ToPrimitive;

use crate::model::constant::ConstantScalar;
use crate::model::variable::VariableScalar;
use crate::model::function::add::FunctionAdd;

pub enum NodeType {
    ConstantScalarType(Rc<ConstantScalar>),
    VariableScalarType(Rc<VariableScalar>),
    FunctionAddType(Rc<FunctionAdd>),
}

pub trait Node {

    fn get_value(&self) -> f64;
    
}

impl Node for NodeType {

    fn get_value(&self) -> f64 {
        match self {
           NodeType::ConstantScalarType(x) => x.get_value(),
           NodeType::VariableScalarType(x) => x.get_value(),
           NodeType::FunctionAddType(x) => x.get_value(),
        }
    }

}

impl Clone for NodeType {
    fn clone(&self) -> Self {
        match self {
            NodeType::ConstantScalarType(x) => NodeType::ConstantScalarType(Rc::clone(&x)),
            NodeType::VariableScalarType(x) => NodeType::VariableScalarType(Rc::clone(&x)),
            NodeType::FunctionAddType(x) => NodeType::FunctionAddType(Rc::clone(&x)),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::ConstantScalarType(x) => write!(f, "{}", x),
            NodeType::VariableScalarType(x) => write!(f, "{}", x),
            NodeType::FunctionAddType(x) => write!(f, "{}", x),
        }
    }
}

impl Add<&NodeType> for &NodeType {      
    type Output = NodeType;
    fn add(self, rhs: &NodeType) -> NodeType {
        FunctionAdd::new(vec![self.clone(), rhs.clone()])
    } 
}

impl Add<&NodeType> for NodeType {      
    type Output = NodeType;
    fn add(self, rhs: &NodeType) -> NodeType {
        FunctionAdd::new(vec![self.clone(), rhs.clone()])
    } 
}

impl Add<NodeType> for &NodeType {      
    type Output = NodeType;
    fn add(self, rhs: NodeType) -> NodeType {
        FunctionAdd::new(vec![self.clone(), rhs.clone()])
    } 
}

impl Add<f64> for &NodeType {
    type Output = NodeType;
    fn add(self, rhs: f64) -> NodeType {
        FunctionAdd::new(vec![
            self.clone(), 
            ConstantScalar::new(rhs.to_f64().unwrap())])
    }
}

impl Add<&NodeType> for f64 {
    type Output = NodeType;
    fn add(self, rhs: &NodeType) -> NodeType {
        FunctionAdd::new(vec![
            ConstantScalar::new(self.to_f64().unwrap()), 
            rhs.clone()])
    } 
}

impl Add<f64> for NodeType {
    type Output = NodeType;
    fn add(self, rhs: f64) -> NodeType {
        FunctionAdd::new(vec![
            self.clone(), 
            ConstantScalar::new(rhs.to_f64().unwrap())])
    }
}

impl Add<NodeType> for f64 {
    type Output = NodeType;
    fn add(self, rhs: NodeType) -> NodeType {
        FunctionAdd::new(vec![
            ConstantScalar::new(self.to_f64().unwrap()), 
            rhs.clone()])
    } 
}

#[cfg(test)]
mod tests {

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
}