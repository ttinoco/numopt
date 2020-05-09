
use crate::model::node::{NodeBase, NodeRef};
use crate::model::constant::ConstantScalar;
use crate::model::function::cos::FunctionCos;
use crate::model::function::sin::FunctionSin;

pub trait NodeFunc {

    fn cos(&self) -> NodeRef;
    fn sin(&self) -> NodeRef;
}

impl NodeFunc for NodeRef {

    fn cos(&self) -> NodeRef {
        match self {
            NodeRef::ConstantScalar(x) => {
                ConstantScalar::new((**x).borrow().value().cos())
            },
            _ =>  FunctionCos::new(self.clone())  
        }
    }

    fn sin(&self) -> NodeRef {
        match self {
            NodeRef::ConstantScalar(x) => {
                ConstantScalar::new((**x).borrow().value().sin())
            },
            _ =>  FunctionSin::new(self.clone())  
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::variable::VariableScalar;
    use crate::model::constant::ConstantScalar;

    #[test]
    fn node_cos() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c = ConstantScalar::new(5.);

        let z1 = x.cos();
        assert_eq!(format!("{}", z1), "cos(x)");
        assert_eq!(z1.value(), 3_f64.cos());

        let z2 = (3.*&x + 5.).cos();
        assert_eq!(format!("{}", z2), "cos(3*x + 5)");
        assert_eq!(z2.value(), (3.*3. + 5_f64).cos());

        let z3 = c.cos();
        assert!(z3.is_constant_with_value(5_f64.cos()));
    }

    #[test]
    fn node_sin() {

        let x = VariableScalar::new_continuous("x", 3.);
        let c = ConstantScalar::new(5.);

        let z1 = x.sin();
        assert_eq!(format!("{}", z1), "sin(x)");
        assert_eq!(z1.value(), 3_f64.sin());

        let z2 = (3.*&x + 5.).sin();
        assert_eq!(format!("{}", z2), "sin(3*x + 5)");
        assert_eq!(z2.value(), (3.*3. + 5_f64).sin());

        let z3 = c.sin();
        assert!(z3.is_constant_with_value(5_f64.sin()));
    }
}