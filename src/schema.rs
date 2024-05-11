use std::marker::PhantomData;

/// Starting point for specifying the dimensions of a dataset.
///
/// ```ignore
/// use flat::Schema;
///
/// let my_dimensions = Schema::two("column_1", "column_2");
/// ```
pub struct Schema;

pub struct Schema1<T> {
    pub(crate) one: PhantomData<T>,
    pub(crate) column_1: String,
}

pub struct Schema2<T, U> {
    pub(crate) one: PhantomData<T>,
    pub(crate) column_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) column_2: String,
}

pub struct Schema2Breakdown2<T, U> {
    pub(crate) one: PhantomData<T>,
    pub(crate) column_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) column_2: String,
}

pub struct Schema3<T, U, V> {
    pub(crate) one: PhantomData<T>,
    pub(crate) column_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) column_2: String,
    pub(crate) three: PhantomData<V>,
    pub(crate) column_3: String,
}

pub struct Schema3Breakdown2<T, U, V> {
    pub(crate) one: PhantomData<T>,
    pub(crate) column_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) column_2: String,
    pub(crate) three: PhantomData<V>,
    pub(crate) column_3: String,
}

pub struct Schema3Breakdown3<T, U, V> {
    pub(crate) one: PhantomData<T>,
    pub(crate) column_1: String,
    pub(crate) two: PhantomData<U>,
    pub(crate) column_2: String,
    pub(crate) three: PhantomData<V>,
    pub(crate) column_3: String,
}

// TODO
// pub struct Schema4<T, U, V, W>;
// pub struct Schema5<T, U, V, W, X>;
// pub struct Schema6<T, U, V, W, X, Y>;
// pub struct Schema7<T, U, V, W, X, Y, Z>;

impl Schema {
    pub fn one<T>(column_1: impl Into<String>) -> Schema1<T> {
        Schema1 {
            one: PhantomData,
            column_1: column_1.into(),
        }
    }

    pub fn two<T, U>(column_1: impl Into<String>, column_2: impl Into<String>) -> Schema2<T, U> {
        Schema2 {
            one: PhantomData,
            column_1: column_1.into(),
            two: PhantomData,
            column_2: column_2.into(),
        }
    }

    pub fn three<T, U, V>(
        column_1: impl Into<String>,
        column_2: impl Into<String>,
        column_3: impl Into<String>,
    ) -> Schema3<T, U, V> {
        Schema3 {
            one: PhantomData,
            column_1: column_1.into(),
            two: PhantomData,
            column_2: column_2.into(),
            three: PhantomData,
            column_3: column_3.into(),
        }
    }
}

impl<T, U> Schema2<T, U> {
    pub fn breakdown_2nd(self) -> Schema2Breakdown2<T, U> {
        let Schema2 {
            one,
            column_1,
            two,
            column_2,
        } = self;
        Schema2Breakdown2 {
            one,
            column_1,
            two,
            column_2,
        }
    }
}

impl<T, U, V> Schema3<T, U, V> {
    pub fn breakdown_2nd(self) -> Schema3Breakdown2<T, U, V> {
        let Schema3 {
            one,
            column_1,
            two,
            column_2,
            three,
            column_3,
        } = self;
        Schema3Breakdown2 {
            one,
            column_1,
            two,
            column_2,
            three,
            column_3,
        }
    }

    pub fn breakdown_3rd(self) -> Schema3Breakdown3<T, U, V> {
        let Schema3 {
            one,
            column_1,
            two,
            column_2,
            three,
            column_3,
        } = self;
        Schema3Breakdown3 {
            one,
            column_1,
            two,
            column_2,
            three,
            column_3,
        }
    }
}
