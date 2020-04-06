pub use approx;
use num_traits::Float;

#[macro_export]
macro_rules! assert_vec_approx_eq {
    ($x:expr, $y:expr, epsilon = $eps:expr) => {
        assert_eq!($x.len(), $y.len());
        for (a,b) in $x.iter().zip($y.iter()) {
            approx::assert_abs_diff_eq!(a, b, epsilon = $eps);
        }
    };
}

pub fn dot<T: Float>(x: &[T], y: &[T]) -> T {
    assert_eq!(x.len(), y.len());
    let mut p = T::from(0.).unwrap();
    for i in 0..x.len() {
        p = p + x[i]*y[i];
    }
    p
}