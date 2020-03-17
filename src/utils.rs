use num_traits::Float;

pub fn dot<T: Float>(x: &[T], y: &[T]) -> T {
    assert_eq!(x.len(), y.len());
    let mut p = T::from(0.).unwrap();
    for i in 0..x.len() {
        p = p + x[i]*y[i];
    }
    p
}