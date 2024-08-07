use crate::abbreviate::find_abbreviations;
use crate::aggregate::{aggregate_apply, minimal_precision_string};
use crate::render::{Alignment, Column, Columns, Grid, Row, Value};
use crate::{DagChartConfig, Dimensions, Schema, View};
use crate::{Flat, Render};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
// We use this in the doc strings.
#[allow(unused_imports)]
use crate::PathChart;

/// The dag-chart widget.
///
/// A dag-chart represents each unique tuple of the view's display dimensions as a path in a directed acyclic graph.
/// Paths with common sub-paths, starting from the primary dimension (1st), are collapsed in the frame.
///
/// See also: [`PathChart`]
///
/// ```
/// use flat::*;
///
/// let schema = Schemas::two("Animal", "Size");
/// let dataset = DatasetBuilder::new(schema)
///     .add(("whale".to_string(), "large".to_string()))
///     .add(("shark".to_string(), "medium".to_string()))
///     .add(("shark".to_string(), "small".to_string()))
///     .add(("tiger".to_string(), "medium".to_string()))
///     .add(("tiger".to_string(), "medium".to_string()))
///     .add(("tiger".to_string(), "small".to_string()))
///     .build();
/// let view = dataset.count();
/// let flat = DagChart::new(&view)
///     .render(Render::default());
/// assert_eq!(
///     format!("\n{}", flat.to_string()),
///     r#"
/// Size      Animal  |Sum(Count)
/// medium  - shark   |**
/// small   ┘
/// medium  - tiger   |***
/// small   ┘
/// large   - whale   |*"#);
/// ```
pub struct DagChart<'a, S, V>
where
    S: Schema,
    V: View<S>,
{
    view: &'a V,
    _phantom: PhantomData<S>,
}

impl<'a, S, V> DagChart<'a, S, V>
where
    S: Schema,
    V: View<S>,
    <V as View<S>>::PrimaryDimension: Clone + PartialEq + Eq + Hash,
    <V as View<S>>::BreakdownDimension: Clone + Display + PartialEq + Eq + Hash + Ord,
    <V as View<S>>::DisplayDimensions: Clone + PartialEq + Eq + Hash + Ord,
{
    /// Construct a dag-chart widget from the provided view.
    pub fn new(view: &'a V) -> Self {
        Self {
            view,
            _phantom: PhantomData::default(),
        }
    }

    /// Generate the flat rendering for this dag-chart.
    pub fn render(self, config: Render<DagChartConfig>) -> Flat {
        let mut aggregate_values: HashMap<(V::PrimaryDimension, V::BreakdownDimension), Vec<f64>> =
            HashMap::default();
        let mut partial_aggregate_values: HashMap<String, Vec<f64>> = HashMap::default();
        let mut full_paths: HashSet<String> = HashSet::default();
        let mut display_dimensions: Vec<V::DisplayDimensions> = Vec::default();
        let mut sort_breakdowns: Vec<V::BreakdownDimension> = Vec::default();
        let mut lookup: HashMap<
            V::DisplayDimensions,
            (V::PrimaryDimension, V::BreakdownDimension),
        > = HashMap::default();
        let mut dimension_values: Vec<HashSet<String>> = (0..self.view.display_headers().len())
            .map(|_| HashSet::default())
            .collect();
        let mut path_occurrences: HashMap<String, usize> = HashMap::default();

        for dims in self.view.dataset().data() {
            let value = self.view.value(dims);
            let primary_dim = self.view.primary_dim(dims);
            let breakdown_dims = self.view.breakdown_dim(dims);
            let aggregate_dims = (primary_dim.clone(), breakdown_dims.clone());
            let display_dims = self.view.display_dims(dims);
            let full_path = display_dims
                .as_strings()
                .iter()
                .fold(String::default(), |acc, part| acc + part + ";");

            for (j, value) in display_dims.as_strings().into_iter().enumerate() {
                dimension_values[j].insert(value);
            }

            // Only count the occurrences once per 'full path'.
            // This is because we might have multiple entries, for example:
            // ```
            // DatasetBuilder::new(schema)
            //     .add(("whale".to_string(), 4u32), 2)
            //     .add(("whale".to_string(), 4u32), 3)
            // ```
            if !full_paths.contains(&full_path) {
                full_paths.insert(full_path);

                for dag_index in 0..display_dims.len() {
                    let partial_path = display_dims.as_strings()[0..dag_index + 1]
                        .iter()
                        .fold(String::default(), |acc, part| acc + part + ";");
                    path_occurrences
                        .entry(partial_path)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }

            if config.widget_config.show_aggregate {
                for dag_index in 1..display_dims.len() {
                    let partial_path = display_dims.as_strings()[0..dag_index + 1]
                        .iter()
                        .fold(String::default(), |acc, part| acc + part + ";");
                    let values = partial_aggregate_values.entry(partial_path).or_default();
                    values.push(value);
                }
            }

            let values = aggregate_values.entry(aggregate_dims.clone()).or_default();
            values.push(value);

            if !lookup.contains_key(&display_dims) {
                // Notice, the breakdown_dim will be different in the case of an `is_breakdown` schema.
                // But in that case, we don't actually use the breakdown from `lookup`.
                // We really only need this so we can get the `Nothing` breakdown for non-`is_breakdown` schemas.
                lookup.insert(
                    display_dims.clone(),
                    (primary_dim.clone(), breakdown_dims.clone()),
                );
                display_dimensions.push(display_dims);
            }

            if !sort_breakdowns.contains(&breakdown_dims) {
                sort_breakdowns.push(breakdown_dims);
            }
        }

        display_dimensions.sort();
        sort_breakdowns.sort();

        let mut dimension_abbreviations: Vec<HashMap<String, String>> =
            (0..self.view.display_headers().len())
                .map(|_| HashMap::default())
                .collect();

        if config.widget_config.abbreviate {
            let headers = self.view.display_headers();
            let max_header_length = headers.iter().map(|h| h.chars().count()).max().unwrap();

            for (dag_index, values) in dimension_values.iter().enumerate() {
                let min_header_length = headers[dag_index].to_string().chars().count();
                let (_, abbreviations) =
                    find_abbreviations(min_header_length, max_header_length, values);
                dimension_abbreviations[dag_index] = abbreviations;
            }
        }

        let mut columns = Columns::default();

        for j in 0..self.view.display_headers().len() {
            // dimension value
            columns.push(Column::string(Alignment::Left));

            if j + 1 < self.view.display_headers().len() {
                // spacer " "
                columns.push(Column::string(Alignment::Center));

                if config.widget_config.show_aggregate {
                    // total left [
                    columns.push(Column::string(Alignment::Left));
                    // total value
                    columns.push(Column::string(Alignment::Right));
                    // total right ]
                    columns.push(Column::string(Alignment::Left));
                }

                // dag marker " . "
                columns.push(Column::string(Alignment::Center));
            }
        }

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

        if self.view.breakdown_label().is_some() {
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

        if let Some(breakdown_header) = self.view.breakdown_label() {
            let value_label = self.view.value_label();

            if value_label == breakdown_header {
                let pre_header = build_preheader(
                    &config,
                    self.view.display_headers().len(),
                    &breakdown_header,
                    true,
                );
                grid.add(pre_header);
            } else {
                let pre_header1 = build_preheader(
                    &config,
                    self.view.display_headers().len(),
                    &breakdown_header,
                    false,
                );
                grid.add(pre_header1);
                let pre_header2 = build_preheader(
                    &config,
                    self.view.display_headers().len(),
                    &value_label,
                    true,
                );
                grid.add(pre_header2);
            }
        }

        let mut header = Row::default();

        for (j, name) in self.view.display_headers().iter().rev().enumerate() {
            header.push(Value::String(name.clone()));

            if j + 1 < self.view.display_headers().len() {
                header.push(Value::String(" ".to_string()));

                if config.widget_config.show_aggregate {
                    header.push(Value::Overflow(config.aggregate.to_string()));
                    header.push(Value::Skip);
                    header.push(Value::Skip);
                }

                // For the dag marker " . "
                header.push(Value::String("   ".to_string()));
            }
        }

        if config.show_aggregate {
            header.push(Value::Empty);
            header.push(Value::Overflow(config.aggregate.to_string()));
            header.push(Value::Skip);
            header.push(Value::Skip);
        }

        header.push(Value::String("  ".to_string()));
        header.push(Value::String("|".to_string()));

        if self.view.breakdown_label().is_some() {
            for (k, breakdown_dim) in sort_breakdowns.iter().enumerate() {
                header.push(Value::String(breakdown_dim.to_string()));

                if k + 1 < sort_breakdowns.len() {
                    header.push(Value::String(" ".to_string()));
                }
            }

            header.push(Value::String("|".to_string()));
        } else {
            header.push(Value::Plain(format!(
                "{}({})",
                config.aggregate.to_string(),
                self.view.value_label()
            )));
            // header.push(Value::Plain(config.aggregate.to_string()));
        }

        grid.add(header);
        let mut column_groups: HashMap<usize, Group> = HashMap::default();
        let mut minimum_value = f64::MAX;
        let mut maximum_value = f64::MIN;

        for display_dims in display_dimensions.iter() {
            let path = display_dims.as_strings();
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
                        column_chunks.push(Value::String(
                            dimension_abbreviations[dag_index][part].clone(),
                        ));
                    } else {
                        column_chunks.push(Value::String(part.clone()));
                    }

                    if dag_index == 0 {
                        let (primary_dim, breakdown_dim) = lookup
                            .get(display_dims)
                            .expect("sort dimensions must be mapped to dimensions");

                        if self.view.breakdown_label().is_some() {
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
                                column_chunks.push(Value::String(" ".to_string()));
                                column_chunks.push(Value::String("[".to_string()));
                                column_chunks.push(Value::String(minimal_precision_string(
                                    config.aggregate.apply(breakdown_values.as_slice()),
                                )));
                                column_chunks.push(Value::String("]".to_string()));
                            }

                            column_chunks.push(Value::String("  ".to_string()));
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
                                column_chunks.push(Value::String(" ".to_string()));
                                column_chunks.push(Value::String("[".to_string()));
                                column_chunks.push(Value::String(minimal_precision_string(value)));
                                column_chunks.push(Value::String("]".to_string()));
                            }

                            column_chunks.push(Value::String("  ".to_string()));
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
                        column_chunks.push(Value::String(" ".to_string()));

                        if config.widget_config.show_aggregate {
                            let value = config
                                .aggregate
                                .apply(partial_aggregate_values[&partial_path].as_slice());
                            column_chunks.push(Value::String("[".to_string()));
                            column_chunks.push(Value::String(minimal_precision_string(value)));
                            column_chunks.push(Value::String("]".to_string()));
                        }

                        if let Some(desc_pos) = &descendant_position {
                            match desc_pos {
                                Position::Above => {
                                    column_chunks.push(Value::String(format!("┐")));
                                }
                                Position::At => {
                                    column_chunks.push(Value::String(format!("-")));
                                }
                                Position::Below => {
                                    column_chunks.push(Value::String(format!("┘")));
                                }
                            }
                        }
                    }
                } else if dag_index != 0 {
                    assert!(descendant_position.is_some());

                    if let Some(desc_pos) = &descendant_position {
                        match desc_pos {
                            Position::At => {
                                column_chunks.push(Value::Empty);
                                column_chunks.push(Value::String(" ".to_string()));

                                if config.widget_config.show_aggregate {
                                    column_chunks.push(Value::Empty);
                                    column_chunks.push(Value::Empty);
                                    column_chunks.push(Value::Empty);
                                }
                                column_chunks.push(Value::String(format!("-")));
                            }
                            Position::Above | Position::Below => {
                                // TODO: handle this case
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

fn build_preheader(
    config: &Render<DagChartConfig>,
    columns: usize,
    label: &str,
    embed: bool,
) -> Row {
    let mut row = Row::default();

    for j in 0..columns {
        row.push(Value::Empty);

        if j + 1 < columns {
            row.push(Value::Empty);

            if config.widget_config.show_aggregate {
                row.push(Value::Empty);
                row.push(Value::Empty);
                row.push(Value::Empty);
            }

            row.push(Value::Empty);
        }
    }

    if config.show_aggregate {
        row.push(Value::Empty);
        row.push(Value::Empty);
        row.push(Value::Empty);
        row.push(Value::Empty);
    }

    row.push(Value::Empty);
    row.push(Value::Empty);

    if embed {
        row.push(Value::Plain(format!(
            "{}({label})",
            config.aggregate.to_string(),
        )));
    } else {
        row.push(Value::Plain(format!("{label}")));
    }

    row
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

    #[cfg(feature = "primitive_impls")]
    mod primitive_impls {
        use crate::{DagChart, DagChartConfig, Render};
        use crate::{DatasetBuilder, Schema1, Schema2, Schemas};

        #[test]
        fn empty() {
            let schema: Schema1<i64> = Schemas::one("abc");
            let builder = DatasetBuilder::new(schema).build();
            let view = builder.reflect_1st();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
abc  |Sum(abc)"#
            );
        }

        #[test]
        fn zero() {
            let schema: Schema1<i64> = Schemas::one("abc");
            let dataset = DatasetBuilder::new(schema).add((0,)).build();
            let view = dataset.reflect_1st();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
abc  |Sum(abc)
0    |"#
            );
        }

        #[test]
        fn negatives_and_positives() {
            let schema: Schema1<i64> = Schemas::one("abc");
            let dataset = DatasetBuilder::new(schema)
                .add((-1,))
                .add((0,))
                .add((1,))
                .build();
            let view = dataset.reflect_1st();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
abc  |Sum(abc)
-1   |⊖
0    |
1    |*"#
            );
        }

        #[test]
        fn one_thousand() {
            let schema: Schema1<i64> = Schemas::one("abc");
            let mut builder = DatasetBuilder::new(schema);

            for _ in 0..1_000 {
                builder.update((1,));
            }

            let dataset = builder.build();
            let view = dataset.reflect_1st();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
abc  |Sum(abc)
1    |**********************************************************************************************************************************************************"#
            );
        }

        #[test]
        fn negative_one_thousand() {
            let schema: Schema1<i64> = Schemas::one("abc");
            let mut builder = DatasetBuilder::new(schema);

            for _ in 0..1_000 {
                builder.update((-1,));
            }

            let dataset = builder.build();
            let view = dataset.reflect_1st();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
abc  |Sum(abc)
-1   |⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖"#
            );
        }

        #[test]
        fn breakdown() {
            let schema: Schema2<u8, u8> = Schemas::two("abc", "something long");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 2))
                .add((2, 3))
                .add((3, 4))
                .build();
            let view = dataset.breakdown_2nd();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
      Sum(something long)
abc  | 2    3    4  |
1    | **           |
2    |     ***      |
3    |          ****|"#
            );
        }

        #[test]
        fn count_breakdown() {
            let schema = Schemas::two("abc", "something long");
            let dataset = DatasetBuilder::new(schema)
                .add((1, 2))
                .add((2, 3))
                .add((3, 4))
                .build();
            let view = dataset.count_breakdown_2nd();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
      something long
      Sum(Count)
abc  |2 3 4|
1    |*    |
2    |  *  |
3    |    *|"#
            );
        }

        #[test]
        fn depth_3_combo111() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema).add(("a1", "b1", "c1")).build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] - b1 [1] - a1 [1]  |*"#
            );
        }

        #[test]
        fn depth_3_combo211() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] - b1 [2] - a1 [2]  |**
