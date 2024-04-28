use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Render {
    pub render_width: usize,
    pub show_values: bool,
}

impl Default for Render {
    fn default() -> Self {
        Self {
            render_width: 120,
            show_values: false,
        }
    }
}

pub struct Flat {
    rows: Vec<Row>,
    maximum_value: usize,
    config: Render,
}

impl Display for Flat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut keys = Vec::with_capacity(self.rows.len());
        let mut key_width = 0;

        for row in self.rows.iter() {
            let key = match row.render_key() {
                Some(key_string) => {
                    let len = key_string.chars().count();

                    if key_width < len {
                        key_width = len;
                    }

                    Some(key_string)
                }
                None => None,
            };

            keys.push(key);
        }

        let mut show_width = 0;

        let show: Option<usize> = if self.config.show_values {
            let width = (self.maximum_value as f64).log10().ceil() as usize;
            show_width = width + 3;
            Some(width)
        } else {
            None
        };

        let mut value_width = self
            .config
            .render_width
            .saturating_sub(key_width + 2 + show_width);

        if value_width < 2 {
            value_width = 2;
        }

        let scale: usize = self.maximum_value.div_ceil(value_width);

        for (i, (row, key)) in self.rows.iter().zip(keys).enumerate() {
            match (key, row.render_value(scale, show)) {
                (Some(key_string), Some(value_string)) => {
                    if i + 1 == self.rows.len() {
                        write!(f, "{key_string:key_width$}  {value_string}")?;
                    } else {
                        write!(f, "{key_string:key_width$}  {value_string}\n")?;
                    }
                }
                (Some(key_string), None) => {
                    if i + 1 == self.rows.len() {
                        write!(f, "{key_string}")?;
                    } else {
                        write!(f, "{key_string}\n")?;
                    }
                }
                _ => {
                    if i + 1 == self.rows.len() {
                        write!(f, "")?;
                    } else {
                        write!(f, "\n")?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Row {
    Full { key: String, value: usize },
    Partial { key: String },
    Empty,
}

impl Row {
    fn render_key(&self) -> Option<String> {
        match &self {
            Row::Full { key, .. } => Some(format!("{}", key)),
            Row::Partial { key, .. } => Some(format!("{}", key)),
            Row::Empty => None,
        }
    }

    fn render_value(&self, scale: usize, show: Option<usize>) -> Option<String> {
        match &self {
            Row::Full { value, .. } => {
                let show = match show {
                    Some(width) => {
                        format!("[{:>width$}] ", value)
                    }
                    None => "".to_string(),
                };
                Some(format!(
                    "{}{}",
                    show,
                    iter::repeat('*')
                        .take(value.div_ceil(scale))
                        .collect::<String>(),
                ))
            }
            Row::Partial { .. } | Row::Empty => None,
        }
    }
}

pub trait Schematic {
    type Dims: Dimensions;

    fn width(&self) -> usize;

    fn columns(&self) -> Vec<String>;
    // fn values(&self, t: T) -> Self::Dims {
}

pub struct Schema;

pub struct Schema1<T> {
    one: PhantomData<T>,
    column_1: String,
}

pub struct Schema2<T, U> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    breakdown: Option<String>,
}

pub struct Schema3<T, U, V> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    three: PhantomData<V>,
    column_3: String,
    breakdown: Option<String>,
}

// pub struct Schema4<T, U, V, W>;
// pub struct Schema5<T, U, V, W, X>;
// pub struct Schema6<T, U, V, W, X, Y>;
// pub struct Schema7<T, U, V, W, X, Y, Z>;

impl Schema {
    pub fn one<T>(column_1: String) -> Schema1<T> {
        Schema1 {
            one: PhantomData,
            column_1,
        }
    }

    pub fn two<T, U>(column_1: impl Into<String>, column_2: impl Into<String>) -> Schema2<T, U> {
        Schema2 {
            one: PhantomData,
            column_1: column_1.into(),
            two: PhantomData,
            column_2: column_2.into(),
            breakdown: None,
        }
    }

    pub fn three<T, U, V>(
        column_1: String,
        column_2: String,
        column_3: String,
    ) -> Schema3<T, U, V> {
        Schema3 {
            one: PhantomData,
            column_1,
            two: PhantomData,
            column_2,
            three: PhantomData,
            column_3,
            breakdown: None,
        }
    }
}

impl<T> Schematic for Schema1<T>
where
    T: Display + Ord,
{
    type Dims = (T,);

    fn width(&self) -> usize {
        1
    }

    fn columns(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }
}

// impl<T> Schema1<T> {
//     pub fn values(&self, t: T) -> (T,) {
//         (t,)
//     }
// }

pub enum Breakdown2 {
    Second,
}

impl<T, U> Schematic for Schema2<T, U>
where
    T: Display + Ord,
    U: Display + Ord,
{
    type Dims = (T, U);

    fn width(&self) -> usize {
        2
    }

    fn columns(&self) -> Vec<String> {
        vec![self.column_1.clone(), self.column_2.clone()]
    }
}

impl<T, U> Schema2<T, U> {
    pub fn breakdown(mut self, breakdown: Breakdown2) -> Self {
        self.breakdown = match breakdown {
            Breakdown2::Second => Some(self.column_2.clone()),
        };
        self
    }

    // pub fn values(&self, t: T, u: U) -> (T, U) {
    //     (t, u)
    // }
}

pub enum Breakdown3 {
    Second,
    Third,
}

impl<T, U, V> Schematic for Schema3<T, U, V>
where
    T: Display + Ord,
    U: Display + Ord,
    V: Display + Ord,
{
    // TODO
    type Dims = (T, U);

    fn width(&self) -> usize {
        3
    }

    fn columns(&self) -> Vec<String> {
        vec![
            self.column_1.clone(),
            self.column_2.clone(),
            self.column_3.clone(),
        ]
    }
}

impl<T, U, V> Schema3<T, U, V> {
    pub fn breakdown(mut self, breakdown: Breakdown3) -> Self {
        self.breakdown = match breakdown {
            Breakdown3::Second => Some(self.column_2.clone()),
            Breakdown3::Third => Some(self.column_3.clone()),
        };
        self
    }

    // pub fn values(&self, t: T, u: U) -> (T, U) {
    //     (t, u)
    // }
}

pub trait Dimensions: Ord {
    // fn locus(&self) -> String;

    fn chain(&self) -> Vec<String>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneD<T> {
    a: T,
}

impl<T> OneD<T> {
    pub fn new(a: T) -> OneD<T> {
        Self { a }
    }
}
//
// impl<T> Display for OneD<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "asdf")
//     }
// }

impl<T: Display + Ord> Dimensions for OneD<T> {
    // fn locus(&self) -> String {
    //     self.a.to_string()
    // }

    fn chain(&self) -> Vec<String> {
        vec![self.a.to_string()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoD<T, U>
where
    T: Ord,
    U: Ord,
{
    primary: T,
    secondary: U,
}

impl<T, U> TwoD<T, U>
where
    T: Ord,
    U: Ord,
{
    pub fn new(primary: T, secondary: U) -> TwoD<T, U> {
        Self { primary, secondary }
    }
}
//
// impl<T, U> Display for TwoD<T, U>
// where
//     T: Ord,
//     U: Ord,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "asdf")
//     }
// }

impl<T, U> PartialOrd for TwoD<T, U>
where
    T: PartialOrd + Ord,
    U: PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &TwoD<T, U>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, U> Ord for TwoD<T, U>
where
    T: Ord,
    U: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.primary
            .cmp(&other.primary)
            .then(self.secondary.cmp(&other.secondary))
    }
}

impl<T, U> Dimensions for TwoD<T, U>
where
    T: Display + Ord,
    U: Display + Ord,
{
    // fn locus(&self) -> String {
    //     self.primary.to_string()
    // }

    fn chain(&self) -> Vec<String> {
        vec![self.primary.to_string(), self.secondary.to_string()]
    }
}
//
// impl<T, U> PartialOrd for (T, U)
// where
//     T: PartialOrd + Ord,
//     U: PartialOrd + Ord,
// {
//     fn partial_cmp(&self, other: &TwoD<T, U>) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl<T, U> Ord for (T, U)
// where
//     T: Ord,
//     U: Ord,
// {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.0.cmp(&other.0).then(self.1.cmp(&other.1))
//     }
// }

impl<T> Dimensions for (T,)
where
    T: Display + Ord,
{
    // fn locus(&self) -> String {
    //     self.0.to_string()
    // }

    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl<T, U> Dimensions for (T, U)
where
    T: Display + Ord,
    U: Display + Ord,
{
    // fn locus(&self) -> String {
    //     self.0.to_string()
    // }

    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreeD<T, U, V>
where
    T: Ord,
    U: Ord,
    V: Ord,
{
    primary: T,
    secondary: U,
    tertiary: V,
}

impl<T, U, V> ThreeD<T, U, V>
where
    T: Ord,
    U: Ord,
    V: Ord,
{
    pub fn new(primary: T, secondary: U, tertiary: V) -> ThreeD<T, U, V> {
        Self {
            primary,
            secondary,
            tertiary,
        }
    }
}

// impl<T, U, V> Display for ThreeD<T, U, V>
// where
//     T: Ord,
//     U: Ord,
//     V: Ord,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "asdf")
//     }
// }

impl<T, U, V> PartialOrd for ThreeD<T, U, V>
where
    T: PartialOrd + Ord,
    U: PartialOrd + Ord,
    V: PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &ThreeD<T, U, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, U, V> Ord for ThreeD<T, U, V>
where
    T: Ord,
    U: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.primary
            .cmp(&other.primary)
            .then(self.secondary.cmp(&other.secondary))
            .then(self.tertiary.cmp(&other.tertiary))
    }
}

impl<T, U, V> Dimensions for ThreeD<T, U, V>
where
    T: Display + Ord,
    U: Display + Ord,
    V: Display + Ord,
{
    // fn locus(&self) -> String {
    //     self.primary.to_string()s
    // }

    fn chain(&self) -> Vec<String> {
        vec![
            self.primary.to_string(),
            self.secondary.to_string(),
            self.tertiary.to_string(),
        ]
    }
}

pub const DEFAULT_SEPARATORS: &[&str; 3] = &[">", "-", "~"];

pub struct Categorical<S>
where
    S: Schematic,
    <S as Schematic>::Dims: Dimensions,
{
    schema: S,
    data: Vec<(S::Dims, usize)>,
}

impl<S: Schematic> Categorical<S> {
    pub fn builder(schema: S) -> Categorical<S> {
        Self {
            schema,
            data: Vec::default(),
        }
    }

    pub fn add(mut self, key: S::Dims, value: usize) -> Categorical<S> {
        self.data.push((key, value));
        self
    }

    pub fn render(self, config: Render, separators: &[&str]) -> Flat {
        let mut values: HashMap<String, usize> = self
            .data
            .iter()
            .map(|(k, _)| (k.chain()[0].clone(), 0))
            .collect();
        let mut full_chains: HashSet<String> = HashSet::default();
        let mut sort_keys: Vec<&S::Dims> = Vec::default();
        let ancestor_key = |chain: &[String], dag_index: usize| chain[0..dag_index + 1].join("|");
        let mut ancestors: HashMap<String, usize> = HashMap::default();
        let mut columns = 0;
        let mut column_widths: HashMap<usize, usize> = self
            .schema
            .columns()
            .iter()
            .enumerate()
            .map(|(dag_index, c)| {
                let j = self.schema.width() - dag_index - 1;
                (j, c.len())
            })
            .collect();
        let mut maximum_value: usize = 0;

        for (k, v) in self.data.iter() {
            let chain = k.chain();
            let locus = chain[0].clone();
            let full_chain = chain.join("|");

            if !full_chains.contains(&full_chain) {
                full_chains.insert(full_chain);

                if chain.len() > columns {
                    columns = chain.len();
                }

                for (dag_index, part) in chain.iter().enumerate() {
                    let j = chain.len() - dag_index - 1;
                    let partial = ancestor_key(chain.as_slice(), dag_index);
                    ancestors
                        .entry(partial)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);

                    let width = part.to_string().len();
                    let w = column_widths.entry(j).or_default();

                    if width > *w {
                        *w = width;
                    }
                }
            }

            let aggregate = values
                .get_mut(&locus)
                .expect(format!("Key/locus must map to one of the aggregating rows").as_str());
            *aggregate += v;

            if *aggregate > maximum_value {
                maximum_value = *aggregate;
            }

            if !sort_keys.contains(&k) {
                sort_keys.push(k);
            }
        }

        sort_keys.sort();
        let columns = self.schema.columns();
        let mut rows = vec![Row::Partial {
            key: column_widths
                .iter()
                .map(|(j, width)| format!("{:width$}   ", columns[*j]))
                .collect(),
        }];
        let mut column_groups: HashMap<usize, Group> = HashMap::default();
        let mut separator_index = 0;
        let mut current_locus = None;

        for k in sort_keys.iter() {
            let mut group_value = None;
            let chain = k.chain();
            let locus = chain[0].clone();

            match &mut current_locus {
                Some(l) => {
                    if l != &locus {
                        current_locus.replace(locus.clone());
                        separator_index = (separator_index + 1) % separators.len();
                    }
                }
                None => {
                    current_locus = Some(locus.clone());
                }
            }

            let separator = separators[separator_index];

            let mut key = "".to_string();

            for (j, part) in chain.iter().rev().enumerate() {
                let dag_index = chain.len() - j - 1;
                let width = column_widths[&j];
                let partial = ancestor_key(chain.as_slice(), dag_index);

                let group = column_groups.entry(j).or_default();

                if group.matches(&partial) {
                    group.increment();
                } else {
                    group.swap(partial.clone());
                }

                let count = ancestors[&partial];

                if group.index == (count as f64 / 2.0).ceil() as usize - 1 {
                    if key.is_empty() {
                        key += format!("{part:width$} {separator}").as_str();
                    } else {
                        key += format!(" {part:width$} {separator}").as_str();
                    }

                    if dag_index == 0 {
                        group_value = Some(*values.get(&locus).unwrap());
                    }
                } else {
                    if key.is_empty() {
                        key += format!("{:width$}", "").as_str();
                    } else {
                        key += format!(" {:width$}", "").as_str();
                    }
                }
            }

            match group_value {
                Some(value) => {
                    rows.push(Row::Full {
                        key: key.trim().to_string(),
                        value,
                    });
                }
                None => {
                    rows.push(Row::Partial {
                        key: key.trim().to_string(),
                    });
                }
            }
        }

        Flat {
            rows,
            maximum_value,
            config,
        }
    }
}

#[derive(Debug)]
struct Group {
    locus: Option<String>,
    index: usize,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            locus: None,
            index: 0,
        }
    }
}

impl Group {
    fn matches(&self, locus: &String) -> bool {
        match &self.locus {
            Some(l) => l == locus,
            None => false,
        }
    }

    fn swap(&mut self, locus: String) {
        self.locus.replace(locus);
        self.index = 0;
    }

    fn increment(&mut self) {
        self.index += 1;
    }
}

pub struct Histogram {
    bins: usize,
    data: Vec<(f64, usize)>,
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

    pub fn add(mut self, key: f64, value: usize) -> Histogram {
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
        let mut values: Vec<usize> = (0..bins).map(|_| 0).collect();
        let mut maximum_value: usize = 0;

        for (k, v) in data {
            // TODO: Fix for performance
            let (index, _) = bin_ranges
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(&k))
                .expect(format!("Key '{k}' must map to one of the aggregating bins").as_str());
            values[index] += v;

            if values[index] > maximum_value {
                maximum_value = values[index];
            }
        }

        Flat {
            rows: bin_ranges
                .into_iter()
                .zip(values)
                .map(|(bounds, value)| Row::Full {
                    key: bounds.to_string(),
                    value: value,
                })
                .collect(),
            maximum_value,
            config,
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
