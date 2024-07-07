use crate::{Dimensions, Nothing, Schema, Schema1, Schema2, Schema3, Schema4};
use std::fmt::Display;
// We use this in the doc strings.
#[allow(unused_imports)]
use crate::Dataset;
// We use this in the doc strings.
#[allow(unused_imports)]
use crate::Render;

/// Trait which defines how to render a [`Dataset`] across different [`Schema`]s.
/// Consumers may choose to implement this trait to provide custom views over datasets.
pub trait View<S: Schema> {
    /// The primary dimension - see [`View::primary_dim`] for more details.
    type PrimaryDimension;
    /// The breakdown dimension - see [`View::breakdown_dim`] for more details.
    type BreakdownDimension;
    /// The display dimension(s) - see [`View::display_dims`] for more details.
    /// The first must always be the `PrimaryDimension` for the `View`.
    type DisplayDimensions: Dimensions;

    /// Get the data associated with this view.
    fn dataset(&self) -> &Dataset<S>;

    /// Extract the render value for this view from the input vector.
    /// This will become aggregated according to the [`Render`] configuration.
    /// ```text
    /// Dimensions.. | Rendering
    /// primary_A    | aggregate(VALUE)
    /// ```
    fn value(&self, dims: &S::Dimensions) -> f64;

    /// Get the label for the values in this view.
    /// For example:
    /// ```text
    /// Dimensions.. | VALUE_LABEL
    /// primary_A    |     **
    /// primary_B    |   ******
    /// ```
    fn value_label(&self) -> String;

    /// Extract the primary dimension for this view from the input vector.
    ///
    /// In `flat`, the primary dimension forms the organizational basis for the rendering
    /// For example:
    /// ```text
    /// Dimensions.. | Rendering
    /// primary_A    |    **
    /// secondary_A                      <-
    /// tertiary_A                       <- these do not take part in the rendering directly
    /// primary_B    |  ******
    /// secondary_B                      <-
    /// ```
    fn primary_dim(&self, dims: &S::Dimensions) -> Self::PrimaryDimension;

    /// Extract the breakdown dimension for this view from the input vector.
    /// In the case of non-breakdown views (when [`View::breakdown_label`].is_none()), this method should return [`Nothing`].
    ///
    /// Breakdown views allow `flat` renderings to span the horizontal space.
    /// For example:
    /// ```text
    ///               | Breakdown Rendering..                 |
    /// Dimensions..  | breakdown_A  breakdown_B  breakdown_C |
    /// primary_A     |      *         ******                 |
    /// primary_B     |     ***                        *      |
    /// ```
    fn breakdown_dim(&self, dims: &S::Dimensions) -> Self::BreakdownDimension;

    /// Get the label for the breakdown in this view, if applicable.
    /// When `Some(...)`, [`View::breakdown_dim`] must extract the breakdown value.
    /// When `None`, [`View::breakdown_dim`] must extract [`Nothing`].
    fn breakdown_label(&self) -> Option<String>;

    /// Extract the display dimensions for this view from the input vector.
    ///
    /// The display dimensions are *optionally* shown in the frame of `flat`.
    /// How they are shown is totally at the discretion of the widget's implementation.
    /// However, the widget implementation must always include the primary dimension.
    /// For example:
    /// ```text
    /// Dimensions..                      | Rendering
    /// DISPLAY_A (primary) .. DISPLAY_N  |    **
    /// ```
    ///
    /// The first display dimension must always be the primary dimension.
    /// The length of the display dimensions must match the length of [`View::display_headers`].
    fn display_dims(&self, dims: &S::Dimensions) -> Self::DisplayDimensions;

    /// Get the dimensional headers for this view.
    /// These serve to label the display dimensions.
    ///
    /// The length of the headers must match the length of [`View::display_dims`].
    /// For example:
    /// ```text
    /// HEADER_1 .. HEADER_N | Rendering
    /// primary_A     ..     |    **
    /// primary_B     ..     |  ******
    /// ```
    fn display_headers(&self) -> Vec<String>;
}

#[doc(hidden)]
pub struct View1Full<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
    pub(crate) value_header: String,
}

impl<'a, T> View<Schema1<T>> for View1Full<'a, Schema1<T>>
where
    T: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = <Schema1<T> as Schema>::Dimensions;

    fn dataset(&self) -> &Dataset<Schema1<T>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema1<T> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.value_header.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(&self, dims: &<Schema1<T> as Schema>::Dimensions) -> Self::DisplayDimensions {
        dims.clone()
    }

    fn display_headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_0.clone()]
    }
}

#[doc(hidden)]
pub struct View2Full<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
    pub(crate) value_header: String,
}

impl<'a, T, U> View<Schema2<T, U>> for View2Full<'a, Schema2<T, U>>
where
    T: Clone + Display,
    U: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = (T, U);

    fn dataset(&self) -> &Dataset<Schema2<T, U>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.value_header.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(
        &self,
        dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View2Regular<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U> View<Schema2<T, U>> for View2Regular<'a, Schema2<T, U>>
where
    T: Clone + Display,
    U: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = (T,);

    fn dataset(&self) -> &Dataset<Schema2<T, U>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_1.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(
        &self,
        dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(),)
    }

    fn display_headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_0.clone()]
    }
}

#[doc(hidden)]
pub struct View2Breakdown2nd<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U> View<Schema2<T, U>> for View2Breakdown2nd<'a, Schema2<T, U>>
where
    T: Clone + Display,
    U: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type DisplayDimensions = (T,);

    fn dataset(&self) -> &Dataset<Schema2<T, U>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema2<T, U> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_1.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_1.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(),)
    }

    fn display_headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_0.clone()]
    }
}

#[doc(hidden)]
pub struct View2Breakdown2ndCount<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T, U> View<Schema2<T, U>> for View2Breakdown2ndCount<'a, Schema2<T, U>>
where
    T: Clone + Display,
    U: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type DisplayDimensions = (T,);

    fn dataset(&self) -> &Dataset<Schema2<T, U>> {
        &self.dataset
    }

    fn value(&self, _dims: &<Schema2<T, U> as Schema>::Dimensions) -> f64 {
        1f64
    }

    fn value_label(&self) -> String {
        "Count".to_string()
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

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_1.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema2<T, U> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(),)
    }

    fn display_headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_0.clone()]
    }
}

#[doc(hidden)]
pub struct View3Full<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
    pub(crate) value_header: String,
}

impl<'a, T, U, V> View<Schema3<T, U, V>> for View3Full<'a, Schema3<T, U, V>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = <Schema3<T, U, V> as Schema>::Dimensions;

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.value_header.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone(), dims.2.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View3Regular<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U, V> View<Schema3<T, U, V>> for View3Regular<'a, Schema3<T, U, V>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = (T, U);

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_2.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View3Breakdown3rd<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U, V> View<Schema3<T, U, V>> for View3Breakdown3rd<'a, Schema3<T, U, V>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = V;
    type DisplayDimensions = (T, U);

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_2.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_2.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View3Breakdown3rdCount<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T, U, V> View<Schema3<T, U, V>> for View3Breakdown3rdCount<'a, Schema3<T, U, V>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = V;
    type DisplayDimensions = (T, U);

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn value(&self, _dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        1f64
    }

    fn value_label(&self) -> String {
        "Count".to_string()
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

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_2.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View3Breakdown2ndView3rd<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U, V> View<Schema3<T, U, V>> for View3Breakdown2ndView3rd<'a, Schema3<T, U, V>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type DisplayDimensions = (T,);

    fn dataset(&self) -> &Dataset<Schema3<T, U, V>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema3<T, U, V> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_2.clone()
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

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_1.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema3<T, U, V> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(),)
    }

    fn display_headers(&self) -> Vec<String> {
        vec![self.dataset.schema.dimension_0.clone()]
    }
}

