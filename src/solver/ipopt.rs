use std::ptr;
use std::marker::PhantomData;
use simple_error::SimpleError;
use num_traits::cast::ToPrimitive;
use libc::{c_int, c_void, c_double};

use libipopt_sys as cipopt;

use crate::solver::{Solver, 
                    SolverStatus};
use crate::problem::{ProblemSol,
                     ProblemNlpBase};

pub struct SolverIpopt<T> {
    status: SolverStatus,
    solution: Option<ProblemSol>,
    phantom: PhantomData<T>,
}

impl<T: ProblemNlpBase> Solver<T> for SolverIpopt<T> {

    fn new(_p: &T) -> Self { 
        Self {
            status: SolverStatus::Unknown,
            solution: None,
            phantom: PhantomData,
        } 
    }

    fn status(&self) -> &SolverStatus { &self.status }
    fn solution(&self) -> &Option<ProblemSol> { &self.solution }

    fn solve(&mut self, p: &mut T) -> Result<(), SimpleError> {

        // Reset
        self.status = SolverStatus::Error;
        self.solution = None;

        let n: c_int = p.nx().to_i32().unwrap();
        let m: c_int = (p.na() + p.nf()).to_i32().unwrap();
        let glu: Vec<f64> = vec![0.; p.na()+p.nf()];
        let nnzj: c_int = (p.a().nnz() + p.j().nnz()).to_i32().unwrap();
        let nnzh: c_int = (p.hphi().nnz() + p.hcomb().nnz()).to_i32().unwrap();

        // Problem
        let cprob: cipopt::IpoptProblem = unsafe {
            cipopt::CreateIpoptProblem(n, 
                                       p.l().as_ptr(), 
                                       p.u().as_ptr(), 
                                       m, 
                                       glu.as_ptr(), 
                                       glu.as_ptr(), 
                                       nnzj, 
                                       nnzh, 
                                       0, 
                                       eval_f_cb::<T>, 
                                       eval_g_cb::<T>, 
                                       eval_grad_f_cb::<T>, 
                                       eval_jac_g_cb::<T>, 
                                       eval_h_cb::<T>)
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

        // Clean up
        unsafe {
            cipopt::FreeIpoptProblem(cprob);
        };

        Ok(())
    }
}

extern fn eval_f_cb<T>(n: c_int, 
                       x: *const c_double, 
                       new_x: c_int, 
                       obj_value: *mut c_double, 
                       user_data: *mut c_void) -> c_int 
where T: ProblemNlpBase {
    if x.is_null() || obj_value.is_null() || user_data.is_null() {
        return cipopt::FALSE;
    }
    unsafe {
        let p: &mut T = &mut *(user_data as *mut T);
        match n.to_usize() {
            Some(nn) => { if nn != p.nx() { return cipopt::FALSE; } },
            None => return cipopt::FALSE,
        }
        let xx = Vec::from_raw_parts(x as *mut c_double, p.nx(), p.nx());
        if new_x == 1 {
            p.evaluate(&xx);
        }
        *obj_value = p.phi();
    };
    cipopt::TRUE
}

extern fn eval_grad_f_cb<T>(n: c_int, 
                            x: *const c_double, 
                            new_x: c_int, 
                            grad_f: *mut c_double, 
                            user_data: *mut c_void) -> c_int 
where T: ProblemNlpBase {
    if x.is_null() || grad_f.is_null() || user_data.is_null() {
        return cipopt::FALSE;
    }
    unsafe {
        let p: &mut T = &mut *(user_data as *mut T);
        match n.to_usize() {
            Some(nn) => { if nn != p.nx() { return cipopt::FALSE; } },
            None => return cipopt::FALSE,
        }
        let xx = Vec::from_raw_parts(x as *mut c_double, p.nx(), p.nx());
        if new_x == 1 {
            p.evaluate(&xx);
        }
        ptr::copy(p.gphi().as_ptr(), grad_f, p.nx());
    };
    cipopt::TRUE
}

extern fn eval_g_cb<T>(n: c_int, 
                       x: *const c_double, 
                       new_x: c_int, 
                       m: c_int,
                       g: *mut c_double, 
                       user_data: *mut c_void) -> c_int 
where T: ProblemNlpBase {
    if x.is_null() || g.is_null() || user_data.is_null() {
        return cipopt::FALSE;
    }
    unsafe {
        let p: &mut T = &mut *(user_data as *mut T);
        match n.to_usize() {
            Some(nn) => { if nn != p.nx() { return cipopt::FALSE; } },
            None => return cipopt::FALSE,
        }
        match m.to_usize() {
            Some(mm) => { if mm != p.na()+p.nf() { return cipopt::FALSE; } },
            None => return cipopt::FALSE,
        }
        let xx = Vec::from_raw_parts(x as *mut c_double, p.nx(), p.nx());
        if new_x == 1 {
            p.evaluate(&xx);
        }
        let ax = p.a()*xx;
        ptr::copy(ax.as_ptr(), g, p.na());
        ptr::copy(p.f().as_ptr(), g.add(p.na()), p.nf());
    };
    cipopt::TRUE
}

extern fn eval_jac_g_cb<T>(n: c_int, 
                           x: *const c_double, 
                           new_x: c_int, 
                           m: c_int,
                           nele_jac: c_int,
                           irow: *mut c_int,
                           jcol: *mut c_int,
                           values: *mut c_double, 
                           user_data: *mut c_void) -> c_int 
where T: ProblemNlpBase {
    if x.is_null() || irow.is_null() || jcol.is_null() || values.is_null() || user_data.is_null() {
        return cipopt::FALSE;
    }
    unsafe {
        let p: &mut T = &mut *(user_data as *mut T);
        match n.to_usize() {
            Some(nn) => { if nn != p.nx() { return cipopt::FALSE; } },
            None => return cipopt::FALSE,
        }
        match m.to_usize() {
            Some(mm) => { if mm != p.na()+p.nf() { return cipopt::FALSE; } },
            None => return cipopt::FALSE,
        }
        match nele_jac.to_usize() {
            Some(nnz) => { if nnz != p.a().nnz()+p.j().nnz() { return cipopt::FALSE; } },
            None => return cipopt::FALSE,
        }
        if x.is_null() {
            let mut k: usize = 0;
            for (row, col, _val) in p.a().iter() {
                *irow.add(k) = row.to_i32().unwrap();
                *jcol.add(k) = col.to_i32().unwrap();
                k += 1;
            }
            for (row, col, _val) in p.j().iter() {
                *irow.add(k) = row.to_i32().unwrap();
                *jcol.add(k) = col.to_i32().unwrap();
                k += 1;
            }
        }
        else {
            let xx = Vec::from_raw_parts(x as *mut c_double, p.nx(), p.nx());
            if new_x == 1 {
                p.evaluate(&xx);
            }
            let mut k: usize = 0;
            for (_row, _col, val) in p.a().iter() {
                *values.add(k) = val; 
                k += 1;
            }
            for (_row, _col, val) in p.j().iter() {
                *values.add(k) = val; 
                k += 1;
            }   
        }
    };
    cipopt::TRUE
}

extern fn eval_h_cb<T>(n: c_int, 
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
                       user_data: *mut c_void) -> c_int 
where T: ProblemNlpBase {
    unsafe {
        let p: &mut T = &mut *(user_data as *mut T);
    };
    cipopt::TRUE
}