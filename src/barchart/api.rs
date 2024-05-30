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
    ///
    /// Default: `false`.
    pub abbreviate: bool,
    /// Whether to show the aggregated result for the *non-primary* dimension(s) of the dataset.
    /// Notice, this is different from `show_aggregate` in the root [`Render`] configuration (which shows the *primary* dimension).
    ///
    /// The aggregate always represents the matched columns from the *primary* towards the *non-primary*.
    /// Consequently, the result at each level shows the aggregate across the previous level - the aggregates "cascade".
    ///
    /// | A (primary) | B | C | value |
    /// |-|-|-|-|
    /// | a1 | b1 | c1 | 1 |
    /// | a1 | b2 | c1 | 2 |
    /// | a1 | b2 | c2 | 3 |
    ///
    /// | Dimensional Combinations | Show Aggregate |
    /// |-|-|
    /// | (a1, ) | aggregate([1, 2, 3]) |
    /// | (a1, b1) | aggregate(\[1\]) |
    /// | (a1, b2) | aggregate([2, 3]) |
    /// | (a1, b1, c1) | aggregate(\[1\]) |
    /// | (a1, b2, c1) | aggregate(\[2\]) |
    /// | (a1, b2, c2) | aggregate(\[3\]) |
    ///
    /// In `flat`, the above dataset as [`Aggregate::Sum`] will render as follows:
    /// ```ignore
    /// // Output (modified for alignment)
    /// // Observe the cascade (ex: `b2 [5] = c1 [2] + c2 [3]`).
    /// r#"C  Sum   B  Sum   A  Sum
    ///    c1 [1] - b1 [1] ┐
    ///    c1 [2] - b2 [5] - a1 [6]
    ///    c2 [3] ┘        ┘
    ///  "#
    /// ```
    ///
    /// Default: `false`.
    pub show_aggregate: bool,
}

/// The internal trait which allows rendering [`BarChart`]s across different [`Schema']s.
/// Consumers should not implement this trait.
#[doc(hidden)]
pub trait BarChartSchematic {
    type Dimensions;
    type PrimaryDimension;
    type BreakdownDimension;
    type SortDimensions;

    #[doc(hidden)]
    fn total_width(&self) -> usize;

    #[doc(hidden)]
    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension;

    #[doc(hidden)]
    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension;

    #[doc(hidden)]
    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions;

    #[doc(hidden)]
    fn headers(&self) -> Vec<String>;

    #[doc(hidden)]
    fn data_header(&self) -> String;

    #[doc(hidden)]
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

    fn data_header(&self) -> String {
        self.values.clone()
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

    fn data_header(&self) -> String {
        self.values.clone()
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

    fn data_header(&self) -> String {
        self.dimension_2.clone()
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

    fn data_header(&self) -> String {
        self.values.clone()
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

    fn data_header(&self) -> String {
        self.dimension_2.clone()
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

    fn data_header(&self) -> String {
        self.dimension_3.clone()
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
        let schema = Schema::one("abc").values("header");
        assert_eq!(schema.primary_dim(&(1u64,)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64,)), Nothing {});
        assert_eq!(schema.sort_dims(&(1u64,)), (1u64,));
        assert_eq!(schema.headers(), vec!["abc".to_string()]);
        assert_eq!(schema.data_header(), "header".to_string());
        assert!(!schema.is_breakdown());
    }

    #[test]
    fn schema2_impl_trait() {
        let schema = Schema::two("abc", "def").values("header");
        assert_eq!(schema.primary_dim(&(1u64, true)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true)), Nothing {});
        assert_eq!(schema.sort_dims(&(1u64, true)), (1u64, true));
        assert_eq!(schema.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(schema.data_header(), "header".to_string());
        assert!(!schema.is_breakdown());
    }

    #[test]
    fn schema2_breakdown2_impl_trait() {
        let schema = Schema::two("abc", "def").breakdown_2nd();
        assert_eq!(schema.primary_dim(&(1u64, true)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true)), true);
        assert_eq!(schema.sort_dims(&(1u64, true)), (1u64,));
        assert_eq!(schema.headers(), vec!["abc".to_string()]);
        assert_eq!(schema.data_header(), "def".to_string());
        assert!(schema.is_breakdown());
    }

    #[test]
    fn schema3_impl_trait() {
        let schema = Schema::three("abc", "def", "ghi").values("header");
        assert_eq!(schema.primary_dim(&(1u64, true, 2f32)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true, 2f32)), Nothing {});
        assert_eq!(schema.sort_dims(&(1u64, true, 2f32)), (1u64, true, 2f32));
        assert_eq!(
            schema.headers(),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
        assert_eq!(schema.data_header(), "header".to_string());
        assert!(!schema.is_breakdown());
    }

    #[test]
    fn schema3_breakdown2_impl_trait() {
        let schema = Schema::three("abc", "def", "ghi").breakdown_2nd();
        assert_eq!(schema.primary_dim(&(1u64, true, 2f32)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true, 2f32)), true);
        assert_eq!(schema.sort_dims(&(1u64, true, 2f32)), (1u64, 2f32));
        assert_eq!(schema.headers(), vec!["abc".to_string(), "ghi".to_string()]);
        assert_eq!(schema.data_header(), "def".to_string());
        assert!(schema.is_breakdown());
    }

    #[test]
    fn schema3_breakdown3_impl_trait() {
        let schema = Schema::three("abc", "def", "ghi").breakdown_3rd();
        assert_eq!(schema.primary_dim(&(1u64, true, 2f32)), 1u64);
        assert_eq!(schema.breakdown_dim(&(1u64, true, 2f32)), 2f32);
        assert_eq!(schema.sort_dims(&(1u64, true, 2f32)), (1u64, true));
        assert_eq!(schema.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(schema.data_header(), "ghi".to_string());
        assert!(schema.is_breakdown());
    }
}
