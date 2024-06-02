use crate::{Dataset, Nothing, Schema, Schema1, Schema2, Schema3};

/// The internal trait which defines how to render a [`Dataset`] across different [`Schema']s.
/// Consumers should not implement this trait.
#[doc(hidden)]
pub trait View<S: Schema> {
    type Dimensions;
    type PrimaryDimension;
    type BreakdownDimension;
    type SortDimensions;

    #[doc(hidden)]
    fn dataset(&self) -> &Dataset<S>;

    #[doc(hidden)]
    fn total_width(&self) -> usize;

    #[doc(hidden)]
    fn primary_dim(&self, dims: &S::Dimensions) -> Self::PrimaryDimension;

    #[doc(hidden)]
    fn breakdown_dim(&self, dims: &S::Dimensions) -> Self::BreakdownDimension;

    #[doc(hidden)]
    fn sort_dims(&self, dims: &S::Dimensions) -> Self::SortDimensions;

    #[doc(hidden)]
    fn headers(&self) -> Vec<String>;

    #[doc(hidden)]
    fn data_header(&self) -> String;

    #[doc(hidden)]
    fn is_breakdown(&self) -> bool;
}

/// The regular view of a 1-dimensional schema.
/// Constructed via [`Dataset`].
#[doc(hidden)]
pub struct View1<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone> View<Schema1<T>> for View1<'a, Schema1<T>> {
    type Dimensions = <Schema1<T> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = <Schema1<T> as Schema>::Dimensions;

    fn dataset(&self) -> &Dataset<Schema1<T>> {
        &self.dataset
    }

    fn total_width(&self) -> usize {
        1
    }

    fn primary_dim(&self, dims: &<Schema1<T> as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema1<T> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing {}
    }

    fn sort_dims(&self, dims: &<Schema1<T> as Schema>::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_1.clone()]
    }

    fn data_header(&self) -> String {
        self.dataset.schema.values.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

/// The regular view of a 2-dimensional schema.
/// Constructed via [`Dataset`].
#[doc(hidden)]
pub struct View2<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone, U: Clone> View<Schema2<T, U>> for View2<'a, Schema2<T, U>> {
    type Dimensions = <Schema2<T, U> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = <Schema2<T, U> as Schema>::Dimensions;

    fn dataset(&self) -> &Dataset<Schema2<T, U>> {
        &self.dataset
    }

    fn total_width(&self) -> usize {
        2
    }

    fn primary_dim(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing {}
    }

    fn sort_dims(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }

    fn data_header(&self) -> String {
        self.dataset.schema.values.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

/// The reverse view of a 2-dimensional schema (`(T, U)` is viewed as `(U, T)`).
/// Constructed via [`Dataset`].
#[doc(hidden)]
pub struct ReverseView2<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone, U: Clone> View<Schema2<T, U>> for ReverseView2<'a, Schema2<T, U>> {
    type Dimensions = <Schema2<T, U> as Schema>::Dimensions;
    type PrimaryDimension = U;
    type BreakdownDimension = Nothing;
    type SortDimensions = (U, T);

    fn dataset(&self) -> &Dataset<Schema2<T, U>> {
        &self.dataset
    }

    fn total_width(&self) -> usize {
        2
    }

    fn primary_dim(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.1.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing {}
    }

    fn sort_dims(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.1.clone(), dims.0.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_2.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }

    fn data_header(&self) -> String {
        self.dataset.schema.values.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

/// The view of a 2-dimensional schema with a breakdown on the 2nd dimension.
/// Constructed via [`Dataset`].
#[doc(hidden)]
pub struct View2Breakdown2<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone, U: Clone> View<Schema2<T, U>> for View2Breakdown2<'a, Schema2<T, U>> {
    type Dimensions = <Schema2<T, U> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type SortDimensions = (T,);

    fn dataset(&self) -> &Dataset<Schema2<T, U>> {
        &self.dataset
    }

    fn total_width(&self) -> usize {
        2
    }

    fn primary_dim(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        dims.1.clone()
    }

    fn sort_dims(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(),)
    }

    fn headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_1.clone()]
    }

    fn data_header(&self) -> String {
        self.dataset.schema.dimension_2.clone()
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

/// The regular view of a 3-dimensional schema.
/// Constructed via [`Dataset`].
#[doc(hidden)]
pub struct View3<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone, U: Clone, V: Clone> View<Schema3<T, U, V>> for View3<'a, Schema3<T, U, V>> {
    type Dimensions = <Schema3<T, U, V> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = <Schema3<T, U, V> as Schema>::Dimensions;

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing {}
    }

    fn sort_dims(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.1.clone(), dims.2.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
            self.dataset.schema.dimension_3.clone(),
        ]
    }

    fn data_header(&self) -> String {
        self.dataset.schema.values.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

/// The view of a 3-dimensional schema with a breakdown on the 2nd dimension.
/// Constructed via [`Dataset`].
#[doc(hidden)]
pub struct View3Breakdown2<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone, U: Clone, V: Clone> View<Schema3<T, U, V>>
    for View3Breakdown2<'a, Schema3<T, U, V>>
{
    type Dimensions = <Schema3<T, U, V> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type SortDimensions = (T, V);

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        dims.1.clone()
    }

    fn sort_dims(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.2.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_3.clone(),
        ]
    }

    fn data_header(&self) -> String {
        self.dataset.schema.dimension_2.clone()
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

/// The view of a 3-dimensional schema with a breakdown on the 3rd dimension.
/// Constructed via [`Dataset`].
#[doc(hidden)]
pub struct View3Breakdown3<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone, U: Clone, V: Clone> View<Schema3<T, U, V>>
    for View3Breakdown3<'a, Schema3<T, U, V>>
{
    type Dimensions = <Schema3<T, U, V> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = V;
    type SortDimensions = (T, U);

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        dims.2.clone()
    }

    fn sort_dims(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }

    fn data_header(&self) -> String {
        self.dataset.schema.dimension_3.clone()
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

// TODO
// pub struct View4<T, U, V, W>;
// pub struct View5<T, U, V, W, X>;
// pub struct View6<T, U, V, W, X, Y>;
// pub struct View7<T, U, V, W, X, Y, Z>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Schemas;

    #[test]
    fn view1() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let builder = Dataset::builder(schema)
            .add((1,), -1)
            .add((2,), 0)
            .add((3,), 1);
        let view = builder.view();
        assert!(std::ptr::eq(view.dataset(), &builder));
        assert_eq!(view.total_width(), 1);
        assert_eq!(view.primary_dim(&(2,)), 2);
        assert_eq!(view.breakdown_dim(&(2,)), Nothing {});
        assert_eq!(view.sort_dims(&(2,)), (2,));
        assert_eq!(view.headers(), vec!["abc".to_string()]);
        assert_eq!(view.data_header(), "header".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view2() {
        let schema: Schema2<u64, f32> = Schemas::two("abc", "def", "header");
        let builder = Dataset::builder(schema)
            .add((1, 0.1), -1)
            .add((2, 0.2), 0)
            .add((3, 0.3), 1);
        let view = builder.view();
        assert!(std::ptr::eq(view.dataset(), &builder));
        assert_eq!(view.total_width(), 2);
        assert_eq!(view.primary_dim(&(2, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing {});
        assert_eq!(view.sort_dims(&(2, 0.2)), (2, 0.2));
        assert_eq!(view.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(view.data_header(), "header".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn reverse_view2() {
        let schema: Schema2<u64, f32> = Schemas::two("abc", "def", "header");
        let builder = Dataset::builder(schema)
            .add((1, 0.1), -1)
            .add((2, 0.2), 0)
            .add((3, 0.3), 1);
        let view = builder.reverse_view();
        assert!(std::ptr::eq(view.dataset(), &builder));
        assert_eq!(view.total_width(), 2);
        assert_eq!(view.primary_dim(&(2, 0.2)), 0.2);
        assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing {});
        assert_eq!(view.sort_dims(&(2, 0.2)), (0.2, 2));
        assert_eq!(view.headers(), vec!["def".to_string(), "abc".to_string()]);
        assert_eq!(view.data_header(), "header".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view2_breakdown2() {
        let schema: Schema2<u64, f32> = Schemas::two("abc", "def", "header");
        let builder = Dataset::builder(schema)
            .add((1, 0.1), -1)
            .add((2, 0.2), 0)
            .add((3, 0.3), 1);
        let view = builder.view_breakdown2();
        assert!(std::ptr::eq(view.dataset(), &builder));
        assert_eq!(view.total_width(), 2);
        assert_eq!(view.primary_dim(&(2, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2)), 0.2);
        assert_eq!(view.sort_dims(&(2, 0.2)), (2,));
        assert_eq!(view.headers(), vec!["abc".to_string()]);
        assert_eq!(view.data_header(), "def".to_string());
        assert_eq!(view.is_breakdown(), true);
    }

    #[test]
    fn view3() {
        let schema: Schema3<u64, f32, bool> = Schemas::three("abc", "def", "ghi", "header");
        let builder = Dataset::builder(schema)
            .add((1, 0.1, true), -1)
            .add((2, 0.2, false), 0)
            .add((3, 0.3, true), 1);
        let view = builder.view();
        assert!(std::ptr::eq(view.dataset(), &builder));
        assert_eq!(view.total_width(), 3);
        assert_eq!(view.primary_dim(&(2, 0.2, false)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2, false)), Nothing {});
        assert_eq!(view.sort_dims(&(2, 0.2, false)), (2, 0.2, false));
        assert_eq!(
            view.headers(),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
        assert_eq!(view.data_header(), "header".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view3_breakdown2() {
        let schema: Schema3<u64, f32, bool> = Schemas::three("abc", "def", "ghi", "header");
        let builder = Dataset::builder(schema)
            .add((1, 0.1, true), -1)
            .add((2, 0.2, false), 0)
            .add((3, 0.3, true), 1);
        let view = builder.view_breakdown2();
        assert!(std::ptr::eq(view.dataset(), &builder));
        assert_eq!(view.total_width(), 3);
        assert_eq!(view.primary_dim(&(2, 0.2, false)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2, false)), 0.2);
        assert_eq!(view.sort_dims(&(2, 0.2, false)), (2, false));
        assert_eq!(view.headers(), vec!["abc".to_string(), "ghi".to_string()]);
        assert_eq!(view.data_header(), "def".to_string());
        assert_eq!(view.is_breakdown(), true);
    }

    #[test]
    fn view3_breakdown3() {
        let schema: Schema3<u64, f32, bool> = Schemas::three("abc", "def", "ghi", "header");
        let builder = Dataset::builder(schema)
            .add((1, 0.1, true), -1)
            .add((2, 0.2, false), 0)
            .add((3, 0.3, true), 1);
        let view = builder.view_breakdown3();
        assert!(std::ptr::eq(view.dataset(), &builder));
        assert_eq!(view.total_width(), 3);
        assert_eq!(view.primary_dim(&(2, 0.2, false)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2, false)), false);
        assert_eq!(view.sort_dims(&(2, 0.2, false)), (2, 0.2));
        assert_eq!(view.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(view.data_header(), "ghi".to_string());
        assert_eq!(view.is_breakdown(), true);
    }
}
