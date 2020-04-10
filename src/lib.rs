//! Optimization problem abstractions and solver interfaces.
//! 
//! ## Features
//! - Abstractions for Minlp, Nlp, Milp, and Lp optimization problems.
//! - Interfaces for COIN-OR optimization solvers Cbc, Clp, and Ipopt.

pub mod problem;
pub mod solver;
pub mod model;
pub mod matrix;
pub mod macros;