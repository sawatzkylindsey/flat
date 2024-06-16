// We use this in the doc strings.
#[allow(unused_imports)]
use super::super::Aggregate;
// We use this in the doc strings.
#[allow(unused_imports)]
use super::super::Render;
// We use this in the doc strings.
#[allow(unused_imports)]
use super::BarChart;

/// Render configuration specific to [`BarChart`]s.
///
/// ### Example
/// ```
/// # use flat::BarChartConfig;
/// let barchart_config = BarChartConfig {
///     abbreviate: true,
///     show_aggregate: true,
///     ..BarChartConfig::default()
/// };
/// ```
#[derive(Debug, Default)]
pub struct BarChartConfig {
    /// Whether to abbreviate the dimensional values in the rendering or not.
    /// Use this option when the dimensions have long [`std::fmt::Display`] forms.
    /// Abbreviation is attempted irrespective of the `width_hint`.
    ///
    /// **Notice**, abbreviation is performed on values only, and not on the column headers.
    /// Moreover, the abbreviation is bounded by the column header width.
    /// If you want shorter abbreviations, be sure to shorten the headers as well!
    ///
    /// Default: `false`.
    pub abbreviate: bool,
    /// Whether to show the aggregated result for the *non-primary* dimension(s) of the dataset.
    /// Notice, this is different from `show_aggregate` in the root [`Render`] configuration (which shows the *primary* dimension).
    ///
    /// The aggregate always represents the matched columns from the *primary* towards the *non-primary*.
    /// Consequently, the result at each level shows the aggregate across the previous level - that is, the aggregates "cascade".
    ///
    /// For example, consider the following `Dataset`.
    /// ```ignore
    /// r#"
    /// A (primary) | B  | C  | value
    /// a1          | b1 | c1 | 1
    /// a1          | b2 | c1 | 2
    /// a1          | b2 | c2 | 3"#
    /// ```
    ///
    /// Then, the aggregate computation for each level is as follows:
    /// ```ignore
    /// r#"
    /// Dimensional Combinations | Show Aggregate
    /// (a1,)                    | aggregate([1, 2, 3])
    /// (a1, b1)                 | aggregate([1])
    /// (a1, b2)                 | aggregate([2, 3])
    /// (a1, b1, c1)             | aggregate([1])
    /// (a1, b2, c1)             | aggregate([2])
    /// (a1, b2, c2)             | aggregate([3])"#
    /// ```
    ///
    /// In `flat`, the above dataset as [`Aggregate::Sum`] will render as follows:
    /// ```ignore
    /// // Output (modified for alignment)
    /// // Observe the cascade (ex: `b2 [5] = c1 [2] + c2 [3]`).
    /// r#"
    /// C  Sum   B  Sum   A  Sum
    /// c1 [1] - b1 [1] ┐
    /// c1 [2] - b2 [5] - a1 [6]
    /// c2 [3] ┘        ┘"#
    /// ```
    ///
    /// Default: `false`.
    pub show_aggregate: bool,
}
