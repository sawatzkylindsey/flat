use crate::aggregate::{aggregate_apply, minimal_precision_string};
use crate::render::{Alignment, Column, Columns, Flat, Grid, Render, Row, Value};
use crate::{Binnable, HistogramConfig, HistogramSchematic};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, Sub};

pub struct Histogram<S: HistogramSchematic> {
    schema: S,
    bins: usize,
    data: Vec<(S::Dimensions, f64)>,
    min: Option<S::PrimaryDimension>,
    max: Option<S::PrimaryDimension>,
}

impl<S: HistogramSchematic> Histogram<S>
where
    S: HistogramSchematic,
    <S as HistogramSchematic>::PrimaryDimension: Clone
        + Display
        + PartialEq
        + PartialOrd
        + Add<
            <S as HistogramSchematic>::PrimaryDimension,
            Output = <S as HistogramSchematic>::PrimaryDimension,
        > + Sub<
            <S as HistogramSchematic>::PrimaryDimension,
            Output = <S as HistogramSchematic>::PrimaryDimension,
        > + Binnable,
    <S as HistogramSchematic>::BreakdownDimension: Clone + Display + PartialEq + Eq + Hash + Ord,
{
    pub fn builder(schema: S, bins: usize) -> Histogram<S> {
        Self {
            schema,
            bins,
            data: Vec::default(),
            min: None,
            max: None,
        }
    }

    pub fn add(mut self, dims: S::Dimensions, value: impl Into<f64>) -> Histogram<S> {
        let primary_dim = self.schema.primary_dim(&dims);

        let update_min = match &self.min {
            Some(min) => primary_dim < *min,
            None => true,
        };

        if update_min {
            self.min.replace(primary_dim.clone());
        }

        let update_max = match &self.max {
            Some(max) => primary_dim > *max,
            None => true,
        };

        if update_max {
            self.max.replace(primary_dim.clone());
        }

        self.data.push((dims, value.into()));
        self
    }

    pub fn render(self, config: Render<HistogramConfig>) -> Flat {
        let Self {
            bins,
            data,
            min,
            max,
            ..
        } = self;

        let min = min.expect("histogram must have some data");
        let max = max.expect("histogram must have some data");
        let delta = max - min.clone();
        let size = delta.divide(bins);
        let bin_ranges: Vec<Bounds<S::PrimaryDimension>> = (0..bins)
            .map(|i| {
                if i + 1 == bins {
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
            .collect();
        let mut bin_aggregates: Vec<HashMap<S::BreakdownDimension, Vec<f64>>> =
            (0..bins).map(|_| HashMap::default()).collect();
        let mut sort_breakdowns: Vec<S::BreakdownDimension> = Vec::default();

        for (dims, v) in data {
            let primary_dim = self.schema.primary_dim(&dims);
            let breakdown_dim = self.schema.breakdown_dim(&dims);
            // TODO: Fix for performance
            let (index, _) = bin_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(&primary_dim))
                .expect(format!("key must map to one of the aggregating bins").as_str());
            let values = bin_aggregates[index]
                .entry(breakdown_dim.clone())
                .or_default();
            values.push(v);

            if !sort_breakdowns.contains(&breakdown_dim) {
                sort_breakdowns.push(breakdown_dim);
            }
        }

        sort_breakdowns.sort();

        let mut columns = Columns::default();
        // histogram range
        columns.push(Column::string(Alignment::Left));
        // spacer "  "
        columns.push(Column::string(Alignment::Center));

        if config.show_aggregate {
            // total left [
            columns.push(Column::string(Alignment::Left));
            // total value
            columns.push(Column::string(Alignment::Right));
            // total right ]
            columns.push(Column::string(Alignment::Left));
            // spacer "  "
            columns.push(Column::string(Alignment::Center));
        }

        if self.schema.is_breakdown() {
            // breakdown left |
            columns.push(Column::string(Alignment::Center));

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
        let mut row = Row::default();

        row.push(Value::String(self.schema.primary_header()));
        row.push(Value::Empty);

        if config.show_aggregate {
            row.push(Value::Overflow(config.aggregate.to_string()));
            row.push(Value::Skip);
            row.push(Value::Skip);
            row.push(Value::Empty);
        }

        if self.schema.is_breakdown() {
            row.push(Value::String("|".to_string()));

            for (k, breakdown_dim) in sort_breakdowns.iter().enumerate() {
                row.push(Value::String(breakdown_dim.to_string()));

                if k + 1 < sort_breakdowns.len() {
                    row.push(Value::String(" ".to_string()));
                }
            }

            row.push(Value::String("|".to_string()));
        }

        grid.add(row);
        let mut minimum_value = f64::MAX;
        let mut maximum_value = f64::MIN;

        for (bounds, aggregates) in bin_ranges.into_iter().zip(bin_aggregates) {
            let mut row = Row::default();
            row.push(Value::String(bounds.to_string()));
            row.push(Value::String("  ".to_string()));

            if self.schema.is_breakdown() {
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
                    row.push(Value::String("[".to_string()));
                    row.push(Value::String(minimal_precision_string(
                        config.aggregate.apply(breakdown_values.as_slice()),
                    )));
                    row.push(Value::String("]".to_string()));
                    row.push(Value::String("  ".to_string()));
                }

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
                    row.push(Value::String("[".to_string()));
                    row.push(Value::String(minimal_precision_string(value)));
                    row.push(Value::String("]".to_string()));
                    row.push(Value::String("  ".to_string()));
                }

                row.push(Value::Value(value));
            }

            grid.add(row);
        }

        Flat::new(
            minimum_value..maximum_value,
            config.width_hint,
            config.abbreviate_breakdown,
            grid,
        )
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
