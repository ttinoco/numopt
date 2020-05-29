//! Sparse matrix data structures and tools.

mod coo;
mod csr;
mod item;

pub use crate::matrix::coo::{CooMat, CooMatIter};
pub use crate::matrix::csr::{CsrMat};






