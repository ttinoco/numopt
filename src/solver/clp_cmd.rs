use std::ffi::OsStr;
use tempfile::Builder;
use std::fs::remove_file;
use std::process::Command;
use simple_error::SimpleError;

use crate::solver::{Solver, SolverStatus};
use crate::problem::{ProblemLp, 
                     ProblemLpIO,
                     ProblemSol};

pub struct SolverClpCMD<T: ProblemLp> {
    status: SolverStatus,
    solution: Option<ProblemSol<T>>,
}

impl<T: ProblemLp> SolverClpCMD<T> {

    fn read_sol_file(filename: &str) -> Result<(SolverStatus, ProblemSol<T>), SimpleError> {
        
        let status = SolverStatus::Unknown;
        let solution = ProblemSol::new(0,0);

        

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
        let (status, solution) = match Self::read_sol_file(&output_filename) {
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