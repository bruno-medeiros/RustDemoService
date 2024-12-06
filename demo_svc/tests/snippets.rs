mod lifetimes;

mod error_handling {
    #![allow(dead_code)]

    use std::convert::Infallible;
    pub fn unwrap_without_panic<T>(x: Result<T, Infallible>) -> T {
        let Ok(x) = x; // the `Err` case does not need to appear
        x
    }
}
