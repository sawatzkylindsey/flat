use crate::aggregate::{aggregate_apply, minimal_precision_string};
use crate::render::{Alignment, Column, Columns, Flat, Grid, Render, Row, Value};
use crate::{Binnable, Dimensions, HistogramConfig, Schema, View};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, Sub};

/// The histogram widget.
///
/// ```
/// use flat::*;
///
/// let schema = Schemas::one("Things", "Count");
/// let builder = Dataset::builder(schema)
///     .add((0,), 0)
///     .add((0,), 1)
///     .add((1,), 4);
/// let view = builder.view();
/// let flat = Histogram::new(&view, 2)
///     .render(Render::default());
/// println!("{flat}");
///
/// // Output (modified for alignment)
/// r#"Things  |Count
///    [0, 1)  |*
///    [1, 2]  |****"#;
/// ```
pub struct Histogram<'a, S, V>
where
    S: Schema,
    <S as Schema>::Dimensions: Dimensions,
    V: View<S>,
    <V as View<S>>::PrimaryDimension: Clone
        + Display
        + PartialEq
        + PartialOrd
        + Add<<V as View<S>>::PrimaryDimension, Output = <V as View<S>>::PrimaryDimension>
        + Sub<<V as View<S>>::PrimaryDimension, Output = <V as View<S>>::PrimaryDimension>
        + Binnable,
    <V as View<S>>::BreakdownDimension: Clone + Display + PartialEq + Eq + Hash + Ord,
{
    view: &'a V,
    bins: usize,
    _phantom: PhantomData<S>,
}

impl<'a, S, V> Histogram<'a, S, V>
where
    S: Schema,
    <S as Schema>::Dimensions: Dimensions,
    V: View<S>,
    <V as View<S>>::PrimaryDimension: Clone
        + Display
        + PartialEq
        + PartialOrd
        + Add<<V as View<S>>::PrimaryDimension, Output = <V as View<S>>::PrimaryDimension>
        + Sub<<V as View<S>>::PrimaryDimension, Output = <V as View<S>>::PrimaryDimension>
        + Binnable,
    <V as View<S>>::BreakdownDimension: Clone + Display + PartialEq + Eq + Hash + Ord,
{
    /// Construct a histogram widget from the provided view and number of bins.
    pub fn new(view: &'a V, bins: usize) -> Self {
        Self {
            view,
            // Make sure there's always at least 1 bin.
            // It is either this, or return a Result<Histogram<S>, _>.
            bins: std::cmp::max(bins, 1),
            _phantom: PhantomData::default(),
        }
    }

    /// Generate the flat rendering for this histogram.
    pub fn render(self, config: Render<HistogramConfig>) -> Flat {
        let bin_ranges: Vec<Bounds<V::PrimaryDimension>> = if self.view.dataset().data.is_empty() {
            Vec::default()
        } else {
            let mut min = None;
            let mut max = None;

            for (dims, _) in self.view.dataset().data.iter() {
                let primary_dim = self.view.primary_dim(dims);

                let update_min = match &min {
                    Some(min) => primary_dim < *min,
                    None => true,
                };

                if update_min {
                    min.replace(primary_dim.clone());
                }

                let update_max = match &max {
                    Some(max) => primary_dim > *max,
                    None => true,
                };

                if update_max {
                    max.replace(primary_dim.clone());
                }
            }

            let min = min.expect("histogram must have some.values");
            let max = max.expect("histogram must have some.values");

            if min == max {
                vec![Bounds {
                    lower: Bound::Inclusive(min.clone()),
                    upper: Bound::Inclusive(min.clone()),
                }]
            } else {
                let delta = max - min.clone();
                let size = delta.divide(self.bins);
                (0..self.bins)
                    .map(|i| {
                        if i + 1 == self.bins {
                            Bounds {
                                lower: Bound::Inclusive(min.clone() + (size.multiply(i))),
                                upper: Bound::Inclusive(min.clone() + (size.multiply(i + 1))),
                            }
                        } else {
                            Bounds {
                                lower: Bound::Inclusive(min.clone() + (size.multiply(i))),
                                upper: Bound::Exclusive(min.clone() + (size.multiply(i + 1))),
                            }
                        }
                    })
                    .collect()
            }
        };

        let mut bin_aggregates: Vec<HashMap<V::BreakdownDimension, Vec<f64>>> =
            (0..self.bins).map(|_| HashMap::default()).collect();
        let mut sort_breakdowns: Vec<V::BreakdownDimension> = Vec::default();

        for (dims, v) in self.view.dataset().data.iter() {
            let primary_dim = self.view.primary_dim(&dims);
            let breakdown_dim = self.view.breakdown_dim(&dims);
            // TODO: Fix for performance
            let (index, _) = bin_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(&primary_dim))
                .expect(format!("key must map to one of the aggregating bins").as_str());
            let values = bin_aggregates[index]
                .entry(breakdown_dim.clone())
                .or_default();
            values.push(*v);

            if !sort_breakdowns.contains(&breakdown_dim) {
                sort_breakdowns.push(breakdown_dim);
            }
        }

        sort_breakdowns.sort();

        let mut columns = Columns::default();
        // histogram range
        columns.push(Column::string(Alignment::Left));

        if config.show_aggregate {
            // spacer " "
            columns.push(Column::string(Alignment::Center));
            // total left [
            columns.push(Column::string(Alignment::Left));
            // total value
            columns.push(Column::string(Alignment::Right));
            // total right ]
            columns.push(Column::string(Alignment::Left));
        }

        // spacer "  "
        columns.push(Column::string(Alignment::Center));
        // rendering delimiter |
        columns.push(Column::string(Alignment::Center));

        if self.view.is_breakdown() {
            for i in 0..sort_breakdowns.len() {
                // aggregate count
                columns.push(Column::breakdown(Alignment::Center));

                if i + 1 < sort_breakdowns.len() {
                    // spacer " "
                    columns.push(Column::string(Alignment::Left));
                }
            }

            // breakdown right |
            columns.push(Column::string(Alignment::Center));
        } else {
            // aggregate count
            columns.push(Column::count(Alignment::Left));
        }

        let mut grid = Grid::new(columns);

        if self.view.is_breakdown() {
            let mut pre_header = Row::default();
            pre_header.push(Value::Empty);

            if config.show_aggregate {
                pre_header.push(Value::Empty);
                pre_header.push(Value::Empty);
                pre_header.push(Value::Empty);
                pre_header.push(Value::Empty);
            }

            pre_header.push(Value::Empty);
            pre_header.push(Value::Empty);
            pre_header.push(Value::Plain(self.view.data_header()));
            grid.add(pre_header);
        }

        let mut header = Row::default();

        header.push(Value::String(self.view.headers()[0].clone()));

        if config.show_aggregate {
            header.push(Value::Empty);
            header.push(Value::Overflow(config.aggregate.to_string()));
            header.push(Value::Skip);
            header.push(Value::Skip);
        }

        header.push(Value::String("  ".to_string()));
        header.push(Value::String("|".to_string()));

        if self.view.is_breakdown() {
            for (k, breakdown_dim) in sort_breakdowns.iter().enumerate() {
                header.push(Value::String(breakdown_dim.to_string()));

                if k + 1 < sort_breakdowns.len() {
                    header.push(Value::String(" ".to_string()));
                }
            }

            header.push(Value::String("|".to_string()));
        } else {
            header.push(Value::Plain(self.view.data_header()));
        }

        grid.add(header);
        let mut minimum_value = f64::MAX;
        let mut maximum_value = f64::MIN;

        for (bounds, aggregates) in bin_ranges.into_iter().zip(bin_aggregates) {
            let mut row = Row::default();
            row.push(Value::String(bounds.to_string()));

            if self.view.is_breakdown() {
                let breakdown_values: Vec<f64> = sort_breakdowns
                    .iter()
                    .map(|breakdown_dim| {
                        aggregate_apply(
                            &config.aggregate,
                            &aggregates,
                            &breakdown_dim,
                            &mut minimum_value,
                            &mut maximum_value,
                        )
                    })
                    .collect();

                if config.show_aggregate {
                    row.push(Value::String(" ".to_string()));
                    row.push(Value::String("[".to_string()));
                    row.push(Value::String(minimal_precision_string(
                        config.aggregate.apply(breakdown_values.as_slice()),
                    )));
                    row.push(Value::String("]".to_string()));
                }

                row.push(Value::String("  ".to_string()));
                row.push(Value::String("|".to_string()));

                for (k, breakdown_value) in breakdown_values.iter().enumerate() {
                    row.push(Value::Value(*breakdown_value));

                    if k + 1 != sort_breakdowns.len() {
                        row.push(Value::String(" ".to_string()));
                    }
                }

                row.push(Value::String("|".to_string()));
            } else {
                let value = aggregate_apply(
                    &config.aggregate,
                    &aggregates,
                    &sort_breakdowns[0],
                    &mut minimum_value,
                    &mut maximum_value,
                );

                if config.show_aggregate {
                    row.push(Value::String(" ".to_string()));
                    row.push(Value::String("[".to_string()));
                    row.push(Value::String(minimal_precision_string(value)));
                    row.push(Value::String("]".to_string()));
                }

                row.push(Value::String("  ".to_string()));
                row.push(Value::String("|".to_string()));
                row.push(Value::Value(value));
            }

            grid.add(row);
        }

        Flat::new(config, minimum_value..maximum_value, grid)
    }
}

