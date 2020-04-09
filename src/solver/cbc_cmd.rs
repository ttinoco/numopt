use std::fs::File;
use std::ffi::OsStr;
use tempfile::Builder;
use std::io::prelude::*;
use std::fs::remove_file;
use std::process::Command;
use std::marker::PhantomData;
use simple_error::SimpleError;
use std::io::{self, BufReader};
use std::collections::HashMap;

use crate::solver::{Solver, 
                    SolverParam,
                    SolverStatus};
use crate::problem::{ProblemSol,
                     ProblemMilpBase, 
                     ProblemMilpIO};

pub struct SolverCbcCmd<T: ProblemMilpBase> {
    status: SolverStatus,
    solution: Option<ProblemSol>,
    phantom: PhantomData<T>,
    parameters: HashMap<String, SolverParam>,
}

impl<T: ProblemMilpBase> SolverCbcCmd<T> {

    pub fn read_sol_file(fname: &str, p: &T, cbc: bool) -> io::Result<(SolverStatus, ProblemSol)> {
        
        let mut name: String;
        let mut dtype: String;
        let mut index: usize;
        let mut value: f64;
        let mut mul: f64;
        let mut status = SolverStatus::Unknown;
        let mut solution = ProblemSol::new(p.nx(), p.na(), 0);
        let f = File::open(fname)?;
        let mut r = BufReader::new(f);
        let mut line = String::new();
        let e = io::Error::new(io::ErrorKind::Other, "bad solution file");

        // Status
        r.read_line(&mut line)?;
        match line.split_ascii_whitespace().next() {
            Some(s) => {
                if !cbc && s == "optimal" {
                    status = SolverStatus::Solved;
                }
                else if cbc && s == "Optimal" {
                    status = SolverStatus::Solved;
                }
            },
            None => return Err(e)
        }

        // Objective value
        if !cbc {
            r.read_line(&mut line)?;
        }

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
                if mul > 0. {
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

impl<T: ProblemMilpBase + ProblemMilpIO>  Solver<T> for SolverCbcCmd<T> {

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
            .prefix("cbc")
            .suffix(".lp")
            .tempfile();
        let input_filename = match input_file {
            Ok(f) => f.path().file_name().and_then(OsStr::to_str).unwrap().to_string(),
            Err(_e) => return Err(SimpleError::new("failed to create input filename")),
        };

        // Output filename
        let output_file = Builder::new()
            .prefix("cbc")
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

        // Params
        let log_level= match self.get_param("logLevel") {
            Some(SolverParam::IntParam(i)) => i,
            _ => return Err(SimpleError::new("unable to get logLevel parameter"))
        };

        // Call Cbc command
        match Command::new("cbc")
                      .args(&[&input_filename,
                              "logLevel",
                              format!("{}", log_level).as_ref(),
                              "printingOptions",
                              "all", 
                              "solve", 
                              "solution",
                              &output_filename])
                      .spawn()
                      .and_then(|mut cmd| cmd.wait())
                      .map(|ecode| assert!(ecode.success())) {
            Ok(()) => (),
            Err(_e) => {
                remove_file(&input_filename).ok();
                remove_file(&output_filename).ok();
                return Err(SimpleError::new("failed executing cbc command"));
            }
        }
        
        // Clean up input file
        remove_file(&input_filename).ok();

        // Read output file
        let (status, solution) = match Self::read_sol_file(&output_filename, p, true) {
            Ok((s, sol)) => (s, sol),
            Err(_e) => {
                remove_file(&output_filename).ok();
                return Err(SimpleError::new("failed to read cbc solution file"))
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
    use crate::problem::{ProblemLp, ProblemMilp};
    use crate::solver::{Solver, SolverParam, SolverStatus, SolverCbcCmd};
    use crate::assert_vec_approx_eq;

    #[test]
    #[serial]
    fn cbc_solve_milp() {

        // Sample problem 
        // min        -x0 - x1 
        // subject to -2*x0 +  2*x1 + x2 == 1
        //            -8*x0 + 10*x1 + x3 ==  13
        //            x2 <= 0
        //            x3 >= 0
        //            x0 integer
        //            x1 integer

        let mut p = ProblemMilp::new(
            vec![-1.,-1., 0., 0.],
            CooMat::new(
                (2, 4),
                vec![0,0,0,1,1,1],
                vec![0,1,2,0,1,3],
                vec![-2.,2.,1.,-8.,10.,1.]),
            vec![1.,13.],
            vec![-1e8,-1e8,-1e8,0.],
            vec![1e8,1e8,0.,1e8],
            vec![true, true, false, false],
            None,
        );

        let mut s = SolverCbcCmd::new(&p);
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
        s.solve(&mut p).unwrap();

        assert_eq!(*s.status(), SolverStatus::Solved);
        assert!(s.solution().is_some());
        assert_vec_approx_eq!(s.solution().as_ref().unwrap().x, 
                              &vec![1., 2., -1., 1.0], 
                              epsilon=1e-8);
    }

    #[test]
    #[serial]
    fn cbc_solve_lp() {

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

        let mut s = SolverCbcCmd::new(&p);
        s.set_param("logLevel", SolverParam::IntParam(0)).unwrap();
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