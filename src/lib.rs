//! `flat` is textual data renderer for Rust.
//!
//! The goal of `flat` is to provide access to complex data in a low-resolution textual form.
//! This can be useful in certain contexts where high-resolution graphics are not suitable for logistic reasons (ex: space constraints, accessibility, etc).
//! Although textual rendering has its drawbacks, we believe these are acceptable in certain contexts.
//! We also think `flat` provides a few visual mechanisms that help overcome some of the low-resolution textual form drawbacks.
//!
//! To get a little specific, we employ a few techniques which improve the "analyzability" of our low-resolution charts.
//! 1. Flattening multi-dimensional categories across a visual hierarchy (tree).
//! 2. Breaking down a single-dimension category across the columnar visual space.
//!
//! #### Hierarchy Tree
//! TODO
//!
//! #### Breakdowns
//! TODO
//!
//! # Usage
//!

// For example, it is simple to extend the debug lines of a standalone tool to include multi-line charts.
// where adding graphical charts may add logistic complexity
//
//
// in the debug logs of a standalone tool, we can easily include some multi-line
// Although it is possible to render graphical charts, saved as files either locally or remotely, this option is not always attractive or practical.
// `flat` is designed to fill this gap.
mod abbreviate;
mod aggregate;
mod barchart;
mod histogram;
mod render;
mod schema;

pub use aggregate::Aggregate;
pub use barchart::*;
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
