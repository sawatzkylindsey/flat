use crate::{
    Schema, Schema1, Schema2, Schema3, View1Full, View2Breakdown, View2BreakdownCount, View2Full,
    View2Regular, View3Breakdown, View3BreakdownCount, View3Full, View3Regular,
};
// We use this in the doc strings.
#[allow(unused_imports)]
use super::View;
// We use this in the doc strings.
#[allow(unused_imports)]
use super::Schemas;
#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
use ordered_float::OrderedFloat;

/// A dataset in `flat`.
/// The same dataset may be observed through multiple views.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let builder: Dataset<Schema1<f64>> = Dataset::builder(Schemas::one("dim1")).build();
/// ```
pub struct Dataset<S: Schema> {
    pub(crate) schema: S,
    data: Vec<S::Dimensions>,
}

impl<S: Schema> Dataset<S> {
    /// Get the data held within this `Dataset`.
    pub fn data(&self) -> &[S::Dimensions] {
        self.data.as_slice()
    }
}

/// Builder for a dataset in `flat`.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let builder: DatasetBuilder<Schema1<f64>> = Dataset::builder(Schemas::one("dim1"));
/// ```
pub struct DatasetBuilder<S: Schema> {
    schema: S,
    data: Vec<S::Dimensions>,
}

impl<'a, S: Schema> Dataset<S> {
    /// Make a copy of this dataset by recasting it into a new schema.
    ///
    /// Notice, this is an expensive operation (typically requires the whole dataset to be cloned).
    /// For a cheaper alternative, use either a built-in or custom [`View`].
    pub fn recast<R: Schema>(
        &self,
        new_schema: R,
        data_mapper: impl Fn(&S::Dimensions) -> R::Dimensions,
    ) -> Dataset<R> {
        Dataset {
            schema: new_schema,
            data: self.data.iter().map(data_mapper).collect(),
        }
    }
}

// Would love to be able to do this, but it conflicts with the specific `impl Dataset<Schema1<i32>>` (etc) implementations.
// We need specialization!
// impl<T: Deref<Target = f64>> Dataset<Schema1<T>> {
// impl<T: Into<f64> + Clone> Dataset<Schema1<T>> {

macro_rules! impl_schema1_view {
    ($T:ty, $attrs:meta) => {
        #[$attrs]
        #[allow(rustdoc::broken_intra_doc_links)]
        impl Dataset<Schema1<$T>> {
            /// Take a reflective view of this 1-dimensional dataset.
            /// Views are rendered differently by different widgets, but
            /// always have a frame on the left and a rendering on the right.
            ///
            /// This view will render the final dimension (1st), and use all other dimensions in the frame of the widget.
            /// The term 'reflection' refers to the fact that the value will appear in both in the frame and rendering of the widget.
            /// ```text
            /// r#"
            /// Frame..   | Rendering....
            /// (dim1, )  | aggregate(dim1)"#
            /// ```
            ///
            /// Note, implemented for all numeric primitives.
            /// To use with pointer types (such as [`ordered_float::OrderedFloat`]), see features.
            pub fn reflect_1st(&self) -> View1Full<Schema1<$T>> {
                let extractor: Box<dyn Fn(&<Schema1<$T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| d.0 as f64);
                View1Full {
                    dataset: &self,
                    extractor,
                    value_header: self.schema.dimension_0.clone(),
                }
            }
        }
    };
}

impl_schema1_view!(f64, doc());
impl_schema1_view!(f32, doc(hidden));
impl_schema1_view!(isize, doc(hidden));
impl_schema1_view!(i128, doc(hidden));
impl_schema1_view!(i64, doc(hidden));
impl_schema1_view!(i32, doc(hidden));
impl_schema1_view!(i16, doc(hidden));
impl_schema1_view!(i8, doc(hidden));
impl_schema1_view!(usize, doc(hidden));
impl_schema1_view!(u128, doc(hidden));
impl_schema1_view!(u64, doc(hidden));
impl_schema1_view!(u32, doc(hidden));
impl_schema1_view!(u16, doc(hidden));
impl_schema1_view!(u8, doc(hidden));

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
macro_rules! impl_schema1_view_smart_pointer {
    ($T:ty) => {
        impl Dataset<Schema1<$T>> {
            pub fn reflect_1st(&self) -> View1Full<Schema1<$T>> {
                let extractor: Box<dyn Fn(&<Schema1<$T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| (*d.0) as f64);
                View1Full {
                    dataset: &self,
                    extractor,
                    value_header: self.schema.dimension_0.clone(),
                }
            }
        }
    };
}

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl_schema1_view_smart_pointer!(OrderedFloat<f64>);
#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl_schema1_view_smart_pointer!(OrderedFloat<f32>);

