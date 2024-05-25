use crate::{Nothing, Schema1, Schema2Breakdown2};
use std::ops::{Add, Sub};

#[derive(Debug, Default)]
pub struct HistogramConfig {}

/// The internal trait which allows rendering [`Histogram`]s across different [`Schema']s.
/// Consumers should not implement this trait.
pub trait HistogramSchematic {
    type Dimensions;
    type PrimaryDimension: Binnable;
    type BreakdownDimension;

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension;

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension;

    fn primary_header(&self) -> String;

    fn breakdown_header(&self) -> Option<String>;

    fn is_breakdown(&self) -> bool;
}

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

impl Binnable for u64 {
    fn multiply(&self, rhs: usize) -> Self {
        (*self as f64 * (rhs as f64)).ceil() as u64
    }

    fn divide(&self, rhs: usize) -> Self {
        (*self as f64 / (rhs as f64)).ceil() as u64
    }
}

impl<T> HistogramSchematic for Schema1<T>
where
    T: Clone + PartialEq + PartialOrd + Add + Sub + Binnable,
{
    type Dimensions = (T,);
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, _dims: &Self::Dimensions) -> Self::BreakdownDimension {
        Nothing
    }

    fn primary_header(&self) -> String {
        self.column_1.clone()
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

impl<T, U> HistogramSchematic for Schema2Breakdown2<T, U>
where
    T: Clone + Binnable,
    U: Clone,
{
    type Dimensions = (T, U);
    type PrimaryDimension = T;
    type BreakdownDimension = U;

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension {
        dims.1.clone()
    }

    fn primary_header(&self) -> String {
        self.column_1.clone()
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_2.clone())
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Schema;

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
    fn binnable_u64() {
        assert_eq!(0u64.multiply(0), 0);
        assert_eq!(1u64.multiply(0), 0);
        assert_eq!(2u64.multiply(0), 0);

        assert_eq!(0u64.multiply(1), 0);
        assert_eq!(1u64.multiply(1), 1);
        assert_eq!(2u64.multiply(1), 2);

        assert_eq!(0u64.divide(1), 0);
        assert_eq!(1u64.divide(1), 1);
        assert_eq!(2u64.divide(1), 2);

        assert_eq!(0u64.divide(2), 0);
        assert_eq!(1u64.divide(2), 1);
        assert_eq!(2u64.divide(2), 1);
    }

    #[test]
    fn schema1_impl_trait() {
        let schema = Schema::one("abc");
        assert_eq!(schema.primary_dim(&(1u64,)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64,)), Nothing);
        assert_eq!(schema.primary_header(), "abc".to_string());
        assert_eq!(schema.breakdown_header(), None);
        assert!(!schema.is_breakdown());
    }

    #[test]
    fn schema2_breakdown2_impl_trait() {
        let schema = Schema::two("abc", "def").breakdown_2nd();
        assert_eq!(schema.primary_dim(&(1u64, true)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true)), true);
        assert_eq!(schema.primary_header(), "abc".to_string());
        assert_eq!(schema.breakdown_header(), Some("def".to_string()));
        assert!(schema.is_breakdown());
    }
}
