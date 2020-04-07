use std::ffi::OsStr;
use tempfile::Builder;
use std::fs::remove_file;
use std::process::Command;
use std::marker::PhantomData;
use simple_error::SimpleError;
use std::collections::HashMap;

use crate::solver::{Solver, 
                    SolverParam,
                    SolverStatus,
                    SolverCbcCmd};
use crate::problem::{ProblemSol,
                     ProblemLpBase, 
                     ProblemMilpIO};

pub struct SolverClpCmd<T> {
    status: SolverStatus,
    solution: Option<ProblemSol>,
    phantom: PhantomData<T>,
    parameters: HashMap<String, SolverParam>,
}

impl<T: ProblemLpBase + ProblemMilpIO> Solver<T> for SolverClpCmd<T> {

    fn new(_p: &T) -> Self { 

        let mut parameters: HashMap<String, SolverParam> = HashMap::new();
        parameters.insert("logLevel".to_string(), SolverParam::IntParam(1));

        Self {
            status: SolverStatus::Unknown,
            solution: None,
            phantom: PhantomData,
            parameters: parameters,
        } 
    }

    fn get_params(&self) -> &HashMap<String, SolverParam> { &self.parameters }
    fn get_params_mut(&mut self) -> &mut HashMap<String, SolverParam> { &mut self.parameters }
    fn status(&self) -> &SolverStatus { &self.status }
    fn solution(&self) -> &Option<ProblemSol> { &self.solution }

    fn solve(&mut self, p: &mut T) -> Result<(), SimpleError> {

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
        let (status, solution) = match SolverCbcCmd::read_sol_file(&output_filename, 
                                                                   p.base(), 
                                                                   false) {
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

#[cfg(test)]
mod tests {

    use serial_test::serial;

    use crate::matrix::CooMat;
    use crate::problem::ProblemLp;
    use crate::solver::{Solver, SolverStatus, SolverClpCmd};
    use crate::assert_vec_approx_eq;

    #[test]
    #[serial]
    fn clp_solve_lp() {

        // Sample problem 
        // min        180*x0 + 160*x1 
        // subject to 6*x0 +   x1 + x2 == 12
        //            3*x0 +   x1 + x3 ==  8
        //            4*x0 + 6*x1 + x4 == 24
        //            0 <= x0 <= 5
        //            0 <= x1 <= 5
        //            x2 <= 0
        //            x3 <= 0
        //            x4 <= 0

        let mut p = ProblemLp::new(
            vec![180.,160., 0., 0., 0.],
            CooMat::new(
                (3, 5),
                vec![0,0,0,1,1,1,2,2,2],
                vec![0,1,2,0,1,3,0,1,4],
                vec![6.,1.,1.,3.,1.,1.,4.,6.,1.]),
            vec![12.,8.,24.],
            vec![0.,0.,-1e8,-1e8,-1e8],
            vec![5.,5.,0.,0.,0.],
            None,
        );

        let mut s = SolverClpCmd::new(&p);
        s.solve(&mut p).unwrap();

        assert_eq!(*s.status(), SolverStatus::Solved);
        assert!(s.solution().is_some());
        assert_vec_approx_eq!(s.solution().as_ref().unwrap().x, 
                              &vec![1.7142857, 2.8571429, -1.1428571, 0., 0.], 
                              epsilon=1e-8);
        assert_vec_approx_eq!(s.solution().as_ref().unwrap().lam, 
                              &vec![0., 31.428571, 21.428571], 
                              epsilon=1e-8);
        assert_vec_approx_eq!(s.solution().as_ref().unwrap().mu, 
                              &vec![1.4210855e-14, 0., 0., 3.1428571e+01, 2.1428571e+01], 
                              epsilon=1e-8);
        assert_vec_approx_eq!(s.solution().as_ref().unwrap().pi, 
                              &vec![0.;5], 
                              epsilon=1e-8);

    }
}