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
//!
//! # Value Rendering Details
//! `flat` follows a few simple rules when generating the "visual" rendering of data.
//! The details follow, but in the general case the visual rendering should be assumed to be *relative*.
//! That is, the literal count of characters (ex: `'*'`) does not necessarily represent the literal value of the data.
//!
//! The `flat` rendering process is subject to change, but can be summarized with the following procedure:
//! 1. Calculate the visual rendering width by `width_hint - OTHER_COLUMNS`.
//! 2. If this width is less than `2`, then set it to `2`.
//! 3. Check if the greatest aggregate value to be rendered fits within the visual rendering.
//! If it does, draw the values literally (in this case, a single character `'*'` does represent the absolute value).
//! 4. Otherwise, find the linear scaling such that the greatest aggregate value fits within the visual rendering.
//! Scale the aggregate values accordingly (ex: a single character `'*'` represents a relative value).
//!
//! Notice, by design this process will only *down-scale* the values to fit within the visual rendering.
//! Values will never be *up-scaled* to fit the `width_hint`.
//!
//! The above process also applies for fractional and negative values.
//! For fractions, `flat` always round the aggregate value before scaling.
//! In the case of negatives, `flat` takes the absolute value to detect the appropriate bounds and to render the representation characters.
//! Negative values are rendering using a different character marker (ex: `'⊖'`).
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
