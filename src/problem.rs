pub trait Problem {
    fn phi(&self) -> f64;
    fn gphi(&self) -> &Vec<f64>;
    fn eval(&mut self, x: &Vec<f64>) -> ();
}
