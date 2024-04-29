use std::fmt::Display;
use std::marker::PhantomData;

pub trait Dimensions: Ord {
    fn chain(&self) -> Vec<String>;
}

pub trait Schematic {
    type Dims: Dimensions;

    fn width(&self) -> usize;

    fn headers(&self) -> Vec<String>;
}

pub struct Schema;

pub struct Schema1<T> {
    one: PhantomData<T>,
    column_1: String,
}

pub struct Schema2<T, U> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    breakdown: Option<String>,
}

pub struct Schema3<T, U, V> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    three: PhantomData<V>,
    column_3: String,
    breakdown: Option<String>,
}

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
            breakdown: None,
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
            breakdown: None,
        }
    }
}

impl<T> Schematic for Schema1<T>
where
    T: Display + Ord,
{
    type Dims = (T,);

    fn width(&self) -> usize {
        1
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }
}

pub enum Breakdown2 {
    Second,
}

impl<T, U> Schematic for Schema2<T, U>
where
    T: Display + Ord,
    U: Display + Ord,
{
    type Dims = (T, U);

    fn width(&self) -> usize {
        2
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }
}

impl<T, U> Schema2<T, U> {
    pub fn breakdown(mut self, breakdown: Breakdown2) -> Self {
        self.breakdown = match breakdown {
            Breakdown2::Second => Some(self.column_2.clone()),
        };
        self
    }
}

pub enum Breakdown3 {
    Second,
    Third,
}

impl<T, U, V> Schematic for Schema3<T, U, V>
where
    T: Display + Ord,
    U: Display + Ord,
    V: Display + Ord,
{
    type Dims = (T, U, V);

    fn width(&self) -> usize {
        3
    }

    fn headers(&self) -> Vec<String> {
        vec![
            self.column_1.clone(),
            self.column_2.clone(),
            self.column_3.clone(),
        ]
    }
}

impl<T, U, V> Schema3<T, U, V> {
    pub fn breakdown(mut self, breakdown: Breakdown3) -> Self {
        self.breakdown = match breakdown {
            Breakdown3::Second => Some(self.column_2.clone()),
            Breakdown3::Third => Some(self.column_3.clone()),
        };
        self
    }
}

impl<T> Dimensions for (T,)
where
    T: Display + Ord,
{
    // fn locus(&self) -> String {
    //     self.0.to_string()
    // }

    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl<T, U> Dimensions for (T, U)
where
    T: Display + Ord,
    U: Display + Ord,
{
    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string()]
    }
}

impl<T, U, V> Dimensions for (T, U, V)
where
    T: Display + Ord,
    U: Display + Ord,
    V: Display + Ord,
{
    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string(), self.2.to_string()]
    }
}
