mod cipopt;

use libc::c_void;

pub type IpoptEvalF = Box<dyn Fn() -> bool>;
pub type IpoptEvalGradF = Box<dyn Fn() -> bool>;
pub type IpoptEvalG = Box<dyn Fn() -> bool>;
pub type IpoptEvalJacG = Box<dyn Fn() -> bool>;
pub type IpoptEvalH = Box<dyn Fn() -> bool>;

pub struct IpoptContext {
    n: i32,
    m: i32,
    nnzj: i32,
    nnzh: i32,
    l: Vec<f64>,
    u: Vec<f64>,
    gl: Vec<f64>,
    gu: Vec<f64>,
    eval_f: IpoptEvalF,
    eval_grad_f: IpoptEvalGradF,
    eval_g: IpoptEvalG,
    eval_jac_g: IpoptEvalJacG,
    eval_h: IpoptEvalH,
    p: cipopt::IpoptProblem,
}

impl IpoptContext {

    pub fn new(n: i32,
               m: i32,
               nnzj: i32,
               nnzh: i32,
               l: Vec<f64>,
               u: Vec<f64>,
               gl: Vec<f64>,
               gu: Vec<f64>,
               eval_f: IpoptEvalF,
               eval_grad_f: IpoptEvalGradF,
               eval_g: IpoptEvalG,
               eval_jac_g: IpoptEvalJacG,
               eval_h: IpoptEvalH) -> () {


        // a bunch of asserts

        // problem
        let p: cipopt::IpoptProblem = unsafe {
            cipopt::CreateIpoptProblem(n, 
                                       l.as_ptr(), 
                                       l.as_ptr(), 
                                       m, 
                                       gl.as_ptr(), 
                                       gu.as_ptr(), 
                                       nnzj, 
                                       nnzh, 
                                       0, 
                                       eval_f_cb, 
                                       eval_g_cb, 
                                       eval_grad_f_cb, 
                                       eval_jac_g_cb, 
                                       eval_h_cb)
        };

        //Self {
        //}
    }
}

extern fn eval_f_cb(n: i32, 
                           x: *const f64, 
                           new_x: i32, 
                           obj_value: *mut f64, 
                           user_data: *mut c_void) -> i32 {

    1
}

extern fn eval_grad_f_cb (n: i32, 
                          x: *const f64, 
                          new_x: i32, 
                          grad_f: *mut f64, 
                          user_data: *mut c_void) -> i32 {

    1
}

extern fn eval_g_cb (n: i32, 
                     x: *const f64, 
                     new_x: i32, 
                     m: i32,
                     g: *mut f64, 
                     user_data: *mut c_void) -> i32 {

    1
}

extern fn eval_jac_g_cb (n: i32, 
                         x: *const f64, 
                         new_x: i32, 
                         m: i32,
                         nele_jac: i32,
                         irow: *mut i32,
                         jcol: *mut i32,
                         values: *mut f64, 
                         user_data: *mut c_void) -> i32 {

    1
}

extern fn eval_h_cb (n: i32, 
                     x: *const f64, 
                     new_x: i32,
                     obj_factor: f64, 
                     m: i32,
                     lambda: *const f64,
                     new_lambda: i32,
                     nele_hess: i32,
                     irow: *mut i32,
                     jcol: *mut i32,
                     values: *mut f64, 
                     user_data: *mut c_void) -> i32 {

    1
}


