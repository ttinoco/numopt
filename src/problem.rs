
use sprs::TriMat;
use num_traits::Float; 

pub trait Problem<N: Float> {

    fn phi(&self) -> N;
    fn gphi(&self) -> &Vec<N>;
    fn a(&self) -> Option<&TriMat<N>> { None }
    fn eval(&mut self, x: &Vec<N>) -> ();
}
