use optrs::Problem;
use sprs::{TriMat, TriMatBase};

fn main () {

    println!("optrs example problem");

    // Sample problem
    // min        180*x1 + 160*x2 
    // subject to 6*x1 +   x2 + x3 == 12
    //            3*x1 +   x2 + x4 ==  8
    //            4*x1 + 6*x2 + x5 == 24
    //            0 <= x1 <= 5
    //            0 <= x2 <= 5
    //            x3 <= 0
    //            x4 <= 0
    //            x5 <= 0

    struct P {
        x: Vec<f64>,
        phi: f64,
        gphi: Vec<f64>,
        a: TriMat<f64>,
        b: Vec<f64>,
        l: Vec<f64>,
        u: Vec<f64>,
    };

    impl Problem for P {
        type N = f64;
        fn x(&self) -> &[f64] { &self.x }
        fn phi(&self) -> f64 { self.phi }
        fn gphi(&self) -> &[f64] { &self.gphi }
        fn a(&self) -> &TriMat<f64> { &self.a }
        fn b(&self) -> &[f64] { &self.b }
        fn l(&self) -> &[f64] { &self.l }
        fn u(&self) -> &[f64] { &self.u }
        fn eval(&mut self, x: &[f64]) -> () {
            self.setx(x);
            self.phi = 180.*x[0] + 160.*x[1];
            self.gphi[0] = 180.;
            self.gphi[1] = 160.;
        }
        fn setx(&mut self, x: &[f64]) -> () {
            self.x = x.to_vec();
        }
    }

    let mut p = P {
        x: vec![0.,0.,0.,0.,0.],
        phi: 0.,
        gphi: vec![0., 0., 0., 0., 0.],
        a: TriMatBase::from_triplets(
            (3, 5),
            vec![0,0,0,1,1,1,1,1,1],
            vec![0,1,2,0,1,3,0,1,4],
            vec![6.,1.,1.,3.,1.,1.,4.,6.,1.]),
        b: vec![12.,8.,24.],
        l: vec![0.,0.,-1e8,-1e8,-1e8],
        u: vec![5.,5.,0.,0.,0.],
    };

    let x = vec![0.5, 2., 1., 2., 3.];

    p.eval(&x);
    
    println!("x = {:?}", p.x());
    println!("phi = {}", p.phi());
    println!("gphi = {:?}", p.gphi());
    println!("a = {:?}", p.a());
    println!("b = {:?}", p.b());
    println!("l = {:?}", p.l());
    println!("u = {:?}", p.u());
}
