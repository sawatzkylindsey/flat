use crate::render::{Alignment, Column, Flat, Grid, Render, Row, Value};
use crate::{Binnable, HistogramSchematic};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, Sub};

pub struct Histogram<S: HistogramSchematic> {
    schema: S,
    bins: usize,
    data: Vec<(S::Dimensions, u64)>,
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

    pub fn add(mut self, dims: S::Dimensions, value: u64) -> Histogram<S> {
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

        self.data.push((dims, value));
        self
    }

    pub fn render(self, config: Render) -> Flat {
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
        let mut counts: Vec<HashMap<S::BreakdownDimension, u64>> =
            (0..bins).map(|_| HashMap::default()).collect();
        let mut sort_breakdowns: Vec<S::BreakdownDimension> = Vec::default();
        let mut maximum_count: u64 = 0;

        for (dims, v) in data {
            let primary_dim = self.schema.primary_dim(&dims);
            let breakdown_dim = self.schema.breakdown_dim(&dims);
            // TODO: Fix for performance
            let (index, _) = bin_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(&primary_dim))
                .expect(format!("key must map to one of the aggregating bins").as_str());
            let count = counts[index].entry(breakdown_dim.clone()).or_default();
            *count += v;

            if *count > maximum_count {
                maximum_count = *count;
            }

            if !sort_breakdowns.contains(&breakdown_dim) {
                sort_breakdowns.push(breakdown_dim);
            }
        }

        sort_breakdowns.sort();

        let mut columns = vec![
            // histogram range
            Column::string(0, Alignment::Left),
            // spacer "  "
            Column::string(1, Alignment::Center),
        ];

        if config.show_total {
            // total left [
            columns.push(Column::string(columns.len(), Alignment::Center));
            // total value
            columns.push(Column::string(columns.len(), Alignment::Right));
            // total right ]
            columns.push(Column::string(columns.len(), Alignment::Center));
        }

        if self.schema.is_breakdown() {
            // breakdown left |
            columns.push(Column::string(columns.len(), Alignment::Center));

            for i in 0..sort_breakdowns.len() {
                // aggregate count
                columns.push(Column::breakdown(columns.len(), Alignment::Center));

                if i + 1 < sort_breakdowns.len() {
                    // spacer " "
                    columns.push(Column::string(columns.len(), Alignment::Left));
                }
            }

            // breakdown right |
            columns.push(Column::string(columns.len(), Alignment::Center));
        } else {
            // aggregate count
            columns.push(Column::count(columns.len(), Alignment::Left));
        }

        let mut grid = Grid::new(columns);
        let mut row = Row::default();

        row.push(Value::String(self.schema.primary_header()));
        row.push(Value::Empty);

        if config.show_total {
            row.push(Value::Empty);
            row.push(Value::Empty);
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

        for (bounds, count) in bin_ranges.into_iter().zip(counts) {
            let mut row = Row::default();
            row.push(Value::String(bounds.to_string()));
            row.push(Value::String("  ".to_string()));

            if config.show_total {
                row.push(Value::String("[".to_string()));
                row.push(Value::String(format!("{}", count.values().sum::<u64>())));
                row.push(Value::String("] ".to_string()));
            }

            if self.schema.is_breakdown() {
                row.push(Value::String("|".to_string()));

                for (k, breakdown_dim) in sort_breakdowns.iter().enumerate() {
                    let count = *count.get(breakdown_dim).unwrap_or(&0);
                    row.push(Value::Count(count));

                    if k + 1 != sort_breakdowns.len() {
                        row.push(Value::String(" ".to_string()));
                    }
                }

                row.push(Value::String("|".to_string()));
            } else {
                let count = count.get(&sort_breakdowns[0]).unwrap_or(&0);
                row.push(Value::Count(*count));
            }

            grid.add(row);
        }

        Flat::new(maximum_count, config.render_width, grid)
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
