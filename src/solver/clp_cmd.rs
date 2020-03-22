use std::fs::File;
use std::ffi::OsStr;
use tempfile::Builder;
use num_traits::NumCast;
use std::io::prelude::*;
use std::fs::remove_file;
use std::process::Command;
use simple_error::SimpleError;
use std::io::{self, BufReader};

use crate::solver::{Solver, 
                    SolverStatus};
use crate::problem::{ProblemDims,
                     ProblemLp, 
                     ProblemLpIO,
                     ProblemSol};

pub struct SolverClpCMD<T: ProblemLp> {
    status: SolverStatus,
    solution: Option<ProblemSol<T>>,
}

impl<T: ProblemLp> SolverClpCMD<T> {

    fn read_sol_file(filename: &str, p: &T) -> io::Result<(SolverStatus, ProblemSol<T>)> {
        
        let mut name: String;
        let mut dtype: String;
        let mut index: usize;
        let mut value: T::N;
        let mut mul: T::N;
        let mut status = SolverStatus::Unknown;
        let mut solution = ProblemSol::new(p.nx(),p.na());
        let f = File::open(filename)?;
        let mut r = BufReader::new(f);
        let mut line = String::new();
        let e = io::Error::new(io::ErrorKind::Other, "bad clp solution file");

        // Status
        r.read_line(&mut line)?;
        line = line.trim().to_string();
        if line == "optimal" {
            status = SolverStatus::Solved;
        }

        // Objective value
        r.read_line(&mut line)?;

        // Results
        for l in r.lines() {
            line = l?;
            let mut iter = line.split_ascii_whitespace();
            iter.next();
            name = match iter.next() {
                Some(s) => s.to_string(),
                None => return Err(e)
            };
            value = match iter.next() {
                Some(s) => match s.parse() { Ok(f) => f, Err(_e) => return Err(e) },
                None => return Err(e)
            };
            mul = match iter.next() {
                Some(s) => match s.parse() { Ok(f) => f, Err(_e) => return Err(e) },
                None => return Err(e)
            };
            let mut name_iter = name.split('_');
            dtype = match name_iter.next() {
                Some(s) => s.to_string(),
                None => return Err(e)
            };
            index = match name_iter.next() {
                Some(s) => match s.parse() { Ok(n) => n, Err(_e) => return Err(e) },
                None => return Err(e)
            };

            // Variable
             if dtype == "x" {
                solution.x[index] = value;
                if mul > NumCast::from(0.).unwrap() {
                    solution.pi[index] = mul;
                }
                else {
                    solution.mu[index] = -mul;
                }
            }

            // Constraint
            else if dtype == "c" {
                solution.lam[index] = mul;
            }
            else {
                return Err(e);
            }
        }

        Ok((status, solution))
    }
}

impl<T: ProblemLp> Solver<T> for SolverClpCMD<T> {

    fn new() -> Self { 
        Self {
            status: SolverStatus::Unknown,
            solution: None,
        } 
    }

    fn status(&self) -> &SolverStatus { &self.status }
    fn solution(&self) -> &Option<ProblemSol<T>> { &self.solution }

    fn solve(&mut self, p: T) -> Result<(), SimpleError> {

        // Reset
        self.status = SolverStatus::Error;
        self.solution = None;
     
        // Input filename
        let input_file = Builder::new()
            .prefix("clp")
            .suffix(".lp")
            .tempfile();
        let input_filename = match input_file {
            Ok(f) => f.path().file_name().and_then(OsStr::to_str).unwrap().to_string(),
            Err(_e) => return Err(SimpleError::new("failed to create input filename")),
        };

        // Output filename
        let output_file = Builder::new()
            .prefix("clp")
            .suffix(".sol")
            .tempfile();
        let output_filename = match output_file {
            Ok(f) => f.path().file_name().and_then(OsStr::to_str).unwrap().to_string(),
            Err(_e) => return Err(SimpleError::new("failed to create output filename")),
        };

        // Write input file
        match p.write_to_lp_file(&input_filename) {
            Ok(()) => (),
            Err(_e) => {
                remove_file(&input_filename).ok();
                remove_file(&output_filename).ok();
                return Err(SimpleError::new("failed to write lp file"));
            }
        };

        // Call Clp command
        match Command::new("clp")
                      .args(&[&input_filename, 
                              "solve", 
                              "printingOptions",
                              "all",
                              "solution",
                              &output_filename])
                      .spawn()
                      .and_then(|mut cmd| cmd.wait())
                      .map(|ecode| assert!(ecode.success())) {
            Ok(()) => (),
            Err(_e) => {
                remove_file(&input_filename).ok();
                remove_file(&output_filename).ok();
                return Err(SimpleError::new("failed executing clp command"));
            }
        }
        
        // Clean up input file
        remove_file(&input_filename).ok();

        // Read output file
        let (status, solution) = match Self::read_sol_file(&output_filename, &p) {
            Ok((s, sol)) => (s, sol),
            Err(_e) => {
                remove_file(&output_filename).ok();
                return Err(SimpleError::new("failed to read clp solution file"))
            }
        };

        // Set status and solution
        self.status = status;
        self.solution = Some(solution);        

        // Clean up output file
        remove_file(&output_filename).ok();
        
        // All good
        Ok(())
    }
}