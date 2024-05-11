use crate::{Nothing, Schema1, Schema2Breakdown2};
use std::ops::{Add, Sub};

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

pub trait Binnable: PartialEq + PartialOrd + Add + Sub + Sized {
    /// Multiply this value (`self: T`) by the `rhs: usize`, resulting in another `T`.
    ///
    /// **Notice**: For *whole* types `T` (types that belong to ℤ) the resulting value must be rounded up.
    /// In other words, implement `multiply` using `ceil` for integer types.
    fn multiply(&self, rhs: usize) -> Self;

    /// Divide this value (`self: T`) by the `rhs: usize`, resulting in another `T`.
    ///
    /// **Notice**: For *whole* types `T` (types that belong to ℤ) the resulting value must be rounded up.
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
