# numopt

[![crate](https://img.shields.io/crates/v/numopt.svg)](https://crates.io/crates/numopt)
[![documentation](https://docs.rs/numopt/badge.svg)](https://docs.rs/numopt)

Numerical optimization problem abstractions, solver interfaces, and modeling tools.

## Contents

* Problem abstractions
  * Lp
  * Nlp
  * Milp
  * Minlp
* Solver interfaces
  * Cbc (via command-line)
  * Clp (via command-line)
  * Ipopt (via linking with "libipopt" library) (feature "ipopt")
* Modeling tools
  * Scalar expressions and variables.
  * Add, divide, multiply, subtract, negate, cosine, and sine functions.
  * Automatic sparse differentiation.
