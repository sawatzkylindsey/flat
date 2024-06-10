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
    fn data(&self) -> &[S::Dimensions];

    fn value(&self, dims: &S::Dimensions) -> f64;

    #[doc(hidden)]
    fn primary_dim(&self, dims: &S::Dimensions) -> Self::PrimaryDimension;

    #[doc(hidden)]
    fn breakdown_dim(&self, dims: &S::Dimensions) -> Self::BreakdownDimension;

    #[doc(hidden)]
    fn sort_dims(&self, dims: &S::Dimensions) -> Self::SortDimensions;

    #[doc(hidden)]
    fn headers(&self) -> Vec<String>;

    #[doc(hidden)]
    fn value_header(&self) -> String;

    #[doc(hidden)]
    fn is_breakdown(&self) -> bool;
}

#[doc(hidden)]
pub struct View1Full<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
    pub(crate) value_header: String,
}

impl<'a, T: Clone> View<Schema1<T>> for View1Full<'a, Schema1<T>> {
    type Dimensions = <Schema1<T> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = <Schema1<T> as Schema>::Dimensions;

    fn data(&self) -> &[<Schema1<T> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, dims: &<Schema1<T> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn primary_dim(&self, dims: &<Schema1<T> as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema1<T> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &<Schema1<T> as Schema>::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_0.clone()]
    }

    fn value_header(&self) -> String {
        self.value_header.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub struct View2Full<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
    pub(crate) value_header: String,
}

impl<'a, T: Clone, U: Clone> View<Schema2<T, U>> for View2Full<'a, Schema2<T, U>> {
    type Dimensions = <Schema2<T, U> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T, U);

