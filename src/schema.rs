use std::marker::PhantomData;

/// The internal trait which defines a schema.
/// Consumers should not implement this trait.
#[doc(hidden)]
pub trait Schema {
    type Dimensions;
}

/// Starting point for defining the dimensions of a dataset.
///
/// ```ignore
/// use flat::Schema;
///
/// let my_schema = Schemas::two("dimension_1", "dimension_2", "header");
/// ```
pub struct Schemas;

impl Schemas {
    /// Define a 1-dimensional schema.
    /// The names of the dimensions are specified in order, with the final name indicating the name of the values.
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// // The inventory of animals in a zoo.
    /// // This dataset has 2 bears, and 1 tiger.
    /// let schema = Schemas::one("Animals", "Count");
    /// let builder = Dataset::builder(schema)
    ///     .add(("Bear", ), 2)
    ///     .add(("Tiger", ), 1);
    ///
    /// // An attribute dataset measuring the zoo animal heights.
    /// // This dataset has a bear which measures height "10", and another which measures height "11".
    /// // There is also a tiger which measures height "5".
    /// let schema = Schemas::one("Animals", "Height");
    /// let builder = Dataset::builder(schema)
    ///     .add(("Bear", ), 10)
    ///     .add(("Bear", ), 11)
    ///     .add(("Tiger", ), 5);
    /// ```
    pub fn one<T>(dimension_1: impl Into<String>, values: impl Into<String>) -> Schema1<T> {
        Schema1 {
            one: PhantomData,
            dimension_1: dimension_1.into(),
            values: values.into(),
        }
    }

    /// Define a 2-dimensional schema.
    /// The names of the dimensions are specified in order, with the final name indicating the name of the values.
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// // A dataset counting the zoo animal heights.
    /// // This dataset has a bear which measures height "10", and another which measures height "11".
    /// // There is also a tiger which measures height "5".
    /// let schema = Schemas::two("Animals", "Height", "Count");
    /// let builder = Dataset::builder(schema)
    ///     .add(("Bear", 10), 1)
    ///     .add(("Bear", 11), 1)
    ///     .add(("Tiger", 5), 1);
    /// ```
    pub fn two<T, U>(
        dimension_1: impl Into<String>,
        dimension_2: impl Into<String>,
        values: impl Into<String>,
    ) -> Schema2<T, U> {
        Schema2 {
            one: PhantomData,
            dimension_1: dimension_1.into(),
            two: PhantomData,
            dimension_2: dimension_2.into(),
            values: values.into(),
        }
    }

    /// Define a 3-dimensional schema.
    /// The names of the dimensions are specified in order, with the final name indicating the name of the values.
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// // A dataset with zoo animal heights and their enclosures.
    /// // This dataset has a bear which measures height "10", and another which measures height "11".
    /// // There is also a tiger which measures height "5".
    /// // The bears live in Pen01, while the tiger lives in Pen02.
    /// let schema = Schemas::three("Animals", "Height", "Enclosure", "Count");
    /// let builder = Dataset::builder(schema)
    ///     .add(("Bear", 10, "Pen01"), 1)
    ///     .add(("Bear", 11, "Pen01"), 1)
    ///     .add(("Tiger", 5, "Pen02"), 1);
    /// ```
    pub fn three<T, U, V>(
        dimension_1: impl Into<String>,
        dimension_2: impl Into<String>,
        dimension_3: impl Into<String>,
        values: impl Into<String>,
    ) -> Schema3<T, U, V> {
        Schema3 {
            one: PhantomData,
            dimension_1: dimension_1.into(),
            two: PhantomData,
            dimension_2: dimension_2.into(),
            three: PhantomData,
            dimension_3: dimension_3.into(),
            values: values.into(),
        }
    }
}

/// A 1-dimensional schema.
///
/// See also: [`Schemas::one`]
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema1<usize> = Schemas::one("dimension_1", "header");
/// ```
pub struct Schema1<T> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) values: String,
}

impl<T> Schema for Schema1<T> {
    type Dimensions = (T,);
}

/// A 2-dimensional schema.
///
/// See also: [`Schemas::two`]
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema2<usize, f64> = Schemas::two("dimension_1", "dimension_2", "header");
/// ```
pub struct Schema2<T, U> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
    pub(crate) values: String,
}

impl<T, U> Schema for Schema2<T, U> {
    type Dimensions = (T, U);
}

/// A 3-dimensional schema.
///
/// See also: [`Schemas::three`]
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema3<usize, f64, bool> = Schemas::three("dimension_1", "dimension_2", "dimension_3", "header");
/// ```
pub struct Schema3<T, U, V> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
    pub(crate) three: PhantomData<V>,
    pub(crate) dimension_3: String,
    pub(crate) values: String,
}

impl<T, U, V> Schema for Schema3<T, U, V> {
    type Dimensions = (T, U, V);
}

// TODO
// pub struct Schema4<T, U, V, W>;
// pub struct Schema5<T, U, V, W, X>;
// pub struct Schema6<T, U, V, W, X, Y>;
// pub struct Schema7<T, U, V, W, X, Y, Z>;