c2 [1] ┘"#
            );
        }

        #[test]
        fn depth_3_combo221() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .add(("a1", "b2", "c2"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] - b1 [2] ┐
c2 [1] ┘        - a1 [3]  |***
c2 [1] - b2 [1] ┘"#
            );
        }

        #[test]
        fn depth_3_combo221x() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .add(("a1", "b2", "c1"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] - b1 [2] ┐
c2 [1] ┘        - a1 [3]  |***
c1 [1] - b2 [1] ┘"#
            );
        }

        #[test]
        fn depth_3_combo311() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .add(("a1", "b1", "c3"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] ┐
c2 [1] - b1 [3] - a1 [3]  |***
c3 [1] ┘"#
            );
        }

        #[test]
        fn depth_3_combo321() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .add(("a1", "b1", "c3"))
                .add(("a1", "b2", "c3"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] ┐
c2 [1] - b1 [3] - a1 [4]  |****
c3 [1] ┘
c3 [1] - b2 [1] ┘"#
            );
        }

        #[test]
        fn depth_3_combo321x() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .add(("a1", "b1", "c3"))
                .add(("a1", "b2", "c2"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] ┐
c2 [1] - b1 [3] - a1 [4]  |****
c3 [1] ┘
c2 [1] - b2 [1] ┘"#
            );
        }

        #[test]
        fn depth_3_combo321y() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .add(("a1", "b1", "c3"))
                .add(("a1", "b2", "c1"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] ┐
c2 [1] - b1 [3] - a1 [4]  |****
c3 [1] ┘
c1 [1] - b2 [1] ┘"#
            );
        }

        #[test]
        fn depth_3_combo331() {
            let schema = Schemas::three("A", "B", "C");
            let dataset = DatasetBuilder::new(schema)
                .add(("a1", "b1", "c1"))
                .add(("a1", "b1", "c2"))
                .add(("a1", "b1", "c3"))
                .add(("a1", "b2", "c1"))
                .add(("a1", "b3", "c1"))
                .build();
            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render {
                show_aggregate: true,
                widget_config: DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                },
                ..Render::default()
            });
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
C  Sum   B  Sum   A  Sum  |Sum(Count)
c1 [1] ┐
c2 [1] - b1 [3] ┐
c3 [1] ┘        - a1 [5]  |*****
c1 [1] - b2 [1] ┘
c1 [1] - b3 [1] ┘"#
            );
        }
    }

    #[cfg(feature = "pointer_impls")]
    mod pointer_impls {
        use crate::{DagChart, Render};
        use crate::{DatasetBuilder, Schema2, Schemas};
        use ordered_float::OrderedFloat;

        #[test]
        fn view2() {
            let schema: Schema2<i64, OrderedFloat<f64>> = Schemas::two("abc", "def");
            let dataset = DatasetBuilder::new(schema)
                .add((1, OrderedFloat(0.1)))
                .add((2, OrderedFloat(0.4)))
                .add((3, OrderedFloat(0.5)))
                .add((4, OrderedFloat(0.9)))
                .build();
            let view = dataset.view_2nd();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
abc  |Sum(def)
1    |
2    |
3    |*
4    |*"#
            );

            let view = dataset.count();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
def    abc  |Sum(Count)
0.1  - 1    |*
0.4  - 2    |*
0.5  - 3    |*
0.9  - 4    |*"#
            );

            let view = dataset.breakdown_2nd();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
      Sum(def)
abc  |0.1 0.4 0.5 0.9|
1    |               |
2    |               |
3    |         *     |
4    |             * |"#
            );

            let view = dataset.count_breakdown_2nd();
            let dagchart = DagChart::new(&view);
            let flat = dagchart.render(Render::default());
            assert_eq!(
                format!("\n{}", flat.to_string()),
                r#"
      def
      Sum(Count)
abc  |0.1 0.4 0.5 0.9|
1    | *             |
2    |     *         |
3    |         *     |
4    |             * |"#
            );
        }
    }
}
