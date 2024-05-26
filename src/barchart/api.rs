use crate::{
    Nothing, Schema1, Schema2, Schema2Breakdown2, Schema3, Schema3Breakdown2, Schema3Breakdown3,
};
// We use this in the doc strings.
#[allow(unused_imports)]
use super::BarChart;

/// Render configuration specific to [`BarChart`]s.
#[derive(Debug, Default)]
pub struct BarChartConfig {
    /// Whether to abbreviate the dimensional values in the rendering or not.
    /// Use this option when the dimensions have long [`std::fmt::Display`] forms.
    /// Abbreviation is attempted irrespective of the `width_hint`.
    ///
    /// **Notice**, abbreviation is performed on values only, and not on the column headers.
    /// Moreover, the abbreviation is bounded by the column header width.
    /// If you want shorter abbreviations, be sure to shorten the headers as well!
    pub abbreviate: bool,
}

/// The internal trait which allows rendering [`BarChart`]s across different [`Schema']s.
/// Consumers should not implement this trait.
#[doc(hidden)]
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
        Nothing {}
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![self.dimension_1.clone()]
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
        Nothing {}
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![self.dimension_1.clone(), self.dimension_2.clone()]
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
        vec![self.dimension_1.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.dimension_2.clone())
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
        Nothing {}
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dimension_1.clone(),
            self.dimension_2.clone(),
            self.dimension_3.clone(),
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
        vec![self.dimension_1.clone(), self.dimension_3.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.dimension_2.clone())
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
        vec![self.dimension_1.clone(), self.dimension_2.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.dimension_3.clone())
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
    fn schema1_impl_trait() {
        let schema = Schema::one("abc");
        assert_eq!(schema.primary_dim(&(1u64,)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64,)), Nothing {});
        assert_eq!(schema.sort_dims(&(1u64,)), (1u64,));
        assert_eq!(schema.headers(), vec!["abc".to_string()]);
        assert_eq!(schema.breakdown_header(), None);
        assert!(!schema.is_breakdown());
    }

    #[test]
    fn schema2_impl_trait() {
        let schema = Schema::two("abc", "def");
        assert_eq!(schema.primary_dim(&(1u64, true)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true)), Nothing {});
        assert_eq!(schema.sort_dims(&(1u64, true)), (1u64, true));
        assert_eq!(schema.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(schema.breakdown_header(), None);
        assert!(!schema.is_breakdown());
    }

    #[test]
    fn schema2_breakdown2_impl_trait() {
        let schema = Schema::two("abc", "def").breakdown_2nd();
        assert_eq!(schema.primary_dim(&(1u64, true)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true)), true);
        assert_eq!(schema.sort_dims(&(1u64, true)), (1u64,));
        assert_eq!(schema.headers(), vec!["abc".to_string()]);
        assert_eq!(schema.breakdown_header(), Some("def".to_string()));
        assert!(schema.is_breakdown());
    }

    #[test]
    fn schema3_impl_trait() {
        let schema = Schema::three("abc", "def", "ghi");
        assert_eq!(schema.primary_dim(&(1u64, true, 2f32)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true, 2f32)), Nothing {});
        assert_eq!(schema.sort_dims(&(1u64, true, 2f32)), (1u64, true, 2f32));
        assert_eq!(
            schema.headers(),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
        assert_eq!(schema.breakdown_header(), None);
        assert!(!schema.is_breakdown());
    }

    #[test]
    fn schema3_breakdown2_impl_trait() {
        let schema = Schema::three("abc", "def", "ghi").breakdown_2nd();
        assert_eq!(schema.primary_dim(&(1u64, true, 2f32)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true, 2f32)), true);
        assert_eq!(schema.sort_dims(&(1u64, true, 2f32)), (1u64, 2f32));
        assert_eq!(schema.headers(), vec!["abc".to_string(), "ghi".to_string()]);
        assert_eq!(schema.breakdown_header(), Some("def".to_string()));
        assert!(schema.is_breakdown());
    }

    #[test]
    fn schema3_breakdown3_impl_trait() {
        let schema = Schema::three("abc", "def", "ghi").breakdown_3rd();
        assert_eq!(schema.primary_dim(&(1u64, true, 2f32)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true, 2f32)), 2f32);
        assert_eq!(schema.sort_dims(&(1u64, true, 2f32)), (1u64, true));
        assert_eq!(schema.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(schema.breakdown_header(), Some("ghi".to_string()));
        assert!(schema.is_breakdown());
    }
}
