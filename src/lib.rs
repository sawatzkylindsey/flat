//! `flat` is textual data renderer for Rust.
//!
//! The goal of `flat` is to provide access to complex data in a low-resolution textual form.
//! This can be useful in certain contexts where high-resolution graphics are not suitable for logistic reasons (ex: space constraints, accessibility, etc).
//! Although textual rendering has its drawbacks, we believe these are acceptable in certain contexts.
//! The output of `flat` is best observed using a monospaced font.
//!
//! This documentation begins with a few example renderings to showcase `flat`.
//! Go to the next section for usage instructions.
//!
//! ### Zoo Dataset
//! Imagine a zoo; it contains various species of animals held in enclosures.
//! The enclosures of this imaginary zoo are organized into the quadrants NorthEast, NorthWest, SouthEast, and SouthWest.
//!
//! Let's use `flat` to visualize this dataset.
//! We'll first look at the density of animals across the zoo:
//! ```text
//! Animal           Enclosure    Quadrant   |Sum(Count)
//! Sea Otter      - Pen01      - NorthEast  |*****
//! Falcon         - Pen02      ┘
//! Duck           - Open       ┐
//! Flamingo       ┘
//! Tiger          - Pen03      - NorthWest  |************
//! Crane          ┐
//! Kingfisher     - Pen04      ┘
//! Stork          ┘
//! Black Bear     - Pen05      - SouthEast  |**
//! Grizzly Bear   - Pen06      - SouthWest  |****
//! Mountain Goat  - Pen07      ┘
//! ```
//!
//! We can also drill into this dataset a number of ways.
//! Let's look at the specific breakdown of animals across quadrants/enclosures:
//! ```text
//!                          Animal
//!                          Sum(Count)
//! Enclosure    Quadrant   |Black Bear     Crane       Duck       Falcon     Flamingo   Grizzly B.. Kingfisher  Mountain ..  Sea Otter     Stork       Tiger   |
//! Pen01      - NorthEast  |                                        **                                                          ***                            |
//! Pen02      ┘
//! Open       ┐
//! Pen03      - NorthWest  |                 *          ***                    ****                      *                                   *          **     |
//! Pen04      ┘
//! Pen05      - SouthEast  |    **                                                                                                                             |
//! Pen06      - SouthWest  |                                                                 *                      ***                                        |
//! Pen07      ┘
//! ```
//!
//! Finally, let's take a look at an attribute directly - we'll compare animal lengths.
//! Notice, the visualization on the right (commonly referred to as the rendering) show's relative values in `flat`.
//! That's why we've also included the absolute value in this visualization (ex: Ducks are on average 29cm long).
//! ```text
//! Animal        Average  |Average(Length (cm))
//! Black Bear    [   75]  |*
//! Crane         [   60]  |*
//! Duck          [   29]  |
//! Falcon        [   55]  |*
//! Flamingo      [   85]  |*
//! Grizzly Bear  [  220]  |****
//! Kingfisher    [   15]  |
//! Mountain Goat [113.3]  |**
//! Sea Otter     [133.3]  |**
//! Stork         [   60]  |*
//! Tiger         [  285]  |******
//! ```
//!
//! We can also look at this data the other way around - how do the zoo animals spread across length.
//! ```text
//! Length (cm)   |Sum(Count)
//! [15, 42.5)    |*****
//! [42.5, 70)    |****
//! [70, 97.5)    |****
//! [97.5, 125)   |****
//! [125, 152.5)  |***
//! [152.5, 180)  |
//! [180, 207.5)  |
//! [207.5, 235)  |*
//! [235, 262.5)  |
//! [262.5, 290]  |**
//! ```
//!
//! As before, we can further drill down into this by looking at how this spread looks specifically across the animal species.
//! ```text
//!                Animal
//!                Sum(Count)
//! Length (cm)   | Black Bear     Crane         Duck        Falcon      Flamingo   Grizzly Bear  Kingfisher  Mountain G..  Sea Otter      Stork        Tiger    |
//! [15, 42.5)    |     *                        ***                                                  *                                                          |
//! [42.5, 70)    |                  *                         **                                                                            *                   |
//! [70, 97.5)    |                                                        ****                                                                                  |
//! [97.5, 125)   |     *                                                                                          **           *                                |
//! [125, 152.5)  |                                                                                                *            **                               |
//! [152.5, 180)  |                                                                                                                                              |
//! [180, 207.5)  |                                                                                                                                              |
//! [207.5, 235)  |                                                                      *                                                                       |
//! [235, 262.5)  |                                                                                                                                              |
//! [262.5, 290]  |                                                                                                                                       **     |
//! ```
//!
//! # Usage
//! This rest of this page describes the general usage for `flat`.
//! Detailed examples can be found in [the source](https://github.com/sawatzkylindsey/flat/tree/main/examples).
//!
//! The general workflow for using `flat` is to 1) construct a dataset, 2) get a specific view of that dataset, and 3) render the view using a widget.
//!
//! ### Construct a Dataset
//! To construct a [`Dataset`], you need both data and a [`Schemas`] \[sic\].
//!
//! The data must come in the form of dense vectors expressed as a [rust tuple](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-tuple-type).
//! For example, one data point in a dataset could be: `(49.238, -123.114, Direction::North, "Strawberry Bush")`.
//! Then, all the data points for this example `Dataset` must come in this 4-dimensional form (and maintain the same type pattern).
//! This is where `Schemas` come in - the schema defines the specific type pattern of your dataset, as well as the column names for that dataset.
//! To continue our example, here's a schema to fit the dataset: `Schemas::four("Latitude", "Longitude", "Direction", "Object")`.
//!
//! From what we've shown, the actual type definitions are [inferred by the compiler](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html).
//! This is the recommended way to use `flat`.
//! If you prefer, the types can be annotated explicitly:
//! ```
//! # use flat::*;
//! # enum Direction {
//! #     North,
//! #     South,
//! # }
//! let my_schema: Schema4<f64, f64, Direction, &str> = Schemas::four("Latitude", "Longitude", "Direction", "Object");
//! ```
//!
//! Datasets are constructed using a builder.
//! See the [`DatasetBuilder`] docs for more details.
//!
//! ### Get a View
//! A view describes *what* to look at the dataset (but not *how* to render it).
//! It includes aspects like which columns form the frame vs. the rendering, as well as how to aggregate the rendering values.
//!
//! The visualizations in `flat` always come in the following form.
//! ```text
//! Frame..  | Rendering..
//! Frame..  | Rendering..
//! Frame..
//! Frame..  | Rendering..
//! ```
//!
//! The "frame" is the organizational basis of the visualization, and it always shows at least 1 dimension from the dataset.
//! The "rendering" is a visualized value which may come directly from the dataset, but also may be inferred (ex: in the case of a "count").
//! Notice, not every line of output in the visualization need necessarily show a rendering.
//!
//! Typically, the rendering does not show a single value from the dataset, but rather some aggregate of values.
//! This is called the [`Aggregate`], and is defined at the time of rendering (next section).
//!
//! To get a view, simply invoke the appropriate method on your dataset.
//! These methods are inferred based off the schema of the dataset, and multiple views may be drawn from the same dataset.
//! For example: `dataset.reflective_view()`, `dataset.view_2nd()`, `dataset.breakdown_3rd()`, or `dataset.counting_view()`.
//! See the [`Dataset`] docs for more details.
//!
//! Additionally, custom view implementations may be defined by the user.
//! Take a look at the docs for more details: [`View`].
//!
//! ### Render a Widget
//! A widget describes *how* to render a view - the same view may be rendered differently by different widgets.
//! This is where the specific appearance of the frame and rendering are defined.
//!
//! To render a widget, construct it with the view, and then invoke render with a `Render` configuration.
//! This will produce a [`Flat`] which provides a `std::fmt::Display` implementation for the visualization.
//!
//! The configuration includes a number of parameters, which come in two sections.
//! 1. The general parameters which apply to all widgets.
//! 2. The widget specific configuration (`widget_config`).
//!
//! We recommend using [the struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax) (with `std::default::Default`) to instantiate the config.
//! ```
//! # use flat::*;
//! // Take the default widget specific configuration.
//! let config: Render<HistogramConfig> = Render {
//!     width_hint: 50,
//!     ..Render::default()
//! };
//!
//! // Override the widget specific configuration.
//! let config: Render<BarChartConfig> = Render {
//!     width_hint: 50,
//!     widget_config: BarChartConfig {
//!         show_aggregate: true,
//!         ..BarChartConfig::default()
//!     },
//!     ..Render::default()
//! };
//! ```
//!
//! For specific details on the configurable values, take a look at the docs: [`Render`].
//! The next section describes one of the key parameters to a rendering - the width (via `width_hint`).
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
mod dataset;
mod histogram;
mod pathchart;
mod render;
mod schema;
mod view;

