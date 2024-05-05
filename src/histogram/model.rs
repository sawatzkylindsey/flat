use crate::render::{Alignment, Column, Flat, Grid, Render, Row, Value};
use crate::{Binnable, HistogramSchematic};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

pub struct Histogram<S: HistogramSchematic> {
    schema: S,
    bins: usize,
    data: Vec<(S::PrimaryDimension, u64)>,
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

    pub fn add(mut self, key: S::PrimaryDimension, value: u64) -> Histogram<S> {
        let update_min = match &self.min {
            Some(min) => key < *min,
            None => true,
        };

        if update_min {
            self.min.replace(key.clone());
        }

        let update_max = match &self.max {
            Some(max) => key > *max,
            None => true,
        };

        if update_max {
            self.max.replace(key.clone());
        }

        self.data.push((key, value));
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
        let size = delta.ceiling_divide(bins);
        let bin_ranges: Vec<Bounds<S::PrimaryDimension>> = (0..bins)
            .map(|i| {
                if i + 1 == bins {
                    Bounds {
                        lower: Bound::Inclusive(min.clone() + (size.ceiling_multiply(i))),
                        upper: Bound::Inclusive(min.clone() + (size.ceiling_multiply(i + 1))),
                    }
                } else {
                    Bounds {
                        lower: Bound::Inclusive(min.clone() + (size.ceiling_multiply(i))),
                        upper: Bound::Exclusive(min.clone() + (size.ceiling_multiply(i + 1))),
                    }
                }
            })
            .collect();
        let mut counts: Vec<u64> = (0..bins).map(|_| 0).collect();
        let mut maximum_count: u64 = 0;

        for (k, v) in data {
            // TODO: Fix for performance
            let (index, _) = bin_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(&k))
                .expect(format!("key must map to one of the aggregating bins").as_str());
            counts[index] += v;

            if counts[index] > maximum_count {
                maximum_count = counts[index];
            }
        }

        let columns = if config.show_total {
            vec![
                // histogram range
                Column::string(0, Alignment::Left),
                // spacer "  "
                Column::string(1, Alignment::Center),
                // total left [
                Column::string(2, Alignment::Center),
                // total value
                Column::string(3, Alignment::Right),
                // total right ]
                Column::string(4, Alignment::Center),
                // histogram count
                Column::count(5, Alignment::Left),
            ]
        } else {
            vec![
                // histogram range
                Column::string(0, Alignment::Left),
                // spacer "  "
                Column::string(1, Alignment::Center),
                // histogram count
                Column::count(2, Alignment::Left),
            ]
        };
        let mut grid = Grid::new(columns);

        for (bounds, count) in bin_ranges.into_iter().zip(counts) {
            let mut row = Row::default();
            row.push(Value::String(bounds.to_string()));
            row.push(Value::String("  ".to_string()));

            if config.show_total {
                row.push(Value::String("[".to_string()));
                row.push(Value::String(format!("{count}")));
                row.push(Value::String("] ".to_string()));
            }

            row.push(Value::Count(count));
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
