use std::fmt;
use simple_error::SimpleError;
use std::collections::HashMap;

use crate::problem::{ProblemSol};

#[derive(Debug, PartialEq)]
pub enum SolverStatus {
    Solved,
    Unknown,
    Error,
}

#[derive(Clone)]
pub enum SolverParam {
    IntParam(i32),
    FloatParam(f64),
    StrParam(String),
}

pub trait Solver<T> {
    fn new(p: &T) -> Self;
    fn get_param(&self, name: &str) -> Option<&SolverParam> { self.get_params().get(name) }
    fn get_params(&self) -> &HashMap<String, SolverParam>;
    fn get_params_mut(&mut self) -> &mut HashMap<String, SolverParam>;
    fn status(&self) -> &SolverStatus;
    fn solution(&self) -> &Option<ProblemSol>;
    fn solve(&mut self, p: &mut T) -> Result<(), SimpleError>;
    fn set_param(&mut self, name: &str, value: SolverParam) -> Result<(), SimpleError> { 
       
        let v = match self.get_params_mut().get_mut(name) {
            Some(x) => x,
            None => return Err(SimpleError::new("unknown parameter"))
        };

        *v = match ((*v).clone(), value) {
            (SolverParam::IntParam(_x), SolverParam::IntParam(y)) => { 
                SolverParam::IntParam(y) 
            },
            (SolverParam::FloatParam(_x), SolverParam::FloatParam(y)) => { 
                SolverParam::FloatParam(y)
            },
            (SolverParam::StrParam(_x), SolverParam::StrParam(y)) => { 
                SolverParam::StrParam(y)
            }, 
            _ => return Err(SimpleError::new("invalid parameter type"))
        };    

        Ok(())
    }
}

impl SolverStatus {
    pub fn is_solved(&self) -> bool {
        match self {
            SolverStatus::Solved => true,
            _ => false
        }
    }
}

impl fmt::Display for SolverStatus {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolverStatus::Error => write!(f, "error"),
            SolverStatus::Unknown => write!(f, "unknown"),
            SolverStatus::Solved => write!(f, "solved")
        }
    }
}

