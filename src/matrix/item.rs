use num_traits::{Zero};
use std::ops::{Mul, AddAssign};

pub trait MatItem: Zero + Mul<Output=Self> + AddAssign + Clone + Copy {}

impl<T: Zero + Mul<Output=Self> + AddAssign + Clone + Copy> MatItem for T {}