use std::marker::PhantomData;

/// Starting point for specifying the dimensions of a dataset.
///
/// ```ignore
/// use flat::Schema;
///
/// let my_dimensions = Schema::two("dimension_1", "dimension_2").values("header");
/// ```
pub struct Schema;

#[doc(hidden)]
pub struct InterimSchema1<T> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
}

/// A one-dimensional schema.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema1<usize> = Schema::one("dimension_1").values("header");
/// ```
pub struct Schema1<T> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) values: String,
}

#[doc(hidden)]
pub struct InterimSchema2<T, U> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
}

/// A two-dimensional schema.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema2<usize, f64> = Schema::two("dimension_1", "dimension_2").values("header");
/// ```
pub struct Schema2<T, U> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
    pub(crate) values: String,
}

/// A two-dimensional schema with a breakdown on the 2nd dimension.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema2Breakdown2<usize, f64> = Schema::two("dimension_1", "dimension_2").breakdown_2nd();
/// ```
pub struct Schema2Breakdown2<T, U> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
}

#[doc(hidden)]
pub struct InterimSchema3<T, U, V> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
    pub(crate) three: PhantomData<V>,
    pub(crate) dimension_3: String,
}

/// A three-dimensional schema.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema3<usize, f64, bool> = Schema::three("dimension_1", "dimension_2", "dimension_3").values("header");
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

/// A three-dimensional schema with a breakdown on the 2nd dimension
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema3Breakdown2<usize, f64, bool> = Schema::three("dimension_1", "dimension_2", "dimension_3").breakdown_2nd();
/// ```
pub struct Schema3Breakdown2<T, U, V> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
    pub(crate) three: PhantomData<V>,
    pub(crate) dimension_3: String,
}

/// A three-dimensional schema with a breakdown on the 3rd dimension
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let my_dimensions: Schema3Breakdown3<usize, f64, bool> = Schema::three("dimension_1", "dimension_2", "dimension_3").breakdown_3rd();
/// ```
pub struct Schema3Breakdown3<T, U, V> {
    pub(crate) one: PhantomData<T>,
    pub(crate) dimension_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) dimension_2: String,
    pub(crate) three: PhantomData<V>,
    pub(crate) dimension_3: String,
}

// TODO
// pub struct Schema4<T, U, V, W>;
// pub struct Schema5<T, U, V, W, X>;
// pub struct Schema6<T, U, V, W, X, Y>;
// pub struct Schema7<T, U, V, W, X, Y, Z>;

impl Schema {
    /// Create an interim schema for one-dimensional data.
    /// The interim schema must be finalized by either `values(..)` or `breakdown_ORDINAL()`.
    pub fn one<T>(dimension_1: impl Into<String>) -> InterimSchema1<T> {
        InterimSchema1 {
            one: PhantomData,
            dimension_1: dimension_1.into(),
        }
    }

    /// Create an interim schema for two-dimensional data.
    /// The interim schema must be finalized by either `values(..)` or `breakdown_ORDINAL()`.
    pub fn two<T, U>(
        dimension_1: impl Into<String>,
        dimension_2: impl Into<String>,
    ) -> InterimSchema2<T, U> {
        InterimSchema2 {
            one: PhantomData,
            dimension_1: dimension_1.into(),
            two: PhantomData,
            dimension_2: dimension_2.into(),
        }
    }

    /// Create an interim schema for three-dimensional data.
    /// The interim schema must be finalized by either `values(..)` or `breakdown_ORDINAL()`.
    pub fn three<T, U, V>(
        dimension_1: impl Into<String>,
        dimension_2: impl Into<String>,
        dimension_3: impl Into<String>,
    ) -> InterimSchema3<T, U, V> {
        InterimSchema3 {
            one: PhantomData,
            dimension_1: dimension_1.into(),
            two: PhantomData,
            dimension_2: dimension_2.into(),
            three: PhantomData,
            dimension_3: dimension_3.into(),
        }
    }
}

impl<T> InterimSchema1<T> {
    /// Specify the name of the values' header.
    pub fn values(self, header: impl Into<String>) -> Schema1<T> {
        let InterimSchema1 { one, dimension_1 } = self;
        Schema1 {
            one,
            dimension_1,
            values: header.into(),
        }
    }
}

impl<T, U> InterimSchema2<T, U> {
    /// Specify the name of the values' header.
    pub fn values(self, header: impl Into<String>) -> Schema2<T, U> {
        let InterimSchema2 {
            one,
            dimension_1,
            two,
            dimension_2,
        } = self;
        Schema2 {
            one,
            dimension_1,
            two,
            dimension_2,
            values: header.into(),
        }
    }

    /// Use the 2nd dimension as the breakdown dimension.
    pub fn breakdown_2nd(self) -> Schema2Breakdown2<T, U> {
        let InterimSchema2 {
            one,
            dimension_1,
            two,
            dimension_2,
            ..
        } = self;
        Schema2Breakdown2 {
            one,
            dimension_1,
            two,
            dimension_2,
        }
    }
}

impl<T, U, V> InterimSchema3<T, U, V> {
    /// Specify the name of the values' header.
    pub fn values(self, header: impl Into<String>) -> Schema3<T, U, V> {
        let InterimSchema3 {
            one,
            dimension_1,
            two,
            dimension_2,
            three,
            dimension_3,
        } = self;
        Schema3 {
            one,
            dimension_1,
            two,
            dimension_2,
            three,
            dimension_3,
            values: header.into(),
        }
    }

    /// Use the 2nd dimension as the breakdown dimension.
    pub fn breakdown_2nd(self) -> Schema3Breakdown2<T, U, V> {
        let InterimSchema3 {
            one,
            dimension_1,
            two,
            dimension_2,
            three,
            dimension_3,
            ..
        } = self;
        Schema3Breakdown2 {
            one,
            dimension_1,
            two,
            dimension_2,
            three,
            dimension_3,
        }
    }

    /// Use the 3rd dimension as the breakdown dimension.
    pub fn breakdown_3rd(self) -> Schema3Breakdown3<T, U, V> {
        let InterimSchema3 {
            one,
            dimension_1,
            two,
            dimension_2,
            three,
            dimension_3,
            ..
        } = self;
        Schema3Breakdown3 {
            one,
            dimension_1,
            two,
            dimension_2,
            three,
            dimension_3,
        }
    }
}