impl<T> Dataset<Schema1<T>> {
    /// Take a counting view of this 1-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the occurrences of each dimensional vector, while using all dimensions in the frame of the widget.
    /// ```text
    /// r#"
    /// Frame..   | Rendering..
    /// (dim1, )  | aggregate(count())"#
    /// ```
    pub fn count(&self) -> View1Full<Schema1<T>> {
        let extractor: Box<dyn Fn(&<Schema1<T> as Schema>::Dimensions) -> f64> = Box::new(|_| 1f64);
        View1Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }
}

macro_rules! impl_schema2_view {
    ($T:ty, $attrs:meta) => {
        #[$attrs]
        #[allow(rustdoc::broken_intra_doc_links)]
        impl<T> Dataset<Schema2<T, $T>> {
            /// Take a reflective view of this 2-dimensional dataset.
            /// Views are rendered differently by different widgets, but
            /// always have a frame on the left and a rendering on the right.
            ///
            /// This view will render the final dimension (2nd), and use all other dimensions in the frame of the widget.
            /// The term 'reflection' refers to the fact that the value will appear in both in the frame and rendering of the widget.
            /// ```text
            /// r#"
            /// Frame..       | Rendering..
            /// (dim1, dim2)  | aggregate(dim2)"#
            /// ```
            ///
            /// Note, implemented for all numeric primitives.
            /// To use with pointer types (such as [`ordered_float::OrderedFloat`]), see features.
            pub fn reflect_2nd(&self) -> View2Full<Schema2<T, $T>> {
                let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| d.1 as f64);
                View2Full {
                    dataset: &self,
                    extractor,
                    value_header: self.schema.dimension_1.clone(),
                }
            }

            /// Take a regular view of this 2-dimensional dataset.
            /// Views are rendered differently by different widgets, but
            /// always have a frame on the left and a rendering on the right.
            ///
            /// This view will render the final dimension (2nd), and use all other dimensions in the frame of the widget.
            /// ```text
            /// r#"
            /// Frame..   | Rendering..
            /// (dim1, )  | aggregate(dim2)"#
            /// ```
            ///
            /// Note, implemented for all numeric primitives.
            /// To use with pointer types (such as [`ordered_float::OrderedFloat`]), see features.
            pub fn view_2nd(&self) -> View2Regular<Schema2<T, $T>> {
                let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| d.1 as f64);
                View2Regular {
                    dataset: &self,
                    extractor,
                }
            }

            /// Take a breakdown view of this 2-dimensional dataset.
            /// Views are rendered differently by different widgets, but
            /// always have a frame on the left and a rendering on the right.
            ///
            /// This view will render the breakdown of the final dimension (2nd), and use all other dimensions in the frame of the widget.
            /// ```text
            /// r#"
            /// Frame..   | Breakdown Rendering..        |
            /// (dim1, )  | breakdown(aggregate(dim2)).. |"#
            /// ```
            pub fn breakdown_2nd(&self) -> View2Breakdown<Schema2<T, $T>> {
                let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| d.1 as f64);
                View2Breakdown {
                    dataset: &self,
                    extractor,
                }
            }
        }
    };
}

impl_schema2_view!(f64, doc());
impl_schema2_view!(f32, doc(hidden));
impl_schema2_view!(isize, doc(hidden));
impl_schema2_view!(i128, doc(hidden));
impl_schema2_view!(i64, doc(hidden));
impl_schema2_view!(i32, doc(hidden));
impl_schema2_view!(i16, doc(hidden));
impl_schema2_view!(i8, doc(hidden));
impl_schema2_view!(usize, doc(hidden));
impl_schema2_view!(u128, doc(hidden));
impl_schema2_view!(u64, doc(hidden));
impl_schema2_view!(u32, doc(hidden));
impl_schema2_view!(u16, doc(hidden));
impl_schema2_view!(u8, doc(hidden));

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
macro_rules! impl_schema2_view_smart_pointer {
    ($T:ty) => {
        impl<T> Dataset<Schema2<T, $T>> {
            pub fn reflect_2nd(&self) -> View2Full<Schema2<T, $T>> {
                let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| (*d.1) as f64);
                View2Full {
                    dataset: &self,
                    extractor,
                    value_header: self.schema.dimension_0.clone(),
                }
            }

            pub fn view_2nd(&self) -> View2Regular<Schema2<T, $T>> {
                let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| (*d.1) as f64);
                View2Regular {
                    dataset: &self,
                    extractor,
                }
            }

            pub fn breakdown_2nd(&self) -> View2Breakdown<Schema2<T, $T>> {
                let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| (*d.1) as f64);
                View2Breakdown {
                    dataset: &self,
                    extractor,
                }
            }
        }
    };
}

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl_schema2_view_smart_pointer!(OrderedFloat<f64>);
#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl_schema2_view_smart_pointer!(OrderedFloat<f32>);