#[doc(hidden)]
pub struct View4Full<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
    pub(crate) value_header: String,
}

impl<'a, T, U, V, W> View<Schema4<T, U, V, W>> for View4Full<'a, Schema4<T, U, V, W>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
    W: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = <Schema4<T, U, V, W> as Schema>::Dimensions;

    fn dataset(&self) -> &Dataset<Schema4<T, U, V, W>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema4<T, U, V, W> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.value_header.clone()
    }

    fn primary_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing
    }

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (
            dims.0.clone(),
            dims.1.clone(),
            dims.2.clone(),
            dims.3.clone(),
        )
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
            self.dataset.schema.dimension_3.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View4Regular<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U, V, W> View<Schema4<T, U, V, W>> for View4Regular<'a, Schema4<T, U, V, W>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
    W: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = (T, U, V);

    fn dataset(&self) -> &Dataset<Schema4<T, U, V, W>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema4<T, U, V, W> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_3.clone()
    }

    fn primary_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing
    }

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone(), dims.2.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View4Breakdown4th<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U, V, W> View<Schema4<T, U, V, W>> for View4Breakdown4th<'a, Schema4<T, U, V, W>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
    W: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = W;
    type DisplayDimensions = (T, U, V);

    fn dataset(&self) -> &Dataset<Schema4<T, U, V, W>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema4<T, U, V, W> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_3.clone()
    }

    fn primary_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        dims.3.clone()
    }

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_3.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone(), dims.2.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View4Breakdown3rdView4th<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
    pub(crate) extractor: Box<dyn Fn(&S::Dimensions) -> f64>,
}

impl<'a, T, U, V, W> View<Schema4<T, U, V, W>> for View4Breakdown3rdView4th<'a, Schema4<T, U, V, W>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
    W: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = V;
    type DisplayDimensions = (T, U);

    fn dataset(&self) -> &Dataset<Schema4<T, U, V, W>> {
        &self.dataset
    }

    fn value(&self, dims: &<Schema4<T, U, V, W> as Schema>::Dimensions) -> f64 {
        (self.extractor)(dims)
    }

    fn value_label(&self) -> String {
        self.dataset.schema.dimension_3.clone()
    }

    fn primary_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        dims.2.clone()
    }

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_2.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
        ]
    }
}

#[doc(hidden)]
pub struct View4Breakdown4thCount<'a, S: Schema> {
    pub(crate) dataset: &'a Dataset<S>,
}

impl<'a, T, U, V, W> View<Schema4<T, U, V, W>> for View4Breakdown4thCount<'a, Schema4<T, U, V, W>>
where
    T: Clone + Display,
    U: Clone + Display,
    V: Clone + Display,
    W: Clone + Display,
{
    type PrimaryDimension = T;
    type BreakdownDimension = W;
    type DisplayDimensions = (T, U, V);

    fn dataset(&self) -> &Dataset<Schema4<T, U, V, W>> {
        &self.dataset
    }

    fn value(&self, _dims: &<Schema4<T, U, V, W> as Schema>::Dimensions) -> f64 {
        1f64
    }

    fn value_label(&self) -> String {
        "Count".to_string()
    }

    fn primary_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        dims.3.clone()
    }

    fn breakdown_label(&self) -> Option<String> {
        Some(self.dataset.schema.dimension_3.clone())
    }

    fn display_dims(
        &self,
        dims: &<Schema4<T, U, V, W> as Schema>::Dimensions,
    ) -> Self::DisplayDimensions {
        (dims.0.clone(), dims.1.clone(), dims.2.clone())
    }

    fn display_headers(&self) -> Vec<String> {
        vec![
            self.dataset.schema.dimension_0.clone(),
            self.dataset.schema.dimension_1.clone(),
            self.dataset.schema.dimension_2.clone(),
        ]
    }
}

