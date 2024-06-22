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
    /// let schema = Schemas::one("Animal");
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
    /// let schema = Schemas::three("Animal", "Height", "Enclosure");
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

    /// Define a 4-dimensional schema.
    /// The names of the dimensions are specified in order, with the final name indicating the name of the values.
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// // A dataset with zoo animal heights and their enclosures.
    /// // This dataset has a bear which measures height "10", and another which measures height "11".
    /// // There is also a tiger which measures height "5".
    /// // The bears live in Pen01 (North East quadrant), while the tiger lives in Pen02 (South West quadrant).
    /// let schema = Schemas::four("Animal", "Height", "Enclosure", "Quadrant");
    /// let dataset = Dataset::builder(schema)
    ///     //   (Animal, Height, Enclosure, Quadrant)
    ///     .add(("Bear", 10, "Pen01", "NorthEast"))
    ///     .add(("Bear", 11, "Pen01", "NorthEast"))
    ///     .add(("Tiger", 5, "Pen02", "SouthWest"))
    ///     .build();
    /// ```
    pub fn four<T, U, V, W>(
        dimension_0: impl Into<String>,
        dimension_1: impl Into<String>,
        dimension_2: impl Into<String>,
        dimension_3: impl Into<String>,
    ) -> Schema4<T, U, V, W> {
        Schema4 {
            phantom_0: PhantomData,
            dimension_0: dimension_0.into(),
            phantom_1: PhantomData,
            dimension_1: dimension_1.into(),
            phantom_2: PhantomData,
            dimension_2: dimension_2.into(),
            phantom_3: PhantomData,
            dimension_3: dimension_3.into(),
        }
    }
}

#[doc(hidden)]
pub struct Schema1<T> {
    pub(crate) phantom_0: PhantomData<T>,
    pub(crate) dimension_0: String,
}

impl<T> Schema for Schema1<T> {
    type Dimensions = (T,);
}

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

#[doc(hidden)]
#[allow(unused)]
pub struct Schema4<T, U, V, W> {
    pub(crate) phantom_0: PhantomData<T>,
    pub(crate) dimension_0: String,
    pub(crate) phantom_1: PhantomData<U>,
    pub(crate) dimension_1: String,
    pub(crate) phantom_2: PhantomData<V>,
    pub(crate) dimension_2: String,
    pub(crate) phantom_3: PhantomData<W>,
    pub(crate) dimension_3: String,
}

impl<T, U, V, W> Schema for Schema4<T, U, V, W> {
    type Dimensions = (T, U, V, W);
}

// TODO
// pub struct Schema5<T, U, V, W, X>;
// pub struct Schema6<T, U, V, W, X, Y>;
// pub struct Schema7<T, U, V, W, X, Y, Z>;
