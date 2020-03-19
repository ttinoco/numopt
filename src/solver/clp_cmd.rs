use tempfile::Builder;
use std::ffi::OsStr;

use crate::solver::Solver;
use crate::problem::ProblemLp;

pub struct SolverClpCMD {

}

impl SolverClpCMD {
    pub fn new() -> Self { Self{} }
}

impl<T: ProblemLp> Solver<T> for SolverClpCMD {

    fn solve(&self, p: T) -> () {

        println!("Testing SolverClpCMD solve");
       
        // Create input input file
        let input_file = Builder::new()
            .prefix("clp")
            .suffix(".lp")
            .tempfile();
        let input_filename = match input_file {
            Ok(f) => f.path().file_name().and_then(OsStr::to_str).unwrap().to_string(),
            Err(e) => return (),
        };

        // Create output file
        let output_file = Builder::new()
            .prefix("clp")
            .suffix(".sol")
            .tempfile();
        let output_filename = match output_file {
            Ok(f) => f.path().file_name().and_then(OsStr::to_str).unwrap().to_string(),
            Err(e) => return (),
        };

        println!("{}", input_filename);
        println!("{}", output_filename);

        // Write input file

        // Call Clp command
        
        // Read output file

        

    }
}