// TODO
// pub struct View5<T, U, V, W, X>;
// pub struct View6<T, U, V, W, X, Y>;
// pub struct View7<T, U, V, W, X, Y, Z>;

#[cfg(test)]
mod tests {

    #[cfg(feature = "primitive_impls")]
    mod primitive_impls {
        use crate::{DatasetBuilder, Nothing, Schema1, Schema2, Schema3, Schema4, Schemas, View};

        #[test]
        fn view1_reflective() {
            let schema: Schema1<i64> = Schemas::one("abc");
            let dataset = DatasetBuilder::new(schema)
                .add((1,))
                .add((2,))
                .add((3,))
                .build();
            let view = dataset.reflect_1st();
            assert_eq!(view.primary_dim(&(2,)), 2);
            assert_eq!(view.breakdown_dim(&(2,)), Nothing);
            assert_eq!(view.display_dims(&(2,)), (2,));
            assert_eq!(view.display_headers(), vec!["abc".to_string()]);
            assert_eq!(view.value_label(), "abc".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view1_counting() {
            let schema: Schema1<i64> = Schemas::one("abc");
            let dataset = DatasetBuilder::new(schema)
                .add((1,))
                .add((2,))
                .add((3,))
                .build();
            let view = dataset.count();
            assert_eq!(view.primary_dim(&(2,)), 2);
            assert_eq!(view.breakdown_dim(&(2,)), Nothing);
            assert_eq!(view.display_dims(&(2,)), (2,));
            assert_eq!(view.display_headers(), vec!["abc".to_string()]);
            assert_eq!(view.value_label(), "Count".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view2_reflective() {
            let schema: Schema2<i64, f64> = Schemas::two("abc", "def");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1))
                .add((2, 0.2))
                .add((3, 0.3))
                .build();
            let view = dataset.reflect_2nd();
            assert_eq!(view.primary_dim(&(2, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing);
            assert_eq!(view.display_dims(&(2, 0.2)), (2, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string()]
            );
            assert_eq!(view.value_label(), "def".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view2_counting() {
            let schema: Schema2<i64, f64> = Schemas::two("abc", "def");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1))
                .add((2, 0.2))
                .add((3, 0.3))
                .build();
            let view = dataset.count();
            assert_eq!(view.primary_dim(&(2, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing);
            assert_eq!(view.display_dims(&(2, 0.2)), (2, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string()]
            );
            assert_eq!(view.value_label(), "Count".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view2_2nd() {
            let schema: Schema2<i64, f64> = Schemas::two("abc", "def");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1))
                .add((2, 0.2))
                .add((3, 0.3))
                .build();
            let view = dataset.view_2nd();
            assert_eq!(view.primary_dim(&(2, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2)), Nothing);
            assert_eq!(view.display_dims(&(2, 0.2)), (2,));
            assert_eq!(view.display_headers(), vec!["abc".to_string()]);
            assert_eq!(view.value_label(), "def".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view2_breakdown_2nd() {
            let schema: Schema2<i64, f64> = Schemas::two("abc", "def");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1))
                .add((2, 0.2))
                .add((3, 0.3))
                .build();
            let view = dataset.breakdown_2nd();
            assert_eq!(view.primary_dim(&(2, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2)), 0.2);
            assert_eq!(view.display_dims(&(2, 0.2)), (2,));
            assert_eq!(view.display_headers(), vec!["abc".to_string()]);
            assert_eq!(view.value_label(), "def".to_string());
            assert_eq!(view.breakdown_label(), Some("def".to_string()));
        }

        #[test]
        fn view2_count_breakdown_2nd() {
            let schema: Schema2<i64, &str> = Schemas::two("abc", "def");
            let dataset = DatasetBuilder::new(schema)
                .add((1, "a"))
                .add((2, "b"))
                .add((3, "c"))
                .build();
            let view = dataset.count_breakdown_2nd();
            assert_eq!(view.primary_dim(&(2, "b")), 2);
            assert_eq!(view.breakdown_dim(&(2, "b")), "b");
            assert_eq!(view.display_dims(&(2, "b")), (2,));
            assert_eq!(view.display_headers(), vec!["abc".to_string()]);
            assert_eq!(view.value_label(), "Count".to_string());
            assert_eq!(view.breakdown_label(), Some("def".to_string()));
        }

        #[test]
        fn view3_reflective() {
            let schema: Schema3<u64, bool, f64> = Schemas::three("abc", "def", "ghi");
            let dataset = DatasetBuilder::new(schema)
                .add((1, true, 0.1))
                .add((2, false, 0.2))
                .add((3, true, 0.3))
                .build();
            let view = dataset.reflect_3rd();
            assert_eq!(view.primary_dim(&(2, false, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, false, 0.2)), Nothing);
            assert_eq!(view.display_dims(&(2, false, 0.2)), (2, false, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
            );
            assert_eq!(view.value_label(), "ghi".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view3_counting() {
            let schema: Schema3<u64, bool, f64> = Schemas::three("abc", "def", "ghi");
            let dataset = DatasetBuilder::new(schema)
                .add((1, true, 0.1))
                .add((2, false, 0.2))
                .add((3, true, 0.3))
                .build();
            let view = dataset.count();
            assert_eq!(view.primary_dim(&(2, false, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, false, 0.2)), Nothing);
            assert_eq!(view.display_dims(&(2, false, 0.2)), (2, false, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
            );
            assert_eq!(view.value_label(), "Count".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view3_3rd() {
            let schema: Schema3<u64, bool, f64> = Schemas::three("abc", "def", "ghi");
            let dataset = DatasetBuilder::new(schema)
                .add((1, true, 0.1))
                .add((2, false, 0.2))
                .add((3, true, 0.3))
                .build();
            let view = dataset.view_3rd();
            assert_eq!(view.primary_dim(&(2, false, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, false, 0.2)), Nothing);
            assert_eq!(view.display_dims(&(2, false, 0.2)), (2, false));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string()]
            );
            assert_eq!(view.value_label(), "ghi".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view3_breakdown_3rd() {
            let schema: Schema3<u64, f32, f64> = Schemas::three("abc", "def", "ghi");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1, 0.01))
                .add((2, 0.2, 0.02))
                .add((3, 0.3, 0.03))
                .build();
            let view = dataset.breakdown_3rd();
            assert_eq!(view.primary_dim(&(2, 0.2, 0.02)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2, 0.02)), 0.02);
            assert_eq!(view.display_dims(&(2, 0.2, 0.02)), (2, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string()]
            );
            assert_eq!(view.value_label(), "ghi".to_string());
            assert_eq!(view.breakdown_label(), Some("ghi".to_string()));
        }

        #[test]
        fn view3_count_breakdown_3rd() {
            let schema: Schema3<u64, f32, bool> = Schemas::three("abc", "def", "ghi");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1, true))
                .add((2, 0.2, false))
                .add((3, 0.3, true))
                .build();
            let view = dataset.count_breakdown_3rd();
            assert_eq!(view.primary_dim(&(2, 0.2, false)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2, false)), false);
            assert_eq!(view.display_dims(&(2, 0.2, false)), (2, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string()]
            );
            assert_eq!(view.value_label(), "Count".to_string());
            assert_eq!(view.breakdown_label(), Some("ghi".to_string()));
        }

        #[test]
        fn view4_reflective() {
            let schema: Schema4<u64, bool, bool, f64> = Schemas::four("abc", "def", "ghi", "jkl");
            let dataset = DatasetBuilder::new(schema)
                .add((1, true, true, 0.1))
                .add((2, false, false, 0.2))
                .add((3, true, true, 0.3))
                .build();
            let view = dataset.reflect_4th();
            assert_eq!(view.primary_dim(&(2, false, false, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, false, false, 0.2)), Nothing);
            assert_eq!(
                view.display_dims(&(2, false, false, 0.2)),
                (2, false, false, 0.2)
            );
            assert_eq!(
                view.display_headers(),
                vec![
                    "abc".to_string(),
                    "def".to_string(),
                    "ghi".to_string(),
                    "jkl".to_string()
                ]
            );
            assert_eq!(view.value_label(), "jkl".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view4_counting() {
            let schema: Schema4<u64, bool, bool, f64> = Schemas::four("abc", "def", "ghi", "jkl");
            let dataset = DatasetBuilder::new(schema)
                .add((1, true, true, 0.1))
                .add((2, false, false, 0.2))
                .add((3, true, true, 0.3))
                .build();
            let view = dataset.count();
            assert_eq!(view.primary_dim(&(2, false, false, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, false, false, 0.2)), Nothing);
            assert_eq!(
                view.display_dims(&(2, false, false, 0.2)),
                (2, false, false, 0.2)
            );
            assert_eq!(
                view.display_headers(),
                vec![
                    "abc".to_string(),
                    "def".to_string(),
                    "ghi".to_string(),
                    "jkl".to_string()
                ]
            );
            assert_eq!(view.value_label(), "Count".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view4_4th() {
            let schema: Schema4<u64, bool, bool, f64> = Schemas::four("abc", "def", "ghi", "jkl");
            let dataset = DatasetBuilder::new(schema)
                .add((1, true, true, 0.1))
                .add((2, false, false, 0.2))
                .add((3, true, false, 0.3))
                .build();
            let view = dataset.view_4th();
            assert_eq!(view.primary_dim(&(2, false, false, 0.2)), 2);
            assert_eq!(view.breakdown_dim(&(2, false, false, 0.2)), Nothing);
            assert_eq!(
                view.display_dims(&(2, false, false, 0.2)),
                (2, false, false)
            );
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
            );
            assert_eq!(view.value_label(), "jkl".to_string());
            assert_eq!(view.breakdown_label(), None);
        }

        #[test]
        fn view4_breakdown_4th() {
            let schema: Schema4<u64, f32, f32, f64> = Schemas::four("abc", "def", "ghi", "jkl");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1, 0.1, 0.01))
                .add((2, 0.2, 0.2, 0.02))
                .add((3, 0.3, 0.3, 0.03))
                .build();
            let view = dataset.breakdown_4th();
            assert_eq!(view.primary_dim(&(2, 0.2, 0.2, 0.02)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2, 0.2, 0.02)), 0.02);
            assert_eq!(view.display_dims(&(2, 0.2, 0.2, 0.02)), (2, 0.2, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
            );
            assert_eq!(view.value_label(), "jkl".to_string());
            assert_eq!(view.breakdown_label(), Some("jkl".to_string()));
        }

        #[test]
        fn view4_count_breakdown_4th() {
            let schema: Schema4<u64, f32, f32, bool> = Schemas::four("abc", "def", "ghi", "jkl");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 0.1, 0.1, true))
                .add((2, 0.2, 0.2, false))
                .add((3, 0.3, 0.3, true))
                .build();
            let view = dataset.count_breakdown_4th();
            assert_eq!(view.primary_dim(&(2, 0.2, 0.2, false)), 2);
            assert_eq!(view.breakdown_dim(&(2, 0.2, 0.2, false)), false);
            assert_eq!(view.display_dims(&(2, 0.2, 0.2, false)), (2, 0.2, 0.2));
            assert_eq!(
                view.display_headers(),
                vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
            );
            assert_eq!(view.value_label(), "Count".to_string());
            assert_eq!(view.breakdown_label(), Some("jkl".to_string()));
        }
    }
}
