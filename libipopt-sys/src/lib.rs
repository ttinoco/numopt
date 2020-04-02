#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::convert::TryInto;
use libc::{c_int,
           c_double};

pub type IpoptEvalF = Box<dyn Fn(usize, &[f64], bool, &mut f64) -> bool>;
pub type IpoptEvalGradF = Box<dyn Fn(usize, &[f64], bool, &mut Vec<f64>) -> bool>;
pub type IpoptEvalG = Box<dyn Fn() -> bool>;
pub type IpoptEvalJacG = Box<dyn Fn() -> bool>;
pub type IpoptEvalH = Box<dyn Fn() -> bool>;

pub struct IpoptContext {
    n: usize,
    m: usize,
    nnzj: usize,
    nnzh: usize,
    l: Vec<f64>,
    u: Vec<f64>,
    gl: Vec<f64>,
    gu: Vec<f64>,
    eval_f: IpoptEvalF,
    eval_grad_f: IpoptEvalGradF,
    eval_g: IpoptEvalG,
    eval_jac_g: IpoptEvalJacG,
    eval_h: IpoptEvalH,
    p: IpoptProblem,
}

impl IpoptContext {

    pub fn new(n: usize,
               m: usize,
               nnzj: usize,
               nnzh: usize,
               l: Vec<f64>,
               u: Vec<f64>,
               gl: Vec<f64>,
               gu: Vec<f64>,
               eval_f: IpoptEvalF,
               eval_grad_f: IpoptEvalGradF,
               eval_g: IpoptEvalG,
               eval_jac_g: IpoptEvalJacG,
               eval_h: IpoptEvalH) -> Self {

        // a bunch of asserts

        // problem
        let p: IpoptProblem = unsafe {
            CreateIpoptProblem(n.try_into().unwrap(), 
                               l.as_ptr(), 
                               l.as_ptr(), 
                               m.try_into().unwrap(), 
                               gl.as_ptr(), 
                               gu.as_ptr(), 
                               nnzj.try_into().unwrap(), 
                               nnzh.try_into().unwrap(), 
                               0, 
                               eval_f_cb, 
                               eval_g_cb, 
                               eval_grad_f_cb, 
                               eval_jac_g_cb, 
                               eval_h_cb)
        };

        Self {
            n: n,
            m: m,
            nnzj: nnzj,
            nnzh: nnzh,
            l: l,
            u: u,
            gl: gl,
            gu: gu,
            eval_f: eval_f,
            eval_grad_f: eval_grad_f,
            eval_g: eval_g,
            eval_jac_g: eval_jac_g,
            eval_h: eval_h,
            p: p,
        }
    }
}

extern fn eval_f_cb(n: c_int, 
                    x: *mut c_double, 
                    new_x: c_int, 
                    obj_value: *mut c_double, 
                    user_data: *mut IpoptContext) -> c_int {

    unsafe {
        let nn = (*user_data).n;
        match ((*user_data).eval_f)(nn, 
                                    &Vec::from_raw_parts(x, nn, nn), 
                                    new_x == 1, 
                                    &mut (*obj_value)) {
            true => 0,
            false => 1
        }
    }
}

extern fn eval_grad_f_cb(n: c_int, 
                         x: *mut c_double, 
                         new_x: c_int, 
                         grad_f: *mut c_double, 
                         user_data: *mut IpoptContext) -> c_int {
    unsafe {
        let nn: usize = (*user_data).n;
        match ((*user_data).eval_grad_f)(nn, 
                                        &Vec::from_raw_parts(x, nn, nn), 
                                        new_x == 1, 
                                        &mut Vec::from_raw_parts(grad_f, nn, nn)) {
            true => 0,
            false => 1
        }
    }   
}

extern fn eval_g_cb(n: c_int, 
                    x: *const c_double, 
                    new_x: c_int, 
                    m: c_int,
                    g: *mut c_double, 
                    user_data: *mut IpoptContext) -> c_int {

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
                        user_data: *mut IpoptContext) -> c_int {

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
                    user_data: *mut IpoptContext) -> c_int {

    1
}           
           
#[repr(C)] struct IpoptProblemInfo { _private: [u8; 0] }

type IpoptProblem = *mut IpoptProblemInfo;

type Eval_F_CB = extern fn(c_int, 
                           *mut c_double, 
                           c_int, 
                           *mut c_double, 
                           *mut IpoptContext) -> c_int;

type Eval_Grad_F_CB = extern fn(c_int, 
                                *mut c_double, 
                                c_int,
                                *mut c_double, 
                                *mut IpoptContext) -> c_int;

type Eval_G_CB = extern fn(c_int, 
                           *const c_double, 
                           c_int,
                           c_int, 
                           *mut c_double, 
                           *mut IpoptContext) -> c_int;

type Eval_Jac_G_CB = extern fn(c_int,
                               *const c_double, 
                               c_int,
                               c_int, 
                               c_int,
                               *mut c_int, 
                               *mut c_int, 
                               *mut c_double,
                               *mut IpoptContext) -> c_int;

type Eval_H_CB = extern fn(c_int, 
                           *const c_double, 
                           c_int, 
                           c_double,
                           c_int, 
                           *const c_double, 
                           c_int,
                           c_int,
                           *mut c_int, 
                           *mut c_int, 
                           *mut c_double,
                           *mut IpoptContext) -> c_int;

#[link(name = "ipopt")]
extern {

    fn CreateIpoptProblem(n: c_int,
                              x_L: *const c_double,
                              x_U: *const c_double,
                              m: c_int,
                              g_L: *const c_double,
                              g_U: *const c_double,
                              nele_jac: c_int,
                              nele_hess: c_int,
                              index_style: c_int,
                              eval_f: Eval_F_CB,
                              eval_g: Eval_G_CB,
                              eval_grad_f: Eval_Grad_F_CB,
                              eval_jac_g: Eval_Jac_G_CB,
                              eval_h: Eval_H_CB
                             ) -> IpoptProblem;

    fn FreeIpoptProblem(ipopt_problem: IpoptProblem) -> ();

}


