use crate::categorical::api::CategoricalSchematic;
use crate::render::{Alignment, Column, Grid, Row, Value};
use crate::Dimensions;
use crate::{Flat, Render};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub struct Categorical<S: CategoricalSchematic> {
    schema: S,
    data: Vec<(S::Dimensions, u64)>,
}

impl<S: CategoricalSchematic> Categorical<S>
where
    S: CategoricalSchematic,
    <S as CategoricalSchematic>::PrimaryDimension: Clone + PartialEq + Eq + Hash,
    <S as CategoricalSchematic>::BreakdownDimension: Clone + Display + PartialEq + Eq + Hash + Ord,
    <S as CategoricalSchematic>::SortDimensions: Dimensions + Clone + PartialEq + Eq + Hash + Ord,
{
    pub fn builder(schema: S) -> Categorical<S> {
        Self {
            schema,
            data: Vec::default(),
        }
    }

    pub fn add(mut self, key: S::Dimensions, value: u64) -> Categorical<S> {
        self.data.push((key, value));
        self
    }

    pub fn render(self, config: Render) -> Flat {
        let mut counts: HashMap<(S::PrimaryDimension, S::BreakdownDimension), u64> =
            HashMap::default();
        let mut full_paths: HashSet<String> = HashSet::default();
        let mut sort_dimensions: Vec<S::SortDimensions> = Vec::default();
        let mut sort_breakdowns: Vec<S::BreakdownDimension> = Vec::default();
        let mut lookup: HashMap<S::SortDimensions, (S::PrimaryDimension, S::BreakdownDimension)> =
            HashMap::default();
        let mut path_occurrences: HashMap<String, usize> = HashMap::default();
        let mut maximum_count: u64 = 0;

        for (dims, v) in self.data.iter() {
            let primary_dim = self.schema.primary_dim(dims);
            let breakdown_dims = self.schema.breakdown_dim(dims);
            let aggregate_dims = (primary_dim.clone(), breakdown_dims.clone());
            let sort_dims = self.schema.sort_dims(dims);
            let full_path = sort_dims
                .as_strings()
                .iter()
                .fold(String::default(), |acc, part| acc + part + ";");

            // Only count the occurrences once per 'full path'.
            // This is because we might have multiple entries, for example:
            // ```
            // Categorical::builder(schema)
            //     .add(("whale".to_string(), 4u32), 2)
            //     .add(("whale".to_string(), 4u32), 3)
            // ```
            if !full_paths.contains(&full_path) {
                full_paths.insert(full_path);

                for dag_index in 0..sort_dims.len() {
                    let path = sort_dims.as_strings()[0..dag_index + 1]
                        .iter()
                        .fold(String::default(), |acc, part| acc + part + ";");
                    path_occurrences
                        .entry(path)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }

            let count = counts.entry(aggregate_dims.clone()).or_default();
            *count += v;

            if *count > maximum_count {
                maximum_count = *count;
            }

            if !lookup.contains_key(&sort_dims) {
                // Notice, the breakdown_dim will be different in the case of an `is_breakdown` schema.
                // But in that case, we don't actually use the breakdown from `lookup`.
                // We really only need this so we can get the `Nothing` breakdown for non-`is_breakdown` schemas.
                lookup.insert(
                    sort_dims.clone(),
                    (primary_dim.clone(), breakdown_dims.clone()),
                );
                sort_dimensions.push(sort_dims);
            }

            if !sort_breakdowns.contains(&breakdown_dims) {
                sort_breakdowns.push(breakdown_dims);
            }
        }

        sort_dimensions.sort();
        sort_breakdowns.sort();
        let mut columns = Vec::default();

        for _ in 0..self.schema.sort_headers().len() {
            // dimension value
            columns.push(Column::string(columns.len(), Alignment::Left));
            // spacer "  "
            columns.push(Column::string(columns.len(), Alignment::Left));
        }

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

        for name in self.schema.sort_headers().iter().rev() {
            row.push(Value::String(name.clone()));
            row.push(Value::Empty);
        }

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
        let mut column_groups: HashMap<usize, Group> = HashMap::default();

        for sort_dims in sort_dimensions.iter() {
            let chain = sort_dims.as_strings();
            let mut column_chunks_reversed: Vec<Vec<Value>> = Vec::default();
            let mut descendant_position = None;

            #[allow(unused_doc_comments)]
            /// Run through the chain in dag index ascending order, which
            /// is the "rendering" reverse order.
            ///
            /// For this example dag, we'll iterate as follows:
            /// a - b ┐
            /// c ┐   - d
            /// e - f ┘
            /// h ┘
            ///
            /// chain: ["d", "b", "a"]
            /// dag_index | j | part | path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "b"   | "d|b"
            /// 2        | 0 | "a"   | "d|b|a"
            ///
            /// chain: ["d", "f", "c"]
            /// dag_index | j | part | path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d|f"
            /// 2        | 0 | "c"   | "d|f|c"
            ///
            /// chain: ["d", "f", "e"]
            /// dag_index | j | part | path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d|f"
            /// 2        | 0 | "e"   | "d|f|e"
            ///
            /// etc..
            ///
            for (dag_index, part) in chain.clone().iter().enumerate() {
                let j = chain.len() - dag_index - 1;
                let path = chain[0..dag_index + 1]
                    .iter()
                    .fold(String::default(), |acc, part| acc + part + ";");
                let group = column_groups.entry(j).or_default();

                if group.matches(&path) {
                    group.increment();
                } else {
                    group.swap(path.clone());
                }

                let occurrences = path_occurrences[&path];
                let position = (occurrences as f64 / 2.0).ceil() as usize - 1;
                let mut column_chunks = Vec::default();

                let position = match position {
                    position if position > group.index => Position::Above,
                    position if position == group.index => Position::At,
                    _ => Position::Below,
                };

                if position == Position::At {
                    column_chunks.push(Value::String(format!("{part} ")));

                    if dag_index == 0 {
                        let (primary_dim, breakdown_dim) = lookup
                            .get(sort_dims)
                            .expect("sort dimensions must be mapped to dimensions");
                        column_chunks.push(Value::String(format!("  ")));

                        if self.schema.is_breakdown() {
                            let breakdown_counts: Vec<u64> = sort_breakdowns
                                .iter()
                                .map(|breakdown_dim| {
                                    let aggregate_dims =
                                        (primary_dim.clone(), breakdown_dim.clone());
                                    *counts.get(&aggregate_dims).unwrap_or(&0)
                                })
                                .collect();

                            if config.show_total {
                                column_chunks.push(Value::String("[".to_string()));
                                column_chunks.push(Value::String(format!(
                                    "{}",
                                    breakdown_counts.iter().sum::<u64>()
                                )));
                                column_chunks.push(Value::String("] ".to_string()));
                            }

                            column_chunks.push(Value::String("|".to_string()));

                            for (k, breakdown_count) in breakdown_counts.iter().enumerate() {
                                column_chunks.push(Value::Count(*breakdown_count));

                                if k + 1 != breakdown_counts.len() {
                                    column_chunks.push(Value::String(" ".to_string()));
                                }
                            }

                            column_chunks.push(Value::String("|".to_string()));
                        } else {
                            let aggregate_dims = (primary_dim.clone(), breakdown_dim.clone());
                            let count = *counts.get(&aggregate_dims).unwrap_or(&0);

                            if config.show_total {
                                column_chunks.push(Value::String("[".to_string()));
                                column_chunks.push(Value::String(format!("{count}")));
                                column_chunks.push(Value::String("] ".to_string()));
                            }

                            column_chunks.push(Value::Count(count));
                        }
                    } else {
                        assert!(descendant_position.is_some());

                        if let Some(desc_pos) = &descendant_position {
                            match desc_pos {
                                Position::Above => {
                                    column_chunks.push(Value::String(format!(" ┐")));
                                }
                                Position::At => {
                                    column_chunks.push(Value::String(format!(" - ")));
                                }
                                Position::Below => {
                                    column_chunks.push(Value::String(format!(" ┘")));
                                }
                            }
                        }
                    }
                }

                descendant_position.replace(position);
                column_chunks_reversed.push(column_chunks);
            }

            let mut row = Row::default();

            for column_chunks in column_chunks_reversed.into_iter().rev() {
                for value in column_chunks.into_iter() {
                    row.push(value);
                }
            }

            grid.add(row);
        }

        Flat::new(maximum_count, config.render_width, grid)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Position {
    Above,
    At,
    Below,
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
    fn matches(&self, path: &String) -> bool {
        match &self.locus {
            Some(l) => l == path,
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
