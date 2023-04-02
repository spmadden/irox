#[macro_export]
macro_rules! assert_eq_eps {
    ($left:expr, $right:expr, $eps:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let delta = (*left_val - *right_val).abs();
                if !(delta <= $eps) {
                    panic!(
                        "Assertion failed, {} - {} = {} > {}",
                        &*left_val, &*right_val, delta, $eps
                    )
                }
            }
        }
    };
}
