use crate::{Nothing, Schema1};
use std::ops::{Add, Sub};

pub trait HistogramSchematic {
    type Dimensions;
    type PrimaryDimension: Binnable;
    type BreakdownDimension;

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension;

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension;

    fn primary_header(&self) -> Vec<String>;

    fn breakdown_header(&self) -> Option<String>;

    fn is_breakdown(&self) -> bool;
}

pub trait Binnable: PartialEq + PartialOrd + Add + Sub + Sized {
    fn ceiling_multiply(&self, rhs: usize) -> Self;

    fn ceiling_divide(&self, rhs: usize) -> Self;
}

impl Binnable for f64 {
    fn ceiling_multiply(&self, rhs: usize) -> Self {
        self * (rhs as f64)
    }

    fn ceiling_divide(&self, rhs: usize) -> Self {
        self / (rhs as f64)
    }
}

impl Binnable for u64 {
    fn ceiling_multiply(&self, rhs: usize) -> Self {
        (*self as f64 * (rhs as f64)).ceil() as u64
    }

    fn ceiling_divide(&self, rhs: usize) -> Self {
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

    fn primary_header(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}
