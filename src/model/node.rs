use std::ops::Add;

use crate::model::constant::ConstantScalar;
use crate::model::variable::VariableScalar;
use crate::model::function::add::FunctionAdd;

pub enum NodeType {
    ConstantScalarType(ConstantScalar),
    VariableScalarType(VariableScalar),
    FunctionAddType(FunctionAdd),
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

impl Add<NodeType> for NodeType {      
    
    type Output = NodeType;
    
    fn add(self, rhs: NodeType) -> NodeType {

        // self is constant zero

        // rhs is constnat zero

        // flag args

        FunctionAdd::new(vec![self, rhs])
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

        let z = x + y;

        println!("vamos z = {}", z.get_value());
    }

    // #[test]
    // fn node_add_scalar() {

    //     let x = VariableScalar::new_continuous("x", 3.);

    //     let z = x + 4.;
    // }

    // #[test]
    // fn scalar_add_node() {

    //     let x = VariableScalar::new_continuous("x", 3.);

    //     let z = 4. + x;
    // }
}