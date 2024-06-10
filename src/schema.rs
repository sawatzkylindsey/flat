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
/// let my_schema = Schemas::two("dimension_0", "dimension_1");
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
    /// let schema = Schemas::one("Animals");
    /// let dataset = Dataset::builder(schema)
    ///     //   (Animal,)
    ///     .add(("Bear",))
    ///     .add(("Bear",))
    ///     .add(("Tiger",))
    ///     .build();
    /// ```
    pub fn one<T>(dimension_0: impl Into<String>) -> Schema1<T> {
        Schema1 {
            phantom_0: PhantomData,
            dimension_0: dimension_0.into(),
        }
    }

    /// Define a 2-dimensional schema.
    /// The names of the dimensions are specified in order, with the final name indicating the name of the values.
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// // A dataset describing the zoo animal heights.
    /// // This dataset has a bear which measures height "10", and another which measures height "11".
    /// // There is also a tiger which measures height "5".
    /// let schema = Schemas::two("Animal", "Height");
    /// let dataset = Dataset::builder(schema)
    ///     //   (Animal, Height)
    ///     .add(("Bear", 10))
    ///     .add(("Bear", 11))
    ///     .add(("Tiger", 5))
    ///     .build();
    /// ```
    pub fn two<T, U>(
        dimension_0: impl Into<String>,
        dimension_1: impl Into<String>,
    ) -> Schema2<T, U> {
        Schema2 {
            phantom_0: PhantomData,
            dimension_0: dimension_0.into(),
            phantom_1: PhantomData,
            dimension_1: dimension_1.into(),
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
    /// let schema = Schemas::three("Animals", "Height", "Enclosure");
    /// let dataset = Dataset::builder(schema)
    ///     //   (Animal, Height, Enclosure)
    ///     .add(("Bear", 10, "Pen01"))
    ///     .add(("Bear", 11, "Pen01"))
    ///     .add(("Tiger", 5, "Pen02"))
    ///     .build();
    /// ```
    pub fn three<T, U, V>(
        dimension_0: impl Into<String>,
        dimension_1: impl Into<String>,
        dimension_2: impl Into<String>,
    ) -> Schema3<T, U, V> {
        Schema3 {
            phantom_0: PhantomData,
            dimension_0: dimension_0.into(),
            phantom_1: PhantomData,
            dimension_1: dimension_1.into(),
            phantom_2: PhantomData,
            dimension_2: dimension_2.into(),
        }
    }
}

/// A 1-dimensional schema.
/// Constructed via [`Schemas`].
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema1<usize> = Schemas::one("dimension_0");
/// ```
#[doc(hidden)]
pub struct Schema1<T> {
    pub(crate) phantom_0: PhantomData<T>,
    pub(crate) dimension_0: String,
}

impl<T> Schema for Schema1<T> {
    type Dimensions = (T,);
}

/// A 2-dimensional schema.
/// Constructed via [`Schemas`].
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema2<usize, f64> = Schemas::two("dimension_0", "dimension_1");
/// ```
#[doc(hidden)]
pub struct Schema2<T, U> {
    pub(crate) phantom_0: PhantomData<T>,
    pub(crate) dimension_0: String,
    pub(crate) phantom_1: PhantomData<U>,
    pub(crate) dimension_1: String,
}

impl<T, U> Schema for Schema2<T, U> {
    type Dimensions = (T, U);
}

/// A 3-dimensional schema.
/// Constructed via [`Schemas`].
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema3<usize, f64, bool> = Schemas::three("dimension_0", "dimension_1", "dimension_2");
/// ```
#[doc(hidden)]
pub struct Schema3<T, U, V> {
    pub(crate) phantom_0: PhantomData<T>,
    pub(crate) dimension_0: String,
    pub(crate) phantom_1: PhantomData<U>,
    pub(crate) dimension_1: String,
    pub(crate) phantom_2: PhantomData<V>,
    pub(crate) dimension_2: String,
}

impl<T, U, V> Schema for Schema3<T, U, V> {
    type Dimensions = (T, U, V);
}

// TODO
// pub struct Schema4<T, U, V, W>;
// pub struct Schema5<T, U, V, W, X>;
// pub struct Schema6<T, U, V, W, X, Y>;
// pub struct Schema7<T, U, V, W, X, Y, Z>;
