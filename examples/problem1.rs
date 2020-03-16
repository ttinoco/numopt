use optcore;
use optcore::Problem;

fn main () {

    println!("optcore example problem 1");

    // Sample problem 1
    // min 2*x + 3*y + 1
    struct P1 {
        phi: f64,
        gphi: Vec<f64>,
    }

    impl Problem for P1 {
        fn phi(&self) -> f64 { self.phi }
        fn gphi(&self) -> &Vec<f64> { &self.gphi }
        fn eval(&mut self, x: &Vec<f64>) -> () {
            self.phi = 2.*x[0] + 3.*x[1] + 1.;
            self.gphi[0] = 2.;
            self.gphi[1] = 3.;
        }
    }

    let mut p = P1 {
        phi: 0.,
        gphi: vec![0., 0.],
    };

    let x = vec![3., 4.];

    p.eval(&x);
    
    println!("P1 phi = {}", p.phi());
    println!("P1 gphi = {:?}", p.gphi());
}
