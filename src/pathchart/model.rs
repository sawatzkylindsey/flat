use crate::aggregate::{aggregate_apply, minimal_precision_string};
use crate::pathchart::api::PathChartConfig;
use crate::render::{Alignment, Column, Columns, Grid, Row, Value};
use crate::{Dimensions, Schema, View};
use crate::{Flat, Render};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// The path-chart widget.
///
/// ```
/// use flat::*;
///
/// let schema = Schemas::two("Animal", "Size");
/// let dataset = Dataset::builder(schema)
///     .add(("whale".to_string(), "large".to_string()))
///     .add(("shark".to_string(), "medium".to_string()))
///     .add(("shark".to_string(), "small".to_string()))
///     .add(("tiger".to_string(), "medium".to_string()))
///     .add(("tiger".to_string(), "medium".to_string()))
///     .add(("tiger".to_string(), "small".to_string()))
///     .build();
/// let view = dataset.counting_view();
/// let flat = PathChart::new(&view)
///     .render(Render::default());
/// assert_eq!(
///     format!("\n{}", flat.to_string()),
///     r#"
/// /Animal /Size  |Sum(Count)
/// /shark         |**
///   /medium
///   /small
/// /tiger         |***
///   /medium
///   /small
/// /whale         |*
///   /large"#);
/// ```
pub struct PathChart<'a, S, V>
where
    S: Schema,
    V: View<S>,
{
    view: &'a V,
    _phantom: PhantomData<S>,
}

impl<'a, S, V> PathChart<'a, S, V>
where
    S: Schema,
    V: View<S>,
    <V as View<S>>::PrimaryDimension: Clone + PartialEq + Eq + Hash,
{
    /// Construct a path-chart widget from the provided view.
    pub fn new(view: &'a V) -> Self {
        Self {
            view,
            _phantom: PhantomData::default(),
        }
    }

    /// Generate the flat rendering for this path-chart.
    pub fn render(self, config: Render<PathChartConfig>) -> Flat {
        let mut aggregate_values: HashMap<(V::PrimaryDimension, V::BreakdownDimension), Vec<f64>> =
            HashMap::default();
        let mut partial_aggregate_values: HashMap<String, Vec<f64>> = HashMap::default();
        let mut dag: HashMap<Path, Vec<String>> = HashMap::default();
        let mut sort_breakdowns: Vec<V::BreakdownDimension> = Vec::default();
        let mut lookup: HashMap<String, (V::PrimaryDimension, V::BreakdownDimension)> =
            HashMap::default();

        for dims in self.view.data() {
            let value = self.view.value(dims);
            let primary_dim = self.view.primary_dim(dims);
            let breakdown_dims = self.view.breakdown_dim(dims);
            let aggregate_dims = (primary_dim.clone(), breakdown_dims.clone());
            let display_dims = self.view.display_dims(dims);
            let root = display_dims.as_strings()[0].clone();
            let mut path = Path { path: vec![] };

            for part in display_dims.as_strings() {
                let dirs = dag.entry(path.clone()).or_default();

                if !dirs.contains(&part) {
                    dirs.push(part.clone());
                }

                path.path.push(part);
            }

            if config.show_aggregate {
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

            if !lookup.contains_key(&root) {
                // Notice, the breakdown_dim will be different in the case of an `is_breakdown` schema.
                // But in that case, we don't actually use the breakdown from `lookup`.
                // We really only need this so we can get the `Nothing` breakdown for non-`is_breakdown` schemas.
                lookup.insert(root, (primary_dim.clone(), breakdown_dims.clone()));
            }

            if !sort_breakdowns.contains(&breakdown_dims) {
                sort_breakdowns.push(breakdown_dims);
            }
        }

        sort_breakdowns.sort();

        let mut columns = Columns::default();
        // dimension value
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

            for _ in 0..self.view.display_headers().len() {
                pre_header.push(Value::Empty);
            }

            if config.show_aggregate {
                pre_header.push(Value::Empty);
                pre_header.push(Value::Empty);
                pre_header.push(Value::Empty);
                pre_header.push(Value::Empty);
            }

            pre_header.push(Value::Empty);
            pre_header.push(Value::Empty);
            pre_header.push(Value::Plain(format!(
                "{}({})",
                config.aggregate.to_string(),
                self.view.value_header()
            )));
            grid.add(pre_header);
        }

        let mut header = Row::default();
        let mut combined = String::default();

        for (j, name) in self.view.display_headers().iter().enumerate() {
            combined.push('/');
            combined.push_str(name);

            if j + 1 < self.view.display_headers().len() {
                combined.push(' ');
            }
        }

        header.push(Value::String(combined));

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
            header.push(Value::Plain(format!(
                "{}({})",
                config.aggregate.to_string(),
                self.view.value_header()
            )));
        }

        grid.add(header);
        let mut minimum_value = f64::MAX;
        let mut maximum_value = f64::MIN;

        let mut stack = Vec::default();
        stack.push(Path { path: vec![] });

        while !stack.is_empty() {
            let current = stack.pop().unwrap();

            if !current.path.is_empty() {
                let part = current.path.last().unwrap();
                let depth = current.path.len();
                let width = (depth - 1) * 2;
                let mut row = Row::default();
                row.push(Value::String(format!("{:width$}/{part}", "")));

                if current.path.len() == 1 {
                    let (primary_dim, breakdown_dim) = lookup
                        .get(part)
                        .expect("sort dimensions must be mapped to dimensions");

                    if self.view.is_breakdown() {
                        let breakdown_values: Vec<f64> = sort_breakdowns
                            .iter()
                            .map(|breakdown_dim| {
                                let aggregate_dims = (primary_dim.clone(), breakdown_dim.clone());
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

                            if k + 1 != breakdown_values.len() {
                                row.push(Value::String(" ".to_string()));
                            }
                        }

                        row.push(Value::String("|".to_string()));
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
                            row.push(Value::String(" ".to_string()));
                            row.push(Value::String("[".to_string()));
                            row.push(Value::String(minimal_precision_string(value)));
                            row.push(Value::String("]".to_string()));
                        }

                        row.push(Value::String("  ".to_string()));
                        row.push(Value::String("|".to_string()));
                        row.push(Value::Value(value));

                        if value < minimum_value {
                            minimum_value = value;
                        }

                        if value > maximum_value {
                            maximum_value = value;
                        }
                    }
                } else if config.show_aggregate {
                    let partial_path = current
                        .path
                        .iter()
                        .fold(String::default(), |acc, part| acc + part + ";");

                    let value = config
                        .aggregate
                        .apply(partial_aggregate_values[&partial_path].as_slice());
                    row.push(Value::String(" ".to_string()));
                    row.push(Value::String("[".to_string()));
                    row.push(Value::String(minimal_precision_string(value)));
                    row.push(Value::String("]".to_string()));
                }

                grid.add(row);
            }

            if let Some(children) = dag.get_mut(&current) {
                children.sort();

                for child in children.iter().rev() {
                    let mut sub_path = current.clone();
                    sub_path.path.push(child.clone());
                    stack.push(sub_path);
                }
            }
        }

        Flat::new(config, minimum_value..maximum_value, grid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    path: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Dataset, Schema1, Schema2, Schemas};
    use ordered_float::OrderedFloat;

    #[test]
    fn empty() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let builder = Dataset::builder(schema).build();
        let view = builder.reflective_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/abc  |Sum(abc)"#
        );
    }

    #[test]
    fn zero() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let dataset = Dataset::builder(schema).add((0,)).build();
        let view = dataset.reflective_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/abc  |Sum(abc)
/0    |"#
        );
    }

    #[test]
    fn negatives_and_positives() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let dataset = Dataset::builder(schema)
            .add((-1,))
            .add((0,))
            .add((1,))
            .build();
        let view = dataset.reflective_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/abc  |Sum(abc)
/-1   |⊖
/0    |
/1    |*"#
        );
    }

    #[test]
    fn one_thousand() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let mut builder = Dataset::builder(schema);

        for _ in 0..1_000 {
            builder.update((1,));
        }

        let dataset = builder.build();
        let view = dataset.reflective_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/abc  |Sum(abc)
/1    |*********************************************************************************************************************************************************"#
        );
    }

    #[test]
    fn negative_one_thousand() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let mut builder = Dataset::builder(schema);

        for _ in 0..1_000 {
            builder.update((-1,));
        }

        let dataset = builder.build();
        let view = dataset.reflective_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/abc  |Sum(abc)
