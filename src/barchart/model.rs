use crate::abbreviate::find_abbreviations;
use crate::aggregate::{aggregate_apply, minimal_precision_string};
use crate::barchart::api::BarChartSchematic;
use crate::render::{Alignment, Column, Columns, Grid, Row, Value};
use crate::{BarChartConfig, Dimensions};
use crate::{Flat, Render};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// The bar-chart widget.
///
/// ```
/// use flat::*;
///
/// let schema = Schema::one("Animal");
/// let builder = BarChart::builder(schema)
///     .add(("whale".to_string(),), 0)
///     .add(("shark".to_string(),), 1)
///     .add(("tiger".to_string(),), 4);
/// let flat = builder.render(Render::default());
/// println!("{flat}");
///
/// // Output (modified for alignment)
/// r#"Animal
///    shark   *
///    tiger   ****
///    whale   "#;
/// ```
pub struct BarChart<S: BarChartSchematic> {
    schema: S,
    data: Vec<(S::Dimensions, f64)>,
}

impl<S> BarChart<S>
where
    S: BarChartSchematic,
    <S as BarChartSchematic>::PrimaryDimension: Clone + PartialEq + Eq + Hash,
    <S as BarChartSchematic>::BreakdownDimension: Clone + Display + PartialEq + Eq + Hash + Ord,
    <S as BarChartSchematic>::SortDimensions: Dimensions + Clone + PartialEq + Eq + Hash + Ord,
{
    /// Build a bar-chart widget based off the provided schema.
    pub fn builder(schema: S) -> BarChart<S> {
        Self {
            schema,
            data: Vec::default(),
        }
    }

    /// Update this bar-chart with a data point to (key, value).
    /// Use this method to add data via mutation.
    ///
    /// See also: [`BarChart::add`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schema::one("Animal");
    /// let mut builder = BarChart::builder(schema);
    /// builder.update(("whale".to_string(),), 0);
    /// builder.update(("shark".to_string(),), 1);
    /// builder.update(("tiger".to_string(),), 4);
    /// let flat = builder.render(Render::default());
    /// println!("{flat}");
    ///
    /// // Output (modified for alignment)
    /// r#"Animal
    ///    shark   *
    ///    tiger   ****
    ///    whale   "#;
    /// ```
    pub fn update(&mut self, key: S::Dimensions, value: impl Into<f64>) {
        self.data.push((key, value.into()));
    }

    /// Add a data point to (key, value) to this bar-chart.
    /// Use this method to add data via method chaining.
    ///
    /// See also: [`BarChart::update`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schema::one("Animal");
    /// let builder = BarChart::builder(schema)
    ///     .add(("whale".to_string(),), 0)
    ///     .add(("shark".to_string(),), 1)
    ///     .add(("tiger".to_string(),), 4);
    /// let flat = builder.render(Render::default());
    /// println!("{flat}");
    ///
    /// // Output (modified for alignment)
    /// r#"Animal
    ///    shark   *
    ///    tiger   ****
    ///    whale   "#;
    /// ```
    pub fn add(mut self, key: S::Dimensions, value: impl Into<f64>) -> BarChart<S> {
        self.update(key, value);
        self
    }

    /// Generate the flat rendering for this bar-chart.
    pub fn render(self, config: Render<BarChartConfig>) -> Flat {
        let mut aggregate_values: HashMap<(S::PrimaryDimension, S::BreakdownDimension), Vec<f64>> =
            HashMap::default();
        let mut full_paths: HashSet<String> = HashSet::default();
        let mut sort_dimensions: Vec<S::SortDimensions> = Vec::default();
        let mut sort_breakdowns: Vec<S::BreakdownDimension> = Vec::default();
        let mut lookup: HashMap<S::SortDimensions, (S::PrimaryDimension, S::BreakdownDimension)> =
            HashMap::default();
        let mut dimension_values: Vec<HashSet<String>> = (0..self.schema.headers().len())
            .map(|_| HashSet::default())
            .collect();
        let mut path_occurrences: HashMap<String, usize> = HashMap::default();

        for (dims, v) in self.data.iter() {
            let primary_dim = self.schema.primary_dim(dims);
            let breakdown_dims = self.schema.breakdown_dim(dims);
            let aggregate_dims = (primary_dim.clone(), breakdown_dims.clone());
            let sort_dims = self.schema.sort_dims(dims);
            let full_path = sort_dims
                .as_strings()
                .iter()
                .fold(String::default(), |acc, part| acc + part + ";");

            for (j, value) in sort_dims.as_strings().into_iter().enumerate() {
                dimension_values[j].insert(value);
            }

            // Only count the occurrences once per 'full path'.
            // This is because we might have multiple entries, for example:
            // ```
            // BarChart::builder(schema)
            //     .add(("whale".to_string(), 4u32), 2)
            //     .add(("whale".to_string(), 4u32), 3)
            // ```
            if !full_paths.contains(&full_path) {
                full_paths.insert(full_path);

                for dag_index in 0..sort_dims.len() {
                    let partial_path = sort_dims.as_strings()[0..dag_index + 1]
                        .iter()
                        .fold(String::default(), |acc, part| acc + part + ";");
                    path_occurrences
                        .entry(partial_path)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }

            let values = aggregate_values.entry(aggregate_dims.clone()).or_default();
            values.push(*v);

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

        let mut dimension_abbreviations: Vec<HashMap<String, String>> =
            (0..self.schema.headers().len())
                .map(|_| HashMap::default())
                .collect();

        if config.widget_config.abbreviate {
            let headers = self.schema.headers();
            let max_header_length = headers.iter().map(|h| h.chars().count()).max().unwrap();

            for (dag_index, values) in dimension_values.iter().enumerate() {
                let min_header_length = headers[dag_index].to_string().chars().count();
                let (_, abbreviations) =
                    find_abbreviations(min_header_length, max_header_length, values);
                dimension_abbreviations[dag_index] = abbreviations;
            }
        }

        let mut columns = Columns::default();

        for _ in 0..self.schema.headers().len() {
            // dimension value
            columns.push(Column::string(Alignment::Left));
            // spacer "  "
            columns.push(Column::string(Alignment::Left));
        }

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

        // rendering delimiter |
        columns.push(Column::string(Alignment::Center));

        if self.schema.is_breakdown() {
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

        for name in self.schema.headers().iter().rev() {
            row.push(Value::String(name.clone()));
            row.push(Value::Empty);
        }

        if config.show_aggregate {
            row.push(Value::Overflow(config.aggregate.to_string()));
            row.push(Value::Skip);
            row.push(Value::Skip);
            row.push(Value::Empty);
        }

        row.push(Value::String("|".to_string()));

        if self.schema.is_breakdown() {
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
        let mut minimum_value = f64::MAX;
        let mut maximum_value = f64::MIN;

        for sort_dims in sort_dimensions.iter() {
            let path = sort_dims.as_strings();
            let mut column_chunks_reversed: Vec<Vec<Value>> = Vec::default();
            let mut descendant_position = None;

            #[allow(unused_doc_comments)]
            /// Run through the path in dag index ascending order, which
            /// is the "rendering" reverse order.
            ///
            /// For this example dag, we'll iterate as follows:
            /// a - b ┐
            /// c ┐   - d
            /// e - f ┘
            /// h ┘
            ///
            /// path: ["d", "b", "a"]
            /// dag_index | j | part | partial_path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "b"   | "d;b"
            /// 2        | 0 | "a"   | "d;b;a"
            ///
            /// path: ["d", "f", "c"]
            /// dag_index | j | part | partial_path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d;f"
            /// 2        | 0 | "c"   | "d;f;c"
            ///
            /// path: ["d", "f", "e"]
            /// dag_index | j | part | partial_path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d;f"
            /// 2        | 0 | "e"   | "d;f;e"
            ///
            /// etc..
            ///
            for (dag_index, part) in path.clone().iter().enumerate() {
                let j = path.len() - dag_index - 1;
                let partial_path = path[0..dag_index + 1]
                    .iter()
                    .fold(String::default(), |acc, part| acc + part + ";");
                let group = column_groups.entry(j).or_default();

                if group.matches(&partial_path) {
                    group.increment();
                } else {
                    group.swap(partial_path.clone());
                }

                let occurrences = path_occurrences[&partial_path];
                let position = (occurrences as f64 / 2.0).ceil() as usize - 1;
                let mut column_chunks = Vec::default();

                let position = match position {
                    position if position > group.index => Position::Above,
                    position if position == group.index => Position::At,
                    _ => Position::Below,
                };

                if position == Position::At {
                    if config.widget_config.abbreviate {
                        column_chunks.push(Value::String(format!(
                            "{} ",
                            dimension_abbreviations[dag_index][part]
                        )));
                    } else {
                        column_chunks.push(Value::String(format!("{part} ")));
                    }

                    if dag_index == 0 {
                        let (primary_dim, breakdown_dim) = lookup
                            .get(sort_dims)
                            .expect("sort dimensions must be mapped to dimensions");
                        column_chunks.push(Value::String(format!("  ")));

                        if self.schema.is_breakdown() {
                            let breakdown_values: Vec<f64> = sort_breakdowns
                                .iter()
                                .map(|breakdown_dim| {
                                    let aggregate_dims =
                                        (primary_dim.clone(), breakdown_dim.clone());
                                    aggregate_apply(
                                        &config.aggregate,
                                        &aggregate_values,
                                        &aggregate_dims,
                                        &mut minimum_value,
                                        &mut maximum_value,
                                    )
                                })
                                .collect();

                            if config.show_aggregate {
                                column_chunks.push(Value::String("[".to_string()));
                                column_chunks.push(Value::String(minimal_precision_string(
                                    config.aggregate.apply(breakdown_values.as_slice()),
                                )));
                                column_chunks.push(Value::String("]".to_string()));
                                column_chunks.push(Value::String("  ".to_string()));
                            }

                            column_chunks.push(Value::String("|".to_string()));

                            for (k, breakdown_value) in breakdown_values.iter().enumerate() {
                                column_chunks.push(Value::Value(*breakdown_value));

                                if k + 1 != breakdown_values.len() {
                                    column_chunks.push(Value::String(" ".to_string()));
                                }
                            }

                            column_chunks.push(Value::String("|".to_string()));
                        } else {
                            let aggregate_dims = (primary_dim.clone(), breakdown_dim.clone());
                            let value = aggregate_apply(
                                &config.aggregate,
                                &aggregate_values,
                                &aggregate_dims,
                                &mut minimum_value,
                                &mut maximum_value,
                            );

                            if config.show_aggregate {
                                column_chunks.push(Value::String("[".to_string()));
                                column_chunks.push(Value::String(minimal_precision_string(value)));
                                column_chunks.push(Value::String("]".to_string()));
                                column_chunks.push(Value::String("  ".to_string()));
                            }

                            column_chunks.push(Value::String("|".to_string()));
                            column_chunks.push(Value::Value(value));

                            if value < minimum_value {
                                minimum_value = value;
                            }

                            if value > maximum_value {
                                maximum_value = value;
                            }
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

        Flat::new(config, minimum_value..maximum_value, grid)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Schema, Schema1};

    #[test]
    fn empty() {
        let schema: Schema1<u64> = Schema::one("abc");
        let builder = BarChart::builder(schema);
        let flat = builder.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc|"#
        );
    }

    #[test]
    fn add_zero() {
        let schema: Schema1<u64> = Schema::one("abc");
        let mut builder = BarChart::builder(schema);
        builder.update((1,), 0);
        let flat = builder.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc  |
1    |"#
        );
    }

    #[test]
    fn add_zero_and_ones() {
        let schema: Schema1<u64> = Schema::one("abc");
        let builder = BarChart::builder(schema)
            .add((1,), -1)
            .add((2,), 0)
            .add((3,), 1);
        let flat = builder.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc  |
1    |⊖
2    |
3    |*"#
        );
    }

    #[test]
    fn add_onethousand() {
        let schema: Schema1<u64> = Schema::one("abc");
        let mut builder = BarChart::builder(schema);
        builder.update((1,), 1_000);
        let flat = builder.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc  |
1    |******************************************************************************************************************************************************************************"#
        );
    }

    #[test]
    fn add_negative_onethousand() {
        let schema: Schema1<u64> = Schema::one("abc");
        let mut builder = BarChart::builder(schema);
        builder.update((1,), -1_000);
        let flat = builder.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
abc  |
1    |⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖"#
        );
    }
}
