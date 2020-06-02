//! Optimization algebraic modeling tools.

pub mod node;
pub mod node_base;
pub mod node_func;
pub mod node_diff;
pub mod node_cmp;
pub mod node_std;
pub mod constant;
pub mod variable;
pub mod function;
pub mod constraint;
pub mod constraint_std;
pub mod model;
pub mod model_std;

pub use variable::VariableScalar;
pub use constant::ConstantScalar;
pub use node::Node;
pub use node_cmp::NodeCmp;
pub use model::Model;
pub use model::Objective;