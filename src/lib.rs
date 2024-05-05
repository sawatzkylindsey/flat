mod categorical;
mod histogram;
mod render;
mod schema;

pub use categorical::*;
pub use histogram::*;
pub use render::{Flat, Render};
pub use schema::*;
use std::fmt::{Display, Formatter};

pub trait Dimensions {
    fn as_strings(&self) -> Vec<String>;

    fn len(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nothing;

impl Display for Nothing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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