impl<T, U> Dataset<Schema2<T, U>> {
    /// Take a counting view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the occurrences of each dimensional vector, while using all dimensions in the frame of the widget.
    /// ```text
    /// r#"
    /// Frame..       | Rendering..
    /// (dim1, dim2)  | aggregate(count())"#
    /// ```
    pub fn count(&self) -> View2Full<Schema2<T, U>> {
        let extractor: Box<dyn Fn(&<Schema2<T, U> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View2Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a counting-breakdown view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the breakdown of the final dimension (2nd), and use all other dimensions in the frame of the widget.
    /// Rather than displaying the value of the final dimension, the occurrences of each dimensional vector are counted.
    /// ```text
    /// r#"
    /// Frame..   | Breakdown Rendering..           |
    /// (dim1, )  | breakdown(aggregate(count())).. |"#
    /// ```
    pub fn count_breakdown_2nd(&self) -> View2BreakdownCount<Schema2<T, U>> {
        View2BreakdownCount { dataset: &self }
    }
}

macro_rules! impl_schema3_view {
    ($T:ty, $attrs:meta) => {
        #[$attrs]
        #[allow(rustdoc::broken_intra_doc_links)]
        impl<T, U> Dataset<Schema3<T, U, $T>> {
            /// Take a reflective view of this 3-dimensional dataset.
            /// Views are rendered differently by different widgets, but
            /// always have a frame on the left and a rendering on the right.
            ///
            /// This view will render the final dimension (3rd), and use all other dimensions in the frame of the widget.
            /// The term 'reflection' refers to the fact that the value will appear in both in the frame and rendering of the widget.
            /// ```text
            /// r#"
            /// Frame..             | Rendering..
            /// (dim1, dim2, dim3)  | aggregate(dim3)"#
            /// ```
            ///
            /// Note, implemented for all numeric primitives.
            /// To use with pointer types (such as [`ordered_float::OrderedFloat`]), see features.
            pub fn reflect_3rd(&self) -> View3Full<Schema3<T, U, $T>> {
                let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| d.2 as f64);
                View3Full {
                    dataset: &self,
                    extractor,
                    value_header: self.schema.dimension_2.to_string(),
                }
            }

            /// Take a regular view of this 3-dimensional dataset.
            /// Views are rendered differently by different widgets, but
            /// always have a frame on the left and a rendering on the right.
            ///
            /// This view will render the final dimension (3rd), and use all other dimensions in the frame of the widget.
            /// ```text
            /// r#"
            /// Frame..       | Rendering..
            /// (dim1, dim2)  | aggregate(dim3)"#
            /// ```
            ///
            /// Note, implemented for all numeric primitives.
            /// To use with pointer types (such as [`ordered_float::OrderedFloat`]), see features.
            pub fn view_3rd(&self) -> View3Regular<Schema3<T, U, $T>> {
                let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| d.2 as f64);
                View3Regular {
                    dataset: &self,
                    extractor,
                }
            }

            /// Take a breakdown view of this 2-dimensional dataset.
            /// Views are rendered differently by different widgets, but
            /// always have a frame on the left and a rendering on the right.
            ///
            /// This view will render the breakdown of the final dimension (3rd), and use all other dimensions in the frame of the widget.
            /// ```text
            /// r#"
            /// Frame..   | Breakdown Rendering..        |
            /// (dim1, )  | breakdown(aggregate(dim2)).. |"#
            /// ```
            pub fn breakdown_3rd(&self) -> View3Breakdown<Schema3<T, U, $T>> {
                let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| d.2 as f64);
                View3Breakdown {
                    dataset: &self,
                    extractor,
                }
            }
        }
    };
}

