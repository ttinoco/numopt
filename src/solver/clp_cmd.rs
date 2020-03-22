use std::fs::File;
use std::ffi::OsStr;
use tempfile::Builder;
use std::io::prelude::*;
use std::fs::remove_file;
use std::process::Command;
use simple_error::SimpleError;
use std::io::{self, BufReader};

use crate::solver::{Solver, 
                    SolverStatus};
use crate::problem::{Problem,
                     ProblemDims,
                     ProblemLp, 
                     ProblemLpIO,
                     ProblemSol};

pub struct SolverClpCMD<T: ProblemLp> {
    status: SolverStatus,
    solution: Option<ProblemSol<T>>,
}

impl<T: ProblemLp> SolverClpCMD<T> {

    fn read_sol_file(filename: &str, p: &T) -> io::Result<(SolverStatus, ProblemSol<T>)> {
        
        let mut index: usize;
        let mut value: T::N;
        let mut status = SolverStatus::Unknown;
        let mut solution = ProblemSol::new(p.nx(),p.na());
        let f = File::open(filename)?;
        let mut r = BufReader::new(f);
        let mut line = String::new();
        let e = io::Error::new(io::ErrorKind::Other, "bad clp solution file");

        // Status
        r.read_line(&mut line)?;
        line = line.trim().to_string();

        println!("{}", line);
        println!("{}", line.len());
        println!("{}", line == "optimal");
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
            let name: String = match iter.next() {
                Some(s) => s.to_string(),
                None => return Err(e)
            };
            let value: T::N = match iter.next() {
                Some(s) => match s.parse() { Ok(f) => f, Err(_e) => return Err(e) },
                None => return Err(e)
            };
            let mul: T::N = match iter.next() {
                Some(s) => match s.parse() { Ok(f) => f, Err(_e) => return Err(e) },
                None => return Err(e)
            };
            let mut name_iter = name.split('_');
            let vartype: String = match name_iter.next() {
                Some(s) => s.to_string(),
                None => return Err(e)
            };
            let varindex: usize  = match name_iter.next() {
                Some(s) => match s.parse() { Ok(n) => n, Err(_e) => return Err(e) },
                None => return Err(e)
            };

            // Variable
             if vartype == "x" {
                println!("variable");

            }

            // Constraint
            else if vartype == "c" {
                println!("constraint");

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

    fn solve(&self, p: T) -> Result<(), SimpleError> {
     
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

        println!("{}", input_filename);
        println!("{}", output_filename);

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
                              "bar.sol"])
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
        let (status, solution) = match Self::read_sol_file("bar.sol", &p) {
            Ok((s, sol)) => (s, sol),
            Err(_e) => {
                remove_file(&output_filename).ok();
                return Err(SimpleError::new("failed to read clp solution file"))
            }
        };

        // Clean up output file
        remove_file(&output_filename).ok();
        
        // All good
        Ok(())
    }
}