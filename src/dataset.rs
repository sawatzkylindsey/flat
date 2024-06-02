use crate::{
    ReverseView2, Schema, Schema1, Schema2, Schema3, View1, View2, View2Breakdown2, View3,
    View3Breakdown2, View3Breakdown3,
};

/// A dataset in `flat`.
/// The same dataset may be observed through multiple views.
pub struct Dataset<S: Schema> {
    pub(crate) schema: S,
    pub(crate) data: Vec<(S::Dimensions, f64)>,
}

impl<T> Dataset<Schema1<T>> {
    /// Take a regular view of this 1-dimensional dataset.
    /// * primary dimension: `T`
    /// * secondary dimension: N/A
    /// * breakdown dimension: N/A
    pub fn view(&self) -> View1<Schema1<T>> {
        View1 { dataset: &self }
    }
}

impl<T, U> Dataset<Schema2<T, U>> {
    /// Take a regular view of this 2-dimensional dataset.
    /// * primary dimension: `T`
    /// * secondary dimension: `U`
    /// * breakdown dimension: N/A
    pub fn view(&self) -> View2<Schema2<T, U>> {
        View2 { dataset: &self }
    }

    /// Take a reverse view of this 2-dimensional dataset.
    /// * primary dimension: `U`
    /// * secondary dimension: `T`
    /// * breakdown dimension: N/A
    pub fn reverse_view(&self) -> ReverseView2<Schema2<T, U>> {
        ReverseView2 { dataset: &self }
    }

    /// Take a view of this 2-dimensional dataset using the 2nd column as the breakdown.
    /// * primary dimension: `T`
    /// * secondary dimension: N/A
    /// * breakdown dimension: `U`
    pub fn view_breakdown2(&self) -> View2Breakdown2<Schema2<T, U>> {
        View2Breakdown2 { dataset: &self }
    }
}

impl<T, U, V> Dataset<Schema3<T, U, V>> {
    /// Take a regular view of this 3-dimensional dataset.
    /// * primary dimension: `T`
    /// * secondary dimension: `U`
    /// * tertiary dimension: `V`
    /// * breakdown dimension: N/A
    pub fn view(&self) -> View3<Schema3<T, U, V>> {
        View3 { dataset: &self }
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

    /// Update this histogram with a data point to (key, value).
    /// Use this method to add data via mutation.
    ///
    /// See also: [`Dataset::add`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schemas::one("Things", "Count");
    /// let mut builder = Dataset::builder(schema);
    /// builder.update((0, ), 0);
    /// builder.update((0, ), 1);
    /// builder.update((1, ), 4);
    /// let view = builder.view();
    ///
    /// let flat = BarChart::new(&view)
    ///     .render(Render::default());
    /// println!("{flat}");
    /// // Output (modified for alignment)
    /// r#"Things  |Count
    ///    0       |*
    ///    1       |****"#;
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// println!("{flat}");
    /// // Output (modified for alignment)
    /// r#"Things  |Count
    ///    [0, 1)  |*
    ///    [1, 2]  |****"#;
    /// ```
    pub fn update(&mut self, key: S::Dimensions, value: impl Into<f64>) {
        self.data.push((key, value.into()));
    }

    /// Add a.values point to (key, value) to this histogram.
    /// Use this method to add.values via method chaining.
    ///
    /// See also: [`Dataset::update`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schemas::one("Things", "Count");
    /// let builder = Dataset::builder(schema)
    ///     .add((0,), 0)
    ///     .add((0,), 1)
    ///     .add((1,), 4);
    /// let view = builder.view();
    ///
    /// let flat = BarChart::new(&view)
    ///     .render(Render::default());
    /// println!("{flat}");
    /// // Output (modified for alignment)
    /// r#"Things  |Count
    ///    0       |*
    ///    1       |****"#;
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// println!("{flat}");
    /// // Output (modified for alignment)
    /// r#"Things  |Count
    ///    [0, 1)  |*
    ///    [1, 2]  |****"#;
    /// ```
    pub fn add(mut self, key: S::Dimensions, value: impl Into<f64>) -> Self {
        self.update(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Schemas;

    #[test]
    fn builder_add() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let builder = Dataset::builder(schema)
            .add((1,), -1)
            .add((2,), 0)
            .add((3,), 1);
        assert_eq!(builder.data.len(), 3);
    }

    #[test]
    fn builder_update() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let mut builder = Dataset::builder(schema);
        builder.update((1,), -1);
        builder.update((2,), 0);
        builder.update((3,), 1);
        assert_eq!(builder.data.len(), 3);
    }
}
