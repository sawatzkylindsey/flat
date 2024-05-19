use crate::{
    Nothing, Schema1, Schema2, Schema2Breakdown2, Schema3, Schema3Breakdown2, Schema3Breakdown3,
};

#[derive(Debug, Default)]
pub struct BarChartConfig {
    /// Whether to abbreviate the dimensional values in the rendering or not.
    /// Use this option when the dimensions have long `std::fmt::Display` forms.
    /// Abbreviation is attempted irrespective of the `width_hint`.
    ///
    /// **Notice**, abbreviation is performed on values only, and not on the column headers.
    /// Moreover, the abbreviation is bounded by the column header width.
    /// If you want shorter abbreviations, be sure to shorten the headers as well!
    pub abbreviate: bool,
}

/// The internal trait which allows rendering [`BarChar`]s across different [`Schema']s.
/// Consumers should not implement this trait.
pub trait BarChartSchematic {
    type Dimensions;
    type PrimaryDimension;
    type BreakdownDimension;
    type SortDimensions;

    fn total_width(&self) -> usize;

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension;

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension;

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions;

    fn headers(&self) -> Vec<String>;

    fn breakdown_header(&self) -> Option<String>;

    fn is_breakdown(&self) -> bool;
}

impl<T> BarChartSchematic for Schema1<T>
where
    T: Clone,
{
    type Dimensions = (T,);
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T,);

    fn total_width(&self) -> usize {
        1
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, _dims: &Self::Dimensions) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

impl<T, U> BarChartSchematic for Schema2<T, U>
where
    T: Clone,
    U: Clone,
{
    type Dimensions = (T, U);
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T, U);

    fn total_width(&self) -> usize {
        2
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, _dims: &Self::Dimensions) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

impl<T, U> BarChartSchematic for Schema2Breakdown2<T, U>
where
    T: Clone,
    U: Clone,
{
    type Dimensions = (T, U);
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type SortDimensions = (T,);

    fn total_width(&self) -> usize {
        2
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension {
        dims.1.clone()
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(),)
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_2.clone())
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

impl<T, U, V> BarChartSchematic for Schema3<T, U, V>
where
    T: Clone,
    U: Clone,
    V: Clone,
{
    type Dimensions = (T, U, V);
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T, U, V);

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, _dims: &Self::Dimensions) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.column_1.clone(),
            self.column_2.clone(),
            self.column_3.clone(),
        ]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

impl<T, U, V> BarChartSchematic for Schema3Breakdown2<T, U, V>
where
    T: Clone,
    U: Clone,
    V: Clone,
{
    type Dimensions = (T, U, V);
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type SortDimensions = (T, V);

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension {
        dims.1.clone()
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.2.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_3.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_2.clone())
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

impl<T, U, V> BarChartSchematic for Schema3Breakdown3<T, U, V>
where
    T: Clone,
    U: Clone,
    V: Clone,
{
    type Dimensions = (T, U, V);
    type PrimaryDimension = T;
    type BreakdownDimension = V;
    type SortDimensions = (T, U);

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension {
        dims.2.clone()
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_3.clone())
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}
