use std::ops::{Add, Sub};
// We use this in the doc strings.
#[allow(unused_imports)]
use super::Histogram;

/// Render configuration specific to [`Histogram`]s.
///
/// ### Example
/// ```
/// # use flat::HistogramConfig;
/// let histogram_config = HistogramConfig {
///     ..HistogramConfig::default()
/// };
/// ```
#[derive(Debug, Default)]
pub struct HistogramConfig {}

/// Allows a type `T` to be used as the *primary* dimension of a [`Histogram`].
/// Consumers may choose to implement this to bin non-standard types in a histogram.
pub trait Binnable: PartialEq + PartialOrd + Add + Sub + Sized {
    /// Multiply this value (`self: T`) by the `rhs: usize`, resulting in another `T`.
    ///
    /// **Notice**: For *whole* types `T` (types that belong to ℤ) the resulting value **must** be rounded up.
    /// In other words, implement `multiply` using `ceil` for integer types.
    fn multiply(&self, rhs: usize) -> Self;

    /// Divide this value (`self: T`) by the `rhs: usize`, resulting in another `T`.
    ///
    /// **Notice**: For *whole* types `T` (types that belong to ℤ) the resulting value **must** be rounded up.
    /// In other words, implement `divide` using `ceil` for integer types.
    fn divide(&self, rhs: usize) -> Self;
}

impl Binnable for f64 {
    fn multiply(&self, rhs: usize) -> Self {
        self * (rhs as f64)
    }

    fn divide(&self, rhs: usize) -> Self {
        self / (rhs as f64)
    }
}

impl Binnable for i64 {
    fn multiply(&self, rhs: usize) -> Self {
        (*self as f64 * (rhs as f64)).ceil() as i64
    }

    fn divide(&self, rhs: usize) -> Self {
        (*self as f64 / (rhs as f64)).ceil() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binnable_f64() {
        assert_eq!(0.0f64.multiply(0), 0.0);
        assert_eq!(1.0f64.multiply(0), 0.0);
        assert_eq!(1.5f64.multiply(0), 0.0);

        assert_eq!(0.0f64.multiply(1), 0.0);
        assert_eq!(1.0f64.multiply(1), 1.0);
        assert_eq!(1.5f64.multiply(1), 1.5);

        assert_eq!(0.0f64.divide(1), 0.0);
        assert_eq!(1.0f64.divide(1), 1.0);
        assert_eq!(1.5f64.divide(1), 1.5);

        assert_eq!(0.0f64.divide(2), 0.0);
        assert_eq!(1.0f64.divide(2), 0.5);
        assert_eq!(1.5f64.divide(2), 0.75);
    }

    #[test]
    fn binnable_i64() {
        assert_eq!(0i64.multiply(0), 0);
        assert_eq!(1i64.multiply(0), 0);
        assert_eq!(2i64.multiply(0), 0);

        assert_eq!(0i64.multiply(1), 0);
        assert_eq!(1i64.multiply(1), 1);
        assert_eq!(2i64.multiply(1), 2);

        assert_eq!(0i64.divide(1), 0);
        assert_eq!(1i64.divide(1), 1);
        assert_eq!(2i64.divide(1), 2);

        assert_eq!(0i64.divide(2), 0);
        assert_eq!(1i64.divide(2), 1);
        assert_eq!(2i64.divide(2), 1);
    }
}