impl_schema3_view!(f64, doc());
impl_schema3_view!(f32, doc(hidden));
impl_schema3_view!(isize, doc(hidden));
impl_schema3_view!(i128, doc(hidden));
impl_schema3_view!(i64, doc(hidden));
impl_schema3_view!(i32, doc(hidden));
impl_schema3_view!(i16, doc(hidden));
impl_schema3_view!(i8, doc(hidden));
impl_schema3_view!(usize, doc(hidden));
impl_schema3_view!(u128, doc(hidden));
impl_schema3_view!(u64, doc(hidden));
impl_schema3_view!(u32, doc(hidden));
impl_schema3_view!(u16, doc(hidden));
impl_schema3_view!(u8, doc(hidden));

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
macro_rules! impl_schema3_view_smart_pointer {
    ($T:ty) => {
        impl<T, U> Dataset<Schema3<T, U, $T>> {
            pub fn reflect_3rd(&self) -> View3Full<Schema3<T, U, $T>> {
                let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| (*d.2) as f64);
                View3Full {
                    dataset: &self,
                    extractor,
                    value_header: self.schema.dimension_2.to_string(),
                }
            }

            pub fn view_3rd(&self) -> View3Regular<Schema3<T, U, $T>> {
                let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| (*d.2) as f64);
                View3Regular {
                    dataset: &self,
                    extractor,
                }
            }

            pub fn breakdown_3rd(&self) -> View3Breakdown<Schema3<T, U, $T>> {
                let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                    Box::new(|d| (*d.2) as f64);
                View3Breakdown {
                    dataset: &self,
                    extractor,
                }
            }
        }
    };
}

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl_schema3_view_smart_pointer!(OrderedFloat<f64>);
#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl_schema3_view_smart_pointer!(OrderedFloat<f32>);

impl<T, U, V> Dataset<Schema3<T, U, V>> {
    /// Take a counting view of this 1-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the occurrences of each dimensional vector, while using all dimensions in the frame of the widget.
    /// ```text
    /// r#"
    /// Frame..             | Rendering..
    /// (dim1, dim2, dim3)  | aggregate(count())"#
    /// ```
    pub fn count(&self) -> View3Full<Schema3<T, U, V>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, V> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View3Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 2-dimensional dataset breaking down the 3rd column.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the breakdown of the final dimension (3rd), and use all other dimensions in the frame of the widget.
    /// ```text
    /// r#"
    /// Frame..       | Breakdown Rendering..           |
    /// (dim1, dim2)  | breakdown(aggregate(count())).. |"#
    /// ```
    pub fn count_breakdown_3rd(&self) -> View3BreakdownCount<Schema3<T, U, V>> {
        View3BreakdownCount { dataset: &self }
    }
}

impl<S: Schema> Dataset<S> {
    /// Build a dataset based for the provided schema.
    pub fn builder(schema: S) -> DatasetBuilder<S> {
        DatasetBuilder {
            schema,
            data: Vec::default(),
        }
    }
}

impl<S: Schema> DatasetBuilder<S> {
    /// Update this dataset with a data point `vector`.
    /// Use this method to add data via mutation.
    ///
    /// See also: [`DatasetBuilder::add`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schemas::one("Things");
    /// let mut builder = Dataset::builder(schema);
    /// builder.update((0, ));
    /// builder.update((0, ));
    /// builder.update((1, ));
    /// let dataset = builder.build();
    /// let view = dataset.count();
    ///
    /// let flat = BarChart::new(&view)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// 0       |**
    /// 1       |*"#);
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// [0, 1)  |**
    /// [1, 2]  |*"#);
    /// ```
    pub fn update(&mut self, vector: S::Dimensions) {
        self.data.push(vector);
    }

    /// Add a data point `vector` to this dataset.
    /// Use this method to add via method chaining.
    ///
    /// See also: [`DatasetBuilder::update`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schemas::one("Things");
    /// let dataset = Dataset::builder(schema)
    ///     .add((0, ))
    ///     .add((0, ))
    ///     .add((1, ))
    ///     .build();
    /// let view = dataset.count();
    ///
    /// let flat = BarChart::new(&view)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// 0       |**
    /// 1       |*"#);
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// [0, 1)  |**
    /// [1, 2]  |*"#);
    /// ```
    pub fn add(mut self, vector: S::Dimensions) -> Self {
        self.update(vector);
        self
    }

    /// Finalize the builder into a [`Dataset`].
    pub fn build(self) -> Dataset<S> {
        let DatasetBuilder { schema, data } = self;
        Dataset { schema, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Schemas;

    #[test]
    fn dataset_add() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let dataset = Dataset::builder(schema).add((1,)).add((2,)).add((3,));
        assert_eq!(dataset.data.len(), 3);
    }

    #[test]
    fn dataset_update() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let mut dataset = Dataset::builder(schema);
        dataset.update((1,));
        dataset.update((2,));
        dataset.update((3,));
        assert_eq!(dataset.data.len(), 3);
    }

    #[test]
    fn dataset_cast() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let dataset = Dataset::builder(schema)
            .add((1,))
            .add((2,))
            .add((3,))
            .build();
        let dataset_2d: Dataset<Schema2<i64, bool>> =
            dataset.recast(Schemas::two("abc", "is_even"), |i| (i.0, i.0 % 2 == 0));
        assert_eq!(dataset.data.len(), 3);
        assert_eq!(dataset_2d.data.len(), 3);
    }
}
