use crate::render::{Alignment, Column, Flat, Grid, Render, Row, Value};
use std::fmt::{Display, Formatter};

pub struct Histogram {
    bins: usize,
    data: Vec<(f64, u64)>,
    min: f64,
    max: f64,
}

impl Histogram {
    pub fn builder(bins: usize) -> Self {
        Self {
            bins,
            data: Vec::default(),
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn add(mut self, key: f64, value: u64) -> Histogram {
        if key < self.min {
            self.min = key;
        }

        if key > self.max {
            self.max = key;
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
        } = self;

        let delta = max - min;
        let size = delta / bins as f64;
        let bin_ranges: Vec<Bounds<f64>> = (0..bins)
            .map(|i| {
                if i + 1 == bins {
                    Bounds {
                        lower: Bound::Inclusive(min + (i as f64 * size)),
                        upper: Bound::Inclusive(min + ((i + 1) as f64 * size)),
                    }
                } else {
                    Bounds {
                        lower: Bound::Inclusive(min + (i as f64 * size)),
                        upper: Bound::Exclusive(min + ((i + 1) as f64 * size)),
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
                .expect(format!("Key '{k}' must map to one of the aggregating bins").as_str());
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