#[derive(Debug)]
struct Bounds<T: PartialOrd> {
    lower: Bound<T>,
    upper: Bound<T>,
}

impl<T: PartialOrd + Display> Display for Bounds<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (lb, lv) = match &self.lower {
            Bound::Exclusive(value) => ("(", value),
            Bound::Inclusive(value) => ("[", value),
        };
        let (ub, uv) = match &self.upper {
            Bound::Exclusive(value) => (")", value),
            Bound::Inclusive(value) => ("]", value),
        };
        write!(f, "{}{}, {}{}", lb, lv, uv, ub)
    }
}

impl<T: PartialOrd> Bounds<T> {
    fn contains(&self, item: &T) -> bool {
        self.lower.left_of(item) && self.upper.right_of(item)
    }
}

#[derive(Debug)]
enum Bound<T: PartialOrd> {
    Exclusive(T),
    Inclusive(T),
}

impl<T: PartialOrd> Bound<T> {
    fn left_of(&self, item: &T) -> bool {
        match &self {
            Bound::Exclusive(b) => b < item,
            Bound::Inclusive(b) => b <= item,
        }
    }

    fn right_of(&self, item: &T) -> bool {
        match &self {
            Bound::Exclusive(b) => b > item,
            Bound::Inclusive(b) => b >= item,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Dataset, Histogram, Schema1, Schema2, Schemas};

    #[test]
    fn empty() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let builder = Dataset::builder(schema);
        let view = builder.view();
        let histogram = Histogram::new(&view, 0);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc  |header"#
        );
    }

    #[test]
    fn add_zero_buckets() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let builder = Dataset::builder(schema)
            .add((1,), -1)
            .add((2,), 0)
            .add((3,), 1);
        let view = builder.view();
        let histogram = Histogram::new(&view, 0);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 3]  |"#
        );
    }

    #[test]
    fn add_one_bucket() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let builder = Dataset::builder(schema)
            .add((1,), 1)
            .add((2,), 1)
            .add((3,), 1);
        let view = builder.view();
        let histogram = Histogram::new(&view, 1);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 3]  |***"#
        );
    }

    #[test]
    fn add_extra_buckets() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let mut builder = Dataset::builder(schema);
        builder.update((1,), 1);
        let view = builder.view();
        let histogram = Histogram::new(&view, 2);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 1]  |*"#
        );
    }

    #[test]
    fn add_zero() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let mut builder = Dataset::builder(schema);
        builder.update((1,), 0);
        let view = builder.view();
        let histogram = Histogram::new(&view, 1);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 1]  |"#
        );
    }

    #[test]
    fn add_zero_and_ones() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let builder = Dataset::builder(schema)
            .add((1,), -1)
            .add((2,), 0)
            .add((3,), 1);
        let view = builder.view();
        let histogram = Histogram::new(&view, 3);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 2)  |⊖
[2, 3)  |
[3, 4]  |*"#
        );
    }

    #[test]
    fn add_onethousand() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let mut builder = Dataset::builder(schema);
        builder.update((1,), 1_000);
        let view = builder.view();
        let histogram = Histogram::new(&view, 1);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 1]  |*******************************************************************************************************************************************************"#
        );
    }

    #[test]
    fn add_negative_onethousand() {
        let schema: Schema1<u64> = Schemas::one("abc", "header");
        let mut builder = Dataset::builder(schema);
        builder.update((1,), -1_000);
        let view = builder.view();
        let histogram = Histogram::new(&view, 3);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 1]  |⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖"#
        );
    }

    #[test]
    fn view2() {
        let schema: Schema2<u64, f64> = Schemas::two("abc", "def", "header");
        let builder = Dataset::builder(schema)
            .add((1, 0.1), 1)
            .add((2, 0.2), 1)
            .add((3, 0.3), 1);
        let view = builder.view();
        let histogram = Histogram::new(&view, 1);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc     |header
[1, 3]  |***"#
        );

        let view = builder.reverse_view();
        let histogram = Histogram::new(&view, 1);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
def         |header
[0.1, 0.3]  |***"#
        );
    }

    #[test]
    fn breakdown() {
        let schema = Schemas::two("abc", "something long", "header");
        let builder = Dataset::builder(schema)
            .add((1, 2), -1)
            .add((2, 3), 0)
            .add((3, 4), 1);
        let view = builder.view_breakdown2();
        let histogram = Histogram::new(&view, 3);
        let flat = histogram.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
         something long
abc     |2 3 4|
[1, 2)  |⊖    |
[2, 3)  |     |
[3, 4]  |    *|"#
        );
    }
}
