use crate::{
    Schema, Schema1, Schema2, Schema3, View1Full, View2Breakdown2, View2Full, View2Regular,
    View3Breakdown2, View3Breakdown3, View3Full, View3Regular,
};
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

impl Dataset<Schema1<f64>> {
    /// Take a reflective view of this 1-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, )     | aggregate(dim1)"#
    /// ```
    pub fn reflective_view(&self) -> View1Full<Schema1<f64>> {
        let extractor: Box<dyn Fn(&<Schema1<f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.0);
        View1Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

impl Dataset<Schema1<i64>> {
    /// Take a reflective view of this 1-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, )     | aggregate(dim1)"#
    /// ```
    pub fn reflective_view(&self) -> View1Full<Schema1<i64>> {
        let extractor: Box<dyn Fn(&<Schema1<i64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.0 as f64);
        View1Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl Dataset<Schema1<OrderedFloat<f64>>> {
    /// Take a reflective view of this 1-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, )     | aggregate(dim1)"#
    /// ```
    pub fn reflective_view(&self) -> View1Full<Schema1<OrderedFloat<f64>>> {
        let extractor: Box<dyn Fn(&<Schema1<OrderedFloat<f64>> as Schema>::Dimensions) -> f64> =
            Box::new(|d| *d.0);
        View1Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

impl<T> Dataset<Schema1<T>> {
    /// Take a counting view of this 1-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'counting' refers to the fact that we count the occurrences of each dimensional vector.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, )     | aggregate(count())"#
    /// ```
    pub fn counting_view(&self) -> View1Full<Schema1<T>> {
        let extractor: Box<dyn Fn(&<Schema1<T> as Schema>::Dimensions) -> f64> = Box::new(|_| 1f64);
        View1Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }
}

impl<T> Dataset<Schema2<T, f64>> {
    /// Take a reflective view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, dim2) | aggregate(dim2)"#
    /// ```
    pub fn reflective_view(&self) -> View2Full<Schema2<T, f64>> {
        let extractor: Box<dyn Fn(&<Schema2<T, f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.1);
        View2Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_1.clone(),
        }
    }

    /// Take a regular view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, )     | aggregate(dim2)"#
    /// ```
    pub fn view_2nd(&self) -> View2Regular<Schema2<T, f64>> {
        let extractor: Box<dyn Fn(&<Schema2<T, f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.1);
        View2Regular {
            dataset: &self,
            extractor,
        }
    }
}

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl<T> Dataset<Schema2<T, OrderedFloat<f64>>> {
    /// Take a reflective view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, dim2) | aggregate(dim2)"#
    /// ```
    pub fn reflective_view(&self) -> View2Full<Schema2<T, OrderedFloat<f64>>> {
        let extractor: Box<dyn Fn(&<Schema2<T, OrderedFloat<f64>> as Schema>::Dimensions) -> f64> =
            Box::new(|d| *d.1);
        View2Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }

    /// Take a regular view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, )     | aggregate(dim2)"#
    /// ```
    pub fn view_2nd(&self) -> View2Regular<Schema2<T, OrderedFloat<f64>>> {
        let extractor: Box<dyn Fn(&<Schema2<T, OrderedFloat<f64>> as Schema>::Dimensions) -> f64> =
            Box::new(|d| *d.1);
        View2Regular {
            dataset: &self,
            extractor,
        }
    }
}

impl<T> Dataset<Schema2<T, i64>> {
    /// Take a reflective view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, dim2) | aggregate(dim2)"#
    /// ```
    pub fn reflective_view(&self) -> View2Full<Schema2<T, i64>> {
        let extractor: Box<dyn Fn(&<Schema2<T, i64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.1 as f64);
        View2Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }

    /// Take a regular view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, )     | aggregate(dim2)"#
    /// ```
    pub fn view_2nd(&self) -> View2Regular<Schema2<T, i64>> {
        let extractor: Box<dyn Fn(&<Schema2<T, i64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.1 as f64);
        View2Regular {
            dataset: &self,
            extractor,
        }
    }
}

impl<T, U> Dataset<Schema2<T, U>> {
    /// Take a counting view of this 2-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'counting' refers to the fact that we count the occurrences of each dimensional vector.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, dim2) | aggregate(count())"#
    /// ```
    pub fn counting_view(&self) -> View2Full<Schema2<T, U>> {
        let extractor: Box<dyn Fn(&<Schema2<T, U> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View2Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 2-dimensional dataset breaking down the 2nd column.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the breakdown of the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// Notice, a breakdown always 'counts' the values.
    /// ```text
    /// r#"
    /// Dimensions.. | Breakdown Rendering.. |
    /// (dim1, )     | aggregate(count())    |"#
    /// ```
    pub fn breakdown_2nd(&self) -> View2Breakdown2<Schema2<T, U>> {
        View2Breakdown2 { dataset: &self }
    }
}

impl<T, U> Dataset<Schema3<T, U, f64>> {
    /// Take a reflective view of this 3-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions..       | Rendering
    /// (dim1, dim2, dim3) | aggregate(dim3)"#
    /// ```
    pub fn reflective_view(&self) -> View3Full<Schema3<T, U, f64>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.2);
        View3Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_2.to_string(),
        }
    }

    /// Take a regular view of this 3-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, dim2) | aggregate(dim3)"#
    /// ```
    pub fn view_3rd(&self) -> View3Regular<Schema3<T, U, f64>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.2);
        View3Regular {
            dataset: &self,
            extractor,
        }
    }
}

#[cfg(any(feature = "impls_ordered_float", feature = "all"))]
impl<T, U> Dataset<Schema3<T, U, OrderedFloat<f64>>> {
    /// Take a reflective view of this 3-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions..       | Rendering
    /// (dim1, dim2, dim3) | aggregate(dim3)"#
    /// ```
    pub fn reflective_view(&self) -> View3Full<Schema3<T, U, OrderedFloat<f64>>> {
        let extractor: Box<
            dyn Fn(&<Schema3<T, U, OrderedFloat<f64>> as Schema>::Dimensions) -> f64,
        > = Box::new(|d| *d.2);
        View3Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_2.to_string(),
        }
    }

    /// Take a regular view of this 3-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, dim2) | aggregate(dim3)"#
    /// ```
    pub fn view_3rd(&self) -> View3Regular<Schema3<T, U, OrderedFloat<f64>>> {
        let extractor: Box<
            dyn Fn(&<Schema3<T, U, OrderedFloat<f64>> as Schema>::Dimensions) -> f64,
        > = Box::new(|d| *d.2);
        View3Regular {
            dataset: &self,
            extractor,
        }
    }
}

impl<T, U> Dataset<Schema3<T, U, i64>> {
    /// Take a reflective view of this 3-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'reflection' refers to the fact that the value will appear in both in the dimensional and rendering aspects of the widget.
    /// ```text
    /// r#"
    /// Dimensions..       | Rendering
    /// (dim1, dim2, dim3) | aggregate(dim3)"#
    /// ```
    pub fn reflective_view(&self) -> View3Full<Schema3<T, U, i64>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, i64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.2 as f64);
        View3Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_2.to_string(),
        }
    }

    /// Take a regular view of this 3-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// ```text
    /// r#"
    /// Dimensions.. | Rendering
    /// (dim1, dim2) | aggregate(dim3)"#
    /// ```
    pub fn view_3rd(&self) -> View3Regular<Schema3<T, U, i64>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, i64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.2 as f64);
        View3Regular {
            dataset: &self,
            extractor,
        }
    }
}

