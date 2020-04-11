//! Sparse matrix data structures and iterators.

pub mod coo;
pub mod csr;
pub mod item;

pub use crate::matrix::coo::{CooMat, CooMatIter};
pub use crate::matrix::csr::{CsrMat};






