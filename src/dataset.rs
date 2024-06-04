use crate::{
    ReverseView2, Schema, Schema1, Schema2, Schema3, View1, View2, View2Breakdown2, View3,
    View3Breakdown2, View3Breakdown3,
};

/// A dataset in `flat`.
/// The same dataset may be observed through multiple views.
pub struct Dataset<S: Schema> {
    pub(crate) schema: S,
    pub(crate) data: Vec<S::Dimensions>,
}

/// A reference dataset in `flat`.
/// The same dataset may be observed through multiple views.
pub struct ReferenceDataset<'a, S: Schema> {
    pub(crate) schema: S,
    pub(crate) data: &'a Vec<S::Dimensions>,
}

// impl<'a, S: Schema> Dataset<S> {
//     pub fn downcast_2d<T, U>(&self) -> &'a Dataset<Schema2<T, U>> {
//         &Dataset {
//             schema: todo!(),
//             data: &self.data,
//         }
//     }
// }

impl Dataset<Schema1<f64>> {
    /// Take a regular view of this 1-dimensional dataset.
    /// * primary dimension: `T`
    pub fn view(&self) -> View1<Schema1<f64>> {
        let extractor: Box<dyn Fn(&<Schema1<f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.0);
        View1 {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

impl Dataset<Schema1<i64>> {
    /// Take a regular view of this 1-dimensional dataset.
    /// * primary dimension: `T`
    pub fn view(&self) -> View1<Schema1<i64>> {
        let extractor: Box<dyn Fn(&<Schema1<i64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.0 as f64);
        View1 {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

impl<T> Dataset<Schema1<T>> {
    /// Take a counting view of this 1-dimensional dataset.
    /// * primary dimension: `T`
    /// * secondary dimension: `1`
    pub fn view_count(&self) -> View1<Schema1<T>> {
        let extractor: Box<dyn Fn(&<Schema1<T> as Schema>::Dimensions) -> f64> = Box::new(|_| 1f64);
        View1 {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }
}

impl<T> Dataset<Schema2<T, f64>> {
    /// Take a regular view of this 2-dimensional dataset.
    /// * primary dimension: `T`
    /// * secondary dimension: `U`
    /// * breakdown dimension: N/A
    pub fn view(&self) -> View2<Schema2<T, f64>> {
        let extractor: Box<dyn Fn(&<Schema2<T, f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.1);
        View2 {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_1.clone(),
        }
    }
}

impl<U> Dataset<Schema2<f64, U>> {
    /// Take a reverse view of this 2-dimensional dataset.
    /// * primary dimension: `U`
    /// * secondary dimension: `T`
    /// * breakdown dimension: N/A
    pub fn reverse_view(&self) -> ReverseView2<Schema2<f64, U>> {
        let extractor: Box<dyn Fn(&<Schema2<f64, U> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.0);
        ReverseView2 {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

impl<U> Dataset<Schema2<i64, U>> {
    /// Take a reverse view of this 2-dimensional dataset.
    /// * primary dimension: `U`
    /// * secondary dimension: `T`
    /// * breakdown dimension: N/A
    pub fn reverse_view(&self) -> ReverseView2<Schema2<i64, U>> {
        let extractor: Box<dyn Fn(&<Schema2<i64, U> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.0 as f64);
        ReverseView2 {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

impl<T, U> Dataset<Schema2<T, U>> {
    pub fn view_count(&self) -> View2<Schema2<T, U>> {
        let extractor: Box<dyn Fn(&<Schema2<T, U> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View2 {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 2-dimensional dataset using the 2nd column as the breakdown.
    /// * primary dimension: `T`
    /// * secondary dimension: N/A
    /// * breakdown dimension: `U`
    pub fn view_breakdown2(&self) -> View2Breakdown2<Schema2<T, U>> {
        View2Breakdown2 { dataset: &self }
    }
}

impl<T, U> Dataset<Schema3<T, U, f64>> {
    /// Take a regular view of this 3-dimensional dataset.
    /// * primary dimension: `T`
    /// * secondary dimension: `U`
    /// * tertiary dimension: `V`
    /// * breakdown dimension: N/A
    pub fn view(&self) -> View3<Schema3<T, U, f64>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, f64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.2);
        View3 {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_2.to_string(),
        }
    }
}

impl<T, U> Dataset<Schema3<T, U, i64>> {
    /// Take a regular view of this 3-dimensional dataset.
    /// * primary dimension: `T`
    /// * secondary dimension: `U`
    /// * tertiary dimension: `V`
    /// * breakdown dimension: N/A
    pub fn view(&self) -> View3<Schema3<T, U, i64>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, i64> as Schema>::Dimensions) -> f64> =
            Box::new(|d| d.2 as f64);
        View3 {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_2.to_string(),
        }
    }
}

impl<T, U, V> Dataset<Schema3<T, U, V>> {
    pub fn view_count(&self) -> View3<Schema3<T, U, V>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, V> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View3 {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 3-dimensional dataset using the 2nd column as the breakdown.
    /// * primary dimension: `T`
    /// * secondary dimension: `V`
    /// * tertiary dimension: N/A
    /// * breakdown dimension: `U`
    pub fn view_breakdown2(&self) -> View3Breakdown2<Schema3<T, U, V>> {
        View3Breakdown2 { dataset: &self }
    }

    /// Take a view of this 3-dimensional dataset using the 3rd column as the breakdown.
    /// * primary dimension: `T`
    /// * secondary dimension: `U`
    /// * tertiary dimension: N/A
    /// * breakdown dimension: `V`
    pub fn view_breakdown3(&self) -> View3Breakdown3<Schema3<T, U, V>> {
        View3Breakdown3 { dataset: &self }
    }
}

impl<S: Schema> Dataset<S> {
    /// Build a dataset based for the provided schema.
    pub fn builder(schema: S) -> Self {
        Self {
            schema,
            data: Vec::default(),
        }
    }

    /// Update this dataset with a data point `vector`.
    /// Use this method to add data via mutation.
    ///
    /// See also: [`Dataset::add`].
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
    /// let view = builder.view();
    ///
    /// let flat = BarChart::new(&view)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Things)
    /// 0       |
    /// 1       |*"#);
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Things)
    /// [0, 1)  |
    /// [1, 2]  |*"#);
    /// ```
    pub fn update(&mut self, vector: S::Dimensions) {
        self.data.push(vector);
    }

    /// Add a data point `vector` to this dataset.
    /// Use this method to add via method chaining.
    ///
    /// See also: [`Dataset::update`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schemas::one("Things");
    /// let builder = Dataset::builder(schema)
    ///     .add((0, ))
    ///     .add((0, ))
    ///     .add((1, ));
    /// let view = builder.view();
    ///
    /// let flat = BarChart::new(&view)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Things)
    /// 0       |
    /// 1       |*"#);
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Things)
    /// [0, 1)  |
    /// [1, 2]  |*"#);
    /// ```
    pub fn add(mut self, vector: S::Dimensions) -> Self {
        self.update(vector);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BarChart, Histogram, Render, Schemas};

    #[test]
    fn builder_add() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let builder = Dataset::builder(schema).add((1,)).add((2,)).add((3,));
        assert_eq!(builder.data.len(), 3);
    }

    #[test]
    fn builder_update() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let mut builder = Dataset::builder(schema);
        builder.update((1,));
        builder.update((2,));
        builder.update((3,));
        assert_eq!(builder.data.len(), 3);
    }
}
