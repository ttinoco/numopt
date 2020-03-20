use tempfile::Builder;
use std::ffi::OsStr;
use simple_error::SimpleError;

use crate::solver::Solver;
use crate::problem::{ProblemLp, ProblemLpWriter};

pub struct SolverClpCMD {

}

impl SolverClpCMD {
    pub fn new() -> Self { Self{} }
}

impl<T: ProblemLp> Solver<T> for SolverClpCMD {

    fn solve(&self, p: T) -> Result<(), SimpleError> {

        println!("Testing SolverClpCMD solve");
     
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
        p.write_to_lp_file(&input_filename)?;

        // Call Clp command
        
        // Read output file

        
        // All good
        Ok(())
    }
}
