//! Optimization solver interfaces.

pub mod base;
pub mod clp_cmd;
pub mod cbc_cmd;

#[cfg(feature = "ipopt")] 
pub mod ipopt;