pub use aggregate::{minimal_precision_string, Aggregate};
pub use barchart::*;
pub use dataset::*;
pub use histogram::*;
pub use pathchart::*;
pub use render::{Flat, Render};
pub use schema::*;
use std::fmt::{Display, Formatter};
pub use view::*;

/// Trait to extract the display values for rendering.
///
/// Typically, consumers need not implement this trait (as long as they use tuple data types, ex: `(T, U, V)`).
pub trait Dimensions {
    /// Get the string format values from this vector/data.
    fn as_strings(&self) -> Vec<String>;

    /// Get the length of this vector/data.
    fn len(&self) -> usize;
}

/// No-op struct used to indicate an unused associated type in the widget's trait.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<T, U, V, W> Dimensions for (T, U, V, W)
where
    T: Display,
    U: Display,
    V: Display,
    W: Display,
{
    fn as_strings(&self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
        ]
    }

    fn len(&self) -> usize {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_one() {
        let one = ("abc".to_string(),);
        assert_eq!(one.len(), 1);
        assert_eq!(one.as_strings(), vec!["abc".to_string()]);
    }

    #[test]
    fn dimensions_two() {
        let two = ("abc".to_string(), 1);
        assert_eq!(two.len(), 2);
        assert_eq!(two.as_strings(), vec!["abc".to_string(), "1".to_string()]);
    }

    #[test]
    fn dimensions_three() {
        let three = ("abc".to_string(), 1, true);
        assert_eq!(three.len(), 3);
        assert_eq!(
            three.as_strings(),
            vec!["abc".to_string(), "1".to_string(), "true".to_string()]
        );
    }
}