impl<T, U, V> Dataset<Schema3<T, U, V>> {
    /// Take a counting view of this 1-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// The term 'counting' refers to the fact that we count the occurrences of each dimensional vector.
    /// ```text
    /// r#"
    /// Dimensions..       | Rendering
    /// (dim1, dim2, dim3) | aggregate(count())"#
    /// ```
    pub fn counting_view(&self) -> View3Full<Schema3<T, U, V>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, V> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View3Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 3-dimensional dataset breaking down the 2nd column.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the breakdown of the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// Notice, a breakdown always 'counts' the values.
    /// ```text
    /// r#"
    /// Dimensions.. | Breakdown Rendering.. |
    /// (dim1, dim3) | aggregate(count())    |"#
    /// ```
    pub fn breakdown_2nd(&self) -> View3Breakdown2<Schema3<T, U, V>> {
        View3Breakdown2 { dataset: &self }
    }

    /// Take a view of this 2-dimensional dataset breaking down the 3rd column.
    /// Views are rendered differently by different widgets, but
    /// always have a dimensional component on the left and a rendering component on the right.
    ///
    /// This view will render the breakdown of the final dimension, and use the other dimensions in the dimensional aspect of the widget.
    /// Notice, a breakdown always 'counts' the values.
    /// ```text
    /// r#"
    /// Dimensions.. | Breakdown Rendering.. |
    /// (dim1, dim2) | aggregate(count())    |"#
    /// ```
    pub fn breakdown_3rd(&self) -> View3Breakdown3<Schema3<T, U, V>> {
        View3Breakdown3 { dataset: &self }
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
    /// let view = dataset.counting_view();
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
    /// let view = dataset.counting_view();
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
