use std::ptr;
use simple_error::SimpleError;
use num_traits::{ToPrimitive};
use libc::{c_int, c_void, c_double};

use libipopt_sys as cipopt;

use crate::solver::{Solver, 
                   SolverStatus};
use crate::problem::{ProblemSol,
                     ProblemDims,
                     ProblemNlpBase};

pub struct SolverIpopt<T: ProblemNlpBase> {
    status: SolverStatus,
    solution: Option<ProblemSol<T::N>>,
}

impl<T: ProblemNlpBase + ProblemDims> Solver<T, T::N> for SolverIpopt<T> {

    fn new() -> Self { 
        Self {
            status: SolverStatus::Unknown,
            solution: None,
        } 
    }

    fn status(&self) -> &SolverStatus { &self.status }
    fn solution(&self) -> &Option<ProblemSol<T::N>> { &self.solution }

    fn solve(&mut self, p: &mut T) -> Result<(), SimpleError> {

        // Reset
        self.status = SolverStatus::Error;
        self.solution = None;

        let n: c_int = p.nx().to_i32().unwrap();
        let m: c_int = (p.na() + p.nf()).to_i32().unwrap();
        let l: Vec<f64> = p.l().iter().map(|x| x.to_f64().unwrap()).collect();
        let u: Vec<f64> = p.u().iter().map(|x| x.to_f64().unwrap()).collect();
        let glu: Vec<f64> = vec![0.; p.na()+p.nf()];
        let nnzj: c_int = (p.a().nnz() + p.j().nnz()).to_i32().unwrap();
        let nnzh: c_int = (p.hphi().nnz() + p.hcomb().nnz()).to_i32().unwrap();

        // Problem
        let cprob: cipopt::IpoptProblem = unsafe {
            cipopt::CreateIpoptProblem(n, 
                                       l.as_ptr(), 
                                       u.as_ptr(), 
                                       m, 
                                       glu.as_ptr(), 
                                       glu.as_ptr(), 
                                       nnzj, 
                                       nnzh, 
                                       0, 
                                       eval_f_cb::<T>, 
                                       eval_g_cb, 
                                       eval_grad_f_cb, 
                                       eval_jac_g_cb, 
                                       eval_h_cb)
        };
        if cprob.is_null() {
            return Err(SimpleError::new("failed to create ipopt problem"))
        }

        let mut x: Vec<f64> = vec![0.;p.nx()];
        let mut lamnu: Vec<f64> = vec![0.;p.na()+p.nf()];        
        let mut pi: Vec<f64> = vec![0.;p.nx()];
        let mut mu: Vec<f64> = vec![0.;p.nx()];

        // Solve
        let cstatus : c_int = unsafe {
            cipopt::IpoptSolve(cprob, 
                               x.as_mut_ptr(), 
                               ptr::null_mut(), 
                               ptr::null_mut(), 
                               lamnu.as_mut_ptr(), 
                               pi.as_mut_ptr(), 
                               mu.as_mut_ptr(), 
                               p as *mut _ as *mut c_void)
        };

        // Set status and solution

        Ok(())
    }
}

extern fn eval_f_cb<T: ProblemNlpBase>(n: c_int, 
                                       x: *const c_double, 
                                       new_x: c_int, 
                                       obj_value: *mut c_double, 
                                       user_data: *mut c_void) -> c_int {
    unsafe {
        let p: &mut T = &mut *(user_data as *mut T);
    };
    1
}

extern fn eval_grad_f_cb(n: c_int, 
                         x: *const c_double, 
                         new_x: c_int, 
                         grad_f: *mut c_double, 
                         user_data: *mut c_void) -> c_int {
    1 
}

extern fn eval_g_cb(n: c_int, 
                    x: *const c_double, 
                    new_x: c_int, 
                    m: c_int,
                    g: *mut c_double, 
                    user_data: *mut c_void) -> c_int {
    1
}

extern fn eval_jac_g_cb(n: c_int, 
                        x: *const c_double, 
                        new_x: c_int, 
                        m: c_int,
                        nele_jac: c_int,
                        irow: *mut c_int,
                        jcol: *mut c_int,
                        values: *mut c_double, 
                        user_data: *mut c_void) -> c_int {
    1
}

extern fn eval_h_cb(n: c_int, 
                    x: *const c_double, 
                    new_x: c_int,
                    obj_factor: c_double, 
                    m: c_int,
                    lambda: *const c_double,
                    new_lambda: c_int,
                    nele_hess: c_int,
                    irow: *mut c_int,
                    jcol: *mut c_int,
                    values: *mut c_double, 
                    user_data: *mut c_void) -> c_int {
    1
}