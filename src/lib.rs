use std::fmt::{Display, Formatter};
use std::iter;

pub struct Flat {
    rows: Vec<Row>,
    maximum_value: u32,
    render_width: u32,
    show_values: bool,
}

impl Display for Flat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let scale: usize = self.maximum_value.div_ceil(self.render_width) as usize;
        let show: Option<usize> = if self.show_values {
            Some((self.maximum_value as f64).log10().ceil() as usize)
        } else {
            None
        };
        let mut keys = Vec::with_capacity(self.rows.len());
        let mut key_width = 0;

        for row in self.rows.iter() {
            let key_string = row.render_key();

            if key_width < key_string.len() {
                key_width = key_string.len();
            }

            keys.push(key_string);
        }

        for (i, (row, key)) in self.rows.iter().zip(keys).enumerate() {
            if i + 1 == self.rows.len() {
                write!(f, "{key:key_width$}  {}", row.render_value(scale, show))?;
            } else {
                write!(f, "{key:key_width$}  {}\n", row.render_value(scale, show))?;
            }
        }

        Ok(())
    }
}

pub struct Row {
    key: String,
    value: u32,
}

impl Row {
    fn render_key(&self) -> String {
        format!("{}", self.key)
    }

    fn render_value(&self, scale: usize, show: Option<usize>) -> String {
        let show = match show {
            Some(width) => {
                format!("[{:>width$}] ", self.value)
            }
            None => "".to_string(),
        };
        format!(
            "{}{}",
            show,
            iter::repeat('*')
                .take((self.value as usize).div_ceil(scale))
                .collect::<String>(),
        )
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {}", self.key, self.value)
    }
}

pub trait Dimensions {}

pub struct OneD<T> {
    a: T,
}

impl<T> OneD<T> {
    pub fn new(a: T) -> OneD<T> {
        Self { a }
    }
}

impl<T> Dimensions for OneD<T> {}

pub struct TwoD<T, U> {
    a: T,
    b: U,
}

impl<T, U> TwoD<T, U> {
    pub fn new(a: T, b: U) -> TwoD<T, U> {
        Self { a, b }
    }
}

impl<T, U> Dimensions for TwoD<T, U> {}

pub struct Categorical<D: Dimensions> {
    data: Vec<(D, u32)>,
}

impl<D: Dimensions> Categorical<D> {
    pub fn builder() -> Categorical<D> {
        todo!()
    }

    pub fn add(self, key: D, value: u32) -> Categorical<D> {
        self
    }

    pub fn render(self) -> Flat {
        todo!()
    }
}

pub struct Histogram {
    bins: usize,
    data: Vec<(f64, u32)>,
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

    pub fn add(mut self, key: f64, value: u32) -> Histogram {
        if key < self.min {
            self.min = key;
        }

        if key > self.max {
            self.max = key;
        }

        self.data.push((key, value));
        self
    }

    pub fn render(self, show_values: bool) -> Flat {
        let Self {
            bins,
            mut data,
            mut min,
            mut max,
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
        let mut values: Vec<u32> = (0..bins).map(|_| 0).collect();
        let mut maximum_value: u32 = 0;

        for (k, v) in data {
            let (index, _) = bin_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(&k))
                .expect(format!("Key '{k}' must map to one of the bins").as_str());
            values[index] += v;

            if values[index] > maximum_value {
                maximum_value = values[index];
            }
        }

        Flat {
            rows: bin_ranges
                .into_iter()
                .zip(values)
                .map(|(bounds, value)| Row {
                    key: bounds.to_string(),
                    value,
                })
                .collect(),
            maximum_value,
            render_width: 20,
            show_values,
        }
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
