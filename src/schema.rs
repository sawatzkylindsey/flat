use std::fmt::Display;
use std::marker::PhantomData;

pub trait Dimensions: Clone {
    fn chain(&self) -> Vec<String>;
}

pub trait Schematic {
    type Dims: Dimensions;
    type SortDims: Dimensions + Ord;
    type BreakdownDim;

    fn width(&self) -> usize;

    fn headers(&self) -> Vec<String>;

    fn sort_dims(&self, dims: &Self::Dims) -> Self::SortDims;

    fn breakdown_header(&self) -> Option<String>;

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)>;

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>);
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
}

pub struct Schema2Breakdown2<T, U> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
}

pub struct Schema3<T, U, V> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    three: PhantomData<V>,
    column_3: String,
}

pub struct Schema3Breakdown2<T, U, V> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    three: PhantomData<V>,
    column_3: String,
}

pub struct Schema3Breakdown3<T, U, V> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    three: PhantomData<V>,
    column_3: String,
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

impl<T> Schematic for Schema1<T>
where
    T: Clone + Display + Ord,
{
    type Dims = (T,);
    type SortDims = (T,);
    type BreakdownDim = ();

    fn width(&self) -> usize {
        1
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn sort_dims(&self, dims: &Self::Dims) -> Self::SortDims {
        dims.clone()
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn path(&self, chain: Vec<String>, _dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 1);
        vec![(0, chain[0].clone())]
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 1);
        (chain[0].clone(), None)
    }
}

impl<T, U> Schematic for Schema2<T, U>
where
    T: Clone + Display + Ord,
    U: Clone + Display + Ord,
{
    type Dims = (T, U);
    type SortDims = (T, U);
    type BreakdownDim = ();

    fn width(&self) -> usize {
        2
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }

    fn sort_dims(&self, dims: &Self::Dims) -> Self::SortDims {
        dims.clone()
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 2);
        let mut out = Vec::default();

        for i in 0..dag_index + 1 {
            out.push((i, chain[i].clone()));
        }

        out
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 2);
        (chain[0].clone(), None)
    }
}

impl<T, U> Schematic for Schema2Breakdown2<T, U>
where
    T: Clone + Display + Ord,
    U: Clone + Display + Ord,
{
    type Dims = (T, U);
    type SortDims = (T,);
    type BreakdownDim = U;

    fn width(&self) -> usize {
        2
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn sort_dims(&self, dims: &Self::Dims) -> Self::SortDims {
        (dims.0.clone(),)
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_2.clone())
    }

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 2);
        let mut out = Vec::default();

        for i in 0..dag_index + 1 {
            // We want to skip the breakdown column
            if i != 1 {
                out.push((i, chain[i].clone()));
            }
        }

        out
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 2);
        (chain[0].clone(), Some(chain[1].clone()))
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

impl<T, U, V> Schematic for Schema3<T, U, V>
where
    T: Clone + Display + Ord,
    U: Clone + Display + Ord,
    V: Clone + Display + Ord,
{
    type Dims = (T, U, V);
    type SortDims = (T, U, V);
    type BreakdownDim = ();

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

    fn sort_dims(&self, dims: &Self::Dims) -> Self::SortDims {
        dims.clone()
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 3);
        let mut out = Vec::default();

        for i in 0..dag_index + 1 {
            out.push((i, chain[i].clone()));
        }

        out
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 3);
        (chain[0].clone(), None)
    }
}

impl<T, U, V> Schematic for Schema3Breakdown2<T, U, V>
where
    T: Clone + Display + Ord,
    U: Clone + Display + Ord,
    V: Clone + Display + Ord,
{
    type Dims = (T, U, V);
    type SortDims = (T, V);
    type BreakdownDim = U;

    fn width(&self) -> usize {
        3
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_3.clone()]
    }

    fn sort_dims(&self, dims: &Self::Dims) -> Self::SortDims {
        (dims.0.clone(), dims.2.clone())
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_2.clone())
    }

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 3);
        let mut out = Vec::default();

        for i in 0..dag_index + 1 {
            // We want to skip the breakdown column
            if i != 1 {
                out.push((i, chain[i].clone()));
            }
        }

        out
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 3);
        (chain[0].clone(), Some(chain[1].clone()))
    }
}

impl<T, U, V> Schematic for Schema3Breakdown3<T, U, V>
where
    T: Clone + Display + Ord,
    U: Clone + Display + Ord,
    V: Clone + Display + Ord,
{
    type Dims = (T, U, V);
    type SortDims = (T, U);
    type BreakdownDim = V;

    fn width(&self) -> usize {
        3
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }

    fn sort_dims(&self, dims: &Self::Dims) -> Self::SortDims {
        (dims.0.clone(), dims.1.clone())
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_3.clone())
    }

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 3);
        let mut out = Vec::default();

        for i in 0..dag_index + 1 {
            // We want to skip the breakdown column
            if i != 2 {
                out.push((i, chain[i].clone()));
            }
        }

        out
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 3);
        (chain[0].clone(), Some(chain[1].clone()))
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

impl<T: Clone> Dimensions for (T,)
where
    T: Display + Ord,
{
    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl<T, U> Dimensions for (T, U)
where
    T: Clone + Display + Ord,
    U: Clone + Display + Ord,
{
    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string()]
    }
}

impl<T, U, V> Dimensions for (T, U, V)
where
    T: Clone + Display + Ord,
    U: Clone + Display + Ord,
    V: Clone + Display + Ord,
{
    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string(), self.2.to_string()]
    }
}
