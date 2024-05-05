use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Dimensions {
    fn as_strings(&self) -> Vec<String>;

    fn len(&self) -> usize;
}

pub trait Schematic {
    type Dimensions;
    type PrimaryDimension;
    type BreakdownDimension;
    type SortDimensions;

    fn total_width(&self) -> usize;

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension;

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension;

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions;

    fn sort_headers(&self) -> Vec<String>;

    fn breakdown_header(&self) -> Option<String>;

    fn is_breakdown(&self) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nothing;

impl Display for Nothing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
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
    T: Clone,
{
    type Dimensions = (T,);
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T,);

    fn total_width(&self) -> usize {
        1
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, _dims: &Self::Dimensions) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn sort_headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

impl<T, U> Schematic for Schema2<T, U>
where
    T: Clone,
    U: Clone,
{
    type Dimensions = (T, U);
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T, U);

    fn total_width(&self) -> usize {
        2
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, _dims: &Self::Dimensions) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn sort_headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

impl<T, U> Schematic for Schema2Breakdown2<T, U>
where
    T: Clone,
    U: Clone,
{
    type Dimensions = (T, U);
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type SortDimensions = (T,);

    fn total_width(&self) -> usize {
        2
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension {
        dims.1.clone()
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(),)
    }

    fn sort_headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_2.clone())
    }

    fn is_breakdown(&self) -> bool {
        true
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
    T: Clone,
    U: Clone,
    V: Clone,
{
    type Dimensions = (T, U, V);
    type PrimaryDimension = T;
    type BreakdownDimension = Nothing;
    type SortDimensions = (T, U, V);

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, _dims: &Self::Dimensions) -> Self::BreakdownDimension {
        Nothing
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        dims.clone()
    }

    fn sort_headers(&self) -> Vec<String> {
        vec![
            self.column_1.clone(),
            self.column_2.clone(),
            self.column_3.clone(),
        ]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn is_breakdown(&self) -> bool {
        false
    }
}

impl<T, U, V> Schematic for Schema3Breakdown2<T, U, V>
where
    T: Clone,
    U: Clone,
    V: Clone,
{
    type Dimensions = (T, U, V);
    type PrimaryDimension = T;
    type BreakdownDimension = U;
    type SortDimensions = (T, V);

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension {
        dims.1.clone()
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.2.clone())
    }

    fn sort_headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_3.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_2.clone())
    }

    fn is_breakdown(&self) -> bool {
        true
    }
}

impl<T, U, V> Schematic for Schema3Breakdown3<T, U, V>
where
    T: Clone,
    U: Clone,
    V: Clone,
{
    type Dimensions = (T, U, V);
    type PrimaryDimension = T;
    type BreakdownDimension = V;
    type SortDimensions = (T, U);

    fn total_width(&self) -> usize {
        3
    }

    fn primary_dim(&self, dims: &Self::Dimensions) -> Self::PrimaryDimension {
        dims.0.clone()
    }

    fn breakdown_dim(&self, dims: &Self::Dimensions) -> Self::BreakdownDimension {
        dims.2.clone()
    }

    fn sort_dims(&self, dims: &Self::Dimensions) -> Self::SortDimensions {
        (dims.0.clone(), dims.1.clone())
    }

    fn sort_headers(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        Some(self.column_3.clone())
    }

    fn is_breakdown(&self) -> bool {
        true
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

impl<T> Dimensions for (T,)
where
    T: Display,
{
    fn as_strings(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }

    fn len(&self) -> usize {
        1
    }
}

impl<T, U> Dimensions for (T, U)
where
    T: Display,
    U: Display,
{
    fn as_strings(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string()]
    }

    fn len(&self) -> usize {
        2
    }
}

impl<T, U, V> Dimensions for (T, U, V)
where
    T: Display,
    U: Display,
    V: Display,
{
    fn as_strings(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string(), self.2.to_string()]
    }

    fn len(&self) -> usize {
        3
    }
}