    fn data(&self) -> &[<Schema2<T, U> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn primary_dim(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }

    fn value_header(&self) -> String {
        self.value_header.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub struct View2Regular<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T: Clone, U: Clone> View<Schema2<T, U>> for View2Regular<'a, Schema2<T, U>> {
    type Dimensions = <Schema2<T, U> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T,);

    fn data(&self) -> &[<Schema2<T, U> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn primary_dim(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(),)
    }

    fn headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_0.clone()]
    }

    fn value_header(&self) -> String {
        self.dataset.schema.dimension_1.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub struct View2Breakdown2<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T: Clone, U: Clone> View<Schema2<T, U>> for View2Breakdown2<'a, Schema2<T, U>> {
    type Dimensions = <Schema2<T, U> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type SortDimensions = (T,);

    fn data(&self) -> &[<Schema2<T, U> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, _dims: &<Schema2<T, U> as Schema>::Dimensions) -> f64 {
        // Breakdown's always use counts.
        1f64
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
        vec![self.dataset.schema.dimension_0.clone()]
    }

    fn value_header(&self) -> String {
        format!("Breakdown({})", self.dataset.schema.dimension_1)
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

#[doc(hidden)]
pub struct View3Full<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
    pub(crate) value_header: String,
}

impl<'a, T: Clone, U: Clone, V: Clone> View<Schema3<T, U, V>> for View3Full<'a, Schema3<T, U, V>> {
    type Dimensions = <Schema3<T, U, V> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = <Schema3<T, U, V> as Schema>::Dimensions;

    fn data(&self) -> &[<Schema3<T, U, V> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
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
        Nothing
    }

    fn sort_dims(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.1.clone(), dims.2.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }

    fn value_header(&self) -> String {
        self.value_header.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub struct View3Regular<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T: Clone, U: Clone, V: Clone> View<Schema3<T, U, V>>
    for View3Regular<'a, Schema3<T, U, V>>
{
    type Dimensions = <Schema3<T, U, V> as Schema>::Dimensions;
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T, U);

    fn data(&self) -> &[<Schema3<T, U, V> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
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
        Nothing
    }

    fn sort_dims(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }

    fn value_header(&self) -> String {
        self.dataset.schema.dimension_2.clone()
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

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

    fn data(&self) -> &[<Schema3<T, U, V> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, _dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        1f64
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
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }

    fn value_header(&self) -> String {
        format!("Breakdown({})", self.dataset.schema.dimension_1)
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

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

    fn data(&self) -> &[<Schema3<T, U, V> as Schema>::Dimensions] {
        self.dataset.data.as_slice()
    }

    fn value(&self, _dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        1f64
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
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }

    fn value_header(&self) -> String {
        format!("Breakdown({})", self.dataset.schema.dimension_2)
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
    fn view1_reflective() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let dataset = Dataset::builder(schema)
            .add((1,))
            .add((2,))
            .add((3,))
            .build();
        let view = dataset.reflective_view();
        assert_eq!(view.primary_dim(&(2,)), 2);
        assert_eq!(view.breakdown_dim(&(2,)), Nothing);
        assert_eq!(view.sort_dims(&(2,)), (2,));
        assert_eq!(view.headers(), vec!["abc".to_string()]);
        assert_eq!(view.value_header(), "abc".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view1_counting() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let dataset = Dataset::builder(schema)
            .add((1,))
            .add((2,))
            .add((3,))
            .build();
        let view = dataset.counting_view();
        assert_eq!(view.primary_dim(&(2,)), 2);
        assert_eq!(view.breakdown_dim(&(2,)), Nothing);
        assert_eq!(view.sort_dims(&(2,)), (2,));
        assert_eq!(view.headers(), vec!["abc".to_string()]);
        assert_eq!(view.value_header(), "Count".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view2_reflective() {
        let schema: Schema2<i64, f64> = Schemas::two("abc", "def");
        let dataset = Dataset::builder(schema)
            .add((1, 0.1))
            .add((2, 0.2))
            .add((3, 0.3))
            .build();
        let view = dataset.reflective_view();
        assert_eq!(view.primary_dim(&(2, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing);
        assert_eq!(view.sort_dims(&(2, 0.2)), (2, 0.2));
        assert_eq!(view.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(view.value_header(), "def".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view2_counting() {
        let schema: Schema2<i64, f64> = Schemas::two("abc", "def");
        let dataset = Dataset::builder(schema)
            .add((1, 0.1))
            .add((2, 0.2))
            .add((3, 0.3))
            .build();
        let view = dataset.counting_view();
        assert_eq!(view.primary_dim(&(2, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing);
        assert_eq!(view.sort_dims(&(2, 0.2)), (2, 0.2));
        assert_eq!(view.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(view.value_header(), "Count".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view2_2nd() {
        let schema: Schema2<i64, f64> = Schemas::two("abc", "def");
        let dataset = Dataset::builder(schema)
            .add((1, 0.1))
            .add((2, 0.2))
            .add((3, 0.3))
            .build();
        let view = dataset.view_2nd();
        assert_eq!(view.primary_dim(&(2, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing);
        assert_eq!(view.sort_dims(&(2, 0.2)), (2,));
        assert_eq!(view.headers(), vec!["abc".to_string()]);
        assert_eq!(view.value_header(), "def".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view2_breakdown2nd() {
        let schema: Schema2<i64, f32> = Schemas::two("abc", "def");
        let dataset = Dataset::builder(schema)
            .add((1, 0.1))
            .add((2, 0.2))
            .add((3, 0.3))
            .build();
        let view = dataset.breakdown_2nd();
        assert_eq!(view.primary_dim(&(2, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2)), 0.2);
        assert_eq!(view.sort_dims(&(2, 0.2)), (2,));
        assert_eq!(view.headers(), vec!["abc".to_string()]);
        assert_eq!(view.value_header(), "Breakdown(def)".to_string());
        assert_eq!(view.is_breakdown(), true);
    }

    #[test]
    fn view3_reflective() {
        let schema: Schema3<u64, bool, f64> = Schemas::three("abc", "def", "ghi");
        let dataset = Dataset::builder(schema)
            .add((1, true, 0.1))
            .add((2, false, 0.2))
            .add((3, true, 0.3))
            .build();
        let view = dataset.reflective_view();
        assert_eq!(view.primary_dim(&(2, false, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, false, 0.2)), Nothing);
        assert_eq!(view.sort_dims(&(2, false, 0.2)), (2, false, 0.2));
        assert_eq!(
            view.headers(),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
        assert_eq!(view.value_header(), "ghi".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view3_counting() {
        let schema: Schema3<u64, bool, f64> = Schemas::three("abc", "def", "ghi");
        let dataset = Dataset::builder(schema)
            .add((1, true, 0.1))
            .add((2, false, 0.2))
            .add((3, true, 0.3))
            .build();
        let view = dataset.counting_view();
        assert_eq!(view.primary_dim(&(2, false, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, false, 0.2)), Nothing);
        assert_eq!(view.sort_dims(&(2, false, 0.2)), (2, false, 0.2));
        assert_eq!(
            view.headers(),
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
        );
        assert_eq!(view.value_header(), "Count".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view3_3rd() {
        let schema: Schema3<u64, bool, f64> = Schemas::three("abc", "def", "ghi");
        let dataset = Dataset::builder(schema)
            .add((1, true, 0.1))
            .add((2, false, 0.2))
            .add((3, true, 0.3))
            .build();
        let view = dataset.view_3rd();
        assert_eq!(view.primary_dim(&(2, false, 0.2)), 2);
        assert_eq!(view.breakdown_dim(&(2, false, 0.2)), Nothing);
        assert_eq!(view.sort_dims(&(2, false, 0.2)), (2, false));
        assert_eq!(view.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(view.value_header(), "ghi".to_string());
        assert_eq!(view.is_breakdown(), false);
    }

    #[test]
    fn view3_breakdown_2nd() {
        let schema: Schema3<u64, f32, bool> = Schemas::three("abc", "def", "ghi");
        let dataset = Dataset::builder(schema)
            .add((1, 0.1, true))
            .add((2, 0.2, false))
            .add((3, 0.3, true))
            .build();
        let view = dataset.breakdown_2nd();
        assert_eq!(view.primary_dim(&(2, 0.2, false)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2, false)), 0.2);
        assert_eq!(view.sort_dims(&(2, 0.2, false)), (2, false));
        assert_eq!(view.headers(), vec!["abc".to_string(), "ghi".to_string()]);
        assert_eq!(view.value_header(), "Breakdown(def)".to_string());
        assert_eq!(view.is_breakdown(), true);
    }

    #[test]
    fn view3_breakdown_3rd() {
        let schema: Schema3<u64, f32, bool> = Schemas::three("abc", "def", "ghi");
        let dataset = Dataset::builder(schema)
            .add((1, 0.1, true))
            .add((2, 0.2, false))
            .add((3, 0.3, true))
            .build();
        let view = dataset.breakdown_3rd();
        assert_eq!(view.primary_dim(&(2, 0.2, false)), 2);
        assert_eq!(view.breakdown_dim(&(2, 0.2, false)), false);
        assert_eq!(view.sort_dims(&(2, 0.2, false)), (2, 0.2));
        assert_eq!(view.headers(), vec!["abc".to_string(), "def".to_string()]);
        assert_eq!(view.value_header(), "Breakdown(ghi)".to_string());
        assert_eq!(view.is_breakdown(), true);
    }
}
