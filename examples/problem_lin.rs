use optcore;
use optcore::Problem;
use sprs::{TriMat, TriMatBase};

fn main () {

    println!("optcore example problem 1");

    // Sample linear problem
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
        phi: f64,
        gphi: Vec<f64>,
        a: TriMat<f64>
    };

    impl Problem<f64> for P {
        fn phi(&self) -> f64 { self.phi }
        fn gphi(&self) -> &Vec<f64> { &self.gphi }
        fn a(&self) -> Option<&TriMat<f64>> { Some(&self.a) }
        fn eval(&mut self, x: &Vec<f64>) -> () {
            self.phi = 180.*x[0] + 160.*x[1];
            self.gphi[0] = 180.;
            self.gphi[1] = 160.;
        }
    }

    let mut p = P {
        phi: 0.,
        gphi: vec![0., 0., 0., 0., 0.],
        a: TriMatBase::from_triplets(
            (3, 5),
            vec![0,0,0,1,1,1,1,1,1],
            vec![0,1,2,0,1,3,0,1,4],
            vec![6.,1.,1.,3.,1.,1.,4.,6.,1.])
    };

    let x = vec![0.5, 2., 1., 2., 3.];

    p.eval(&x);
    
    println!("phi = {}", p.phi());
    println!("gphi = {:?}", p.gphi());
    println!("a = {:?}", p.a());
}