/-1   |⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖⊖"#
        );
    }

    #[test]
    fn breakdown() {
        let schema = Schemas::two("abc", "something long");
        let dataset = Dataset::builder(schema)
            .add((1, 2))
            .add((2, 3))
            .add((3, 4))
            .build();
        let view = dataset.breakdown_2nd();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
       Sum(Breakdown(something long))
/abc  |2 3 4|
/1    |*    |
/2    |  *  |
/3    |    *|"#
        );
    }

    #[test]
    fn depth_3_combo111() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema).add(("a1", "b1", "c1")).build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [1]  |*
  /b1    [1]
    /c1  [1]"#
        );
    }

    #[test]
    fn depth_3_combo211() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [2]  |**
  /b1    [2]
    /c1  [1]
    /c2  [1]"#
        );
    }

    #[test]
    fn depth_3_combo221() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b2", "c2"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [3]  |***
  /b1    [2]
    /c1  [1]
    /c2  [1]
  /b2    [1]
    /c2  [1]"#
        );
    }

    #[test]
    fn depth_3_combo221x() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b2", "c1"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [3]  |***
  /b1    [2]
    /c1  [1]
    /c2  [1]
  /b2    [1]
    /c1  [1]"#
        );
    }

    #[test]
    fn depth_3_combo311() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b1", "c3"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [3]  |***
  /b1    [3]
    /c1  [1]
    /c2  [1]
    /c3  [1]"#
        );
    }

    #[test]
    fn depth_3_combo321() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b1", "c3"))
            .add(("a1", "b2", "c3"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [4]  |****
  /b1    [3]
    /c1  [1]
    /c2  [1]
    /c3  [1]
  /b2    [1]
    /c3  [1]"#
        );
    }

    #[test]
    fn depth_3_combo321x() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b1", "c3"))
            .add(("a1", "b2", "c2"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [4]  |****
  /b1    [3]
    /c1  [1]
    /c2  [1]
    /c3  [1]
  /b2    [1]
    /c2  [1]"#
        );
    }

    #[test]
    fn depth_3_combo321y() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b1", "c3"))
            .add(("a1", "b2", "c1"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [4]  |****
  /b1    [3]
    /c1  [1]
    /c2  [1]
    /c3  [1]
  /b2    [1]
    /c1  [1]"#
        );
    }

    #[test]
    fn depth_3_combo331() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            .add(("a1", "b1", "c1"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b1", "c3"))
            .add(("a1", "b2", "c1"))
            .add(("a1", "b3", "c1"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [5]  |*****
  /b1    [3]
    /c1  [1]
    /c2  [1]
    /c3  [1]
  /b2    [1]
    /c1  [1]
  /b3    [1]
    /c1  [1]"#
        );
    }

    #[test]
    fn depth_3_combo331x() {
        let schema = Schemas::three("A", "B", "C");
        let dataset = Dataset::builder(schema)
            // different order
            .add(("a1", "b3", "c1"))
            .add(("a1", "b2", "c1"))
            .add(("a1", "b1", "c3"))
            .add(("a1", "b1", "c2"))
            .add(("a1", "b1", "c1"))
            .build();
        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/A /B /C Sum  |Sum(Count)
/a1      [5]  |*****
  /b1    [3]
    /c1  [1]
    /c2  [1]
    /c3  [1]
  /b2    [1]
    /c1  [1]
  /b3    [1]
    /c1  [1]"#
        );
    }

    #[test]
    fn view2() {
        let schema: Schema2<i64, OrderedFloat<f64>> = Schemas::two("abc", "def");
        let dataset = Dataset::builder(schema)
            .add((1, OrderedFloat(0.1)))
            .add((2, OrderedFloat(0.4)))
            .add((3, OrderedFloat(0.5)))
            .add((4, OrderedFloat(0.9)))
            .build();
        let view = dataset.view_2nd();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/abc  |Sum(def)
/1    |
/2    |
/3    |*
/4    |*"#
        );

        let view = dataset.counting_view();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
/abc /def  |Sum(Count)
/1         |*
  /0.1
/2         |*
  /0.4
/3         |*
  /0.5
/4         |*
  /0.9"#
        );

        let view = dataset.breakdown_2nd();
        let barchart = PathChart::new(&view);
        let flat = barchart.render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
       Sum(Breakdown(def))
/abc  |0.1 0.4 0.5 0.9|
/1    | *             |
/2    |     *         |
/3    |         *     |
/4    |             * |"#
        );
    }
}
