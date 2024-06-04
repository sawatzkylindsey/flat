use crate::abbreviate::find_abbreviations;
use crate::Aggregate;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::iter;
use std::ops::Range;

/// The general configuration for rendering a `flat` widget.
///
/// ### Example
/// ```
/// # use flat::{Aggregate, HistogramConfig, Render};
/// let config = Render {
///     aggregate: Aggregate::Average,
///     width_hint: 10,
///     show_aggregate: true,
///     abbreviate_breakdown: true,
///     positive_marker: '+',
///     negative_marker: '-',
///     widget_config: HistogramConfig::default(),
///     ..Render::default()
/// };
/// ```
#[derive(Debug)]
pub struct Render<C> {
    /// The function to apply when aggregating values in the widget.
    ///
    /// Default: `Aggregate::Sum`.
    pub aggregate: Aggregate,
    /// The hint to use to determine the width of the rendering.
    /// `Flat` will try to make the rendering at most `width_hint` wide, with some exceptions:
    /// * If the rendering can reasonably fit in a smaller width, the `width_hint` is ignored.
    /// * If the rendering cannot reasonably fit the `width_hint`, then it is minimally extended (such that a reasonable rendering may be produced).
    ///
    /// Default: `160`.
    pub width_hint: usize,
    /// Whether to show the aggregated result for the *primary* dimension of the dataset.
    /// While the *rendered* data in `flat` uses a relative representation, this option extends the widget to show the absolute values of the data.
    ///
    /// | Show Aggregate | Rendering of Aggregate |
    /// |-|-|
    /// | aggregate([1, 2, 3, 4]) | aggregate([1, 2, 3, 4]) |
    ///
    /// In the case of a breakdown, this represents the aggregate applied to the breakdown aggregates.
    ///
    /// | Show Aggregate | Rendering of Aggregate of A | Rendering of Aggregate of B |
    /// |-|-|-|
    /// | aggregate([aggregate([1, 2, 3]), aggregate(\[4\])]) | aggregate([1, 2, 3]) | aggregate(\[4\]) |
    ///
    /// Default: `false`.
    pub show_aggregate: bool,
    /// Whether to abbreviate the column headings (which come from dimensional values) in the breakdown or not.
    /// Use this option when the breakdown dimensions have long `std::fmt::Display` forms.
    /// Abbreviation is attempted irrespective of the `width_hint`.
    ///
    /// Default: `false`.
    pub abbreviate_breakdown: bool,
    /// The marker character for positive values of the rendering.
    ///
    /// Default: `'*'`.
    pub positive_marker: char,
    /// The marker character for negative values of the rendering.
    ///
    /// Default: `'⊖'`.
    pub negative_marker: char,
    /// The widget specific rendering configuration.
    ///
    /// Default: `C::default()`.
    pub widget_config: C,
}

impl<C: Default> Default for Render<C> {
    fn default() -> Self {
        Self {
            aggregate: Aggregate::Sum,
            width_hint: 160,
            show_aggregate: false,
            abbreviate_breakdown: false,
            positive_marker: '*',
            negative_marker: '⊖',
            widget_config: C::default(),
        }
    }
}

#[derive(Debug)]
struct Config {
    width_hint: usize,
    abbreviate_breakdown: bool,
    positive_marker: char,
    negative_marker: char,
}

impl<T> From<Render<T>> for Config {
    fn from(value: Render<T>) -> Self {
        Self {
            width_hint: value.width_hint,
            abbreviate_breakdown: value.abbreviate_breakdown,
            positive_marker: value.positive_marker,
            negative_marker: value.negative_marker,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Column {
    alignment: Alignment,
    column_type: ColumnType,
}

#[derive(Debug)]
pub(crate) enum ColumnType {
    String(usize),
    Count,
    Breakdown,
}

impl Column {
    pub fn string(alignment: Alignment) -> Self {
        Self {
            alignment,
            column_type: ColumnType::String(0),
        }
    }

    pub fn count(alignment: Alignment) -> Self {
        Self {
            alignment,
            column_type: ColumnType::Count,
        }
    }

    pub fn breakdown(alignment: Alignment) -> Self {
        Self {
            alignment,
            column_type: ColumnType::Breakdown,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Alignment {
    Left,
    Center,
    Right,
}

impl Column {
    fn write(
        &self,
        f: &mut Formatter<'_>,
        cell: &Cell,
        view: &View,
        width: usize,
        overflow_width_override: Option<&usize>,
    ) -> std::fmt::Result {
        let mut width = match &self.column_type {
            ColumnType::String(width) => *width,
            ColumnType::Count | ColumnType::Breakdown => width,
        };

        if matches!(cell.value, Value::Overflow(_)) {
            if let Some(w) = overflow_width_override {
                width = *w;
            }
        }

        let is_breakdown = match &self.column_type {
            ColumnType::Breakdown => true,
            _ => false,
        };

        match &self.alignment {
            Alignment::Left => {
                write!(f, "{:<width$}", cell.value.render(&view, is_breakdown))
            }
            Alignment::Center => {
                write!(f, "{:^width$}", cell.value.render(&view, is_breakdown))
            }
            Alignment::Right => {
                write!(f, "{:>width$}", cell.value.render(&view, is_breakdown))
            }
        }
    }

    fn write_final(
        &self,
        f: &mut Formatter<'_>,
        cell: &Cell,
        view: &View,
        width: usize,
    ) -> std::fmt::Result {
        let width = match &self.column_type {
            ColumnType::String(width) => *width,
            ColumnType::Count | ColumnType::Breakdown => width,
        };

        let is_breakdown = match &self.column_type {
            ColumnType::Breakdown => true,
            _ => false,
        };

        if let Value::Plain(value) = &cell.value {
            write!(f, "{value}")
        } else {
            match &self.alignment {
                Alignment::Left => {
                    write!(f, "{}", cell.value.render(&view, is_breakdown))
                }
                Alignment::Center => {
                    let render_string = cell.value.render(&view, is_breakdown);
                    let render_length = render_string.chars().count();

                    if render_length < width {
                        let left = (width - render_length) / 2;
                        write!(f, "{:left$}{}", "", render_string)
                    } else {
                        write!(f, "{}", cell.value.render(&view, is_breakdown))
                    }
                }
                Alignment::Right => {
                    write!(f, "{:>width$}", cell.value.render(&view, is_breakdown))
                }
            }
        }
    }

    fn fill(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = match &self.column_type {
            ColumnType::String(width) => *width,
            ColumnType::Count | ColumnType::Breakdown => {
                unreachable!("should never fill a breakdown");
            }
        };

        write!(f, "{:width$}", "")
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Cell {
    column: usize,
    value: Value,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Empty,
    String(String),
    Overflow(String),
    Plain(String),
    Value(f64),
    Skip,
}

impl Value {
    fn render_width(&self) -> Option<usize> {
        match &self {
            Value::Empty => Some(0),
            Value::String(string) | Value::Overflow(string) => Some(string.chars().count()),
            Value::Plain(_) | Value::Value(_) | Value::Skip => None,
        }
    }

    fn render(&self, view: &View, is_breakdown: bool) -> String {
        match &self {
            Value::Empty => "".to_string(),
            Value::Skip => unreachable!("must never call render for a Skip"),
            Value::String(string) => {
                if is_breakdown {
                    match view.breakdown_abbreviations.get(string) {
                        Some(abbr) => abbr.clone(),
                        None => string.clone(),
                    }
                } else {
                    string.clone()
                }
            }
            Value::Overflow(string) | Value::Plain(string) => string.clone(),
            Value::Value(value) => {
                let value = value.round();

                let marker = if value.is_sign_positive() {
                    view.positive_marker
                } else {
                    view.negative_marker
                };

                iter::repeat(marker)
                    .take((value.abs() * view.scale) as usize)
                    .collect::<String>()
            }
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Columns {
    types: Vec<Column>,
}

impl Columns {
    pub fn push(&mut self, column: Column) {
        self.types.push(column);
    }

    fn get(&self, index: usize) -> &Column {
        &self.types[index]
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.types.get_mut(index)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Row {
    cells: Vec<Cell>,
}

impl Row {
    pub fn push(&mut self, value: Value) {
        self.cells.push(Cell {
            column: self.cells.len(),
            value,
        });
    }
}

#[derive(Debug, PartialEq)]
struct Overflow {
    width: usize,
    columns: Vec<usize>,
}

#[derive(Debug)]
pub(crate) struct Grid {
    columns: Columns,
    rows: Vec<HashMap<usize, Cell>>,
    overflows: Vec<Overflow>,
    breakdown_values: HashSet<String>,
    /// The minimum width of all the columns (before abbreviation).
    minimum_breakdown_width: usize,
    /// The maximum width of all the columns (before abbreviation).
    maximum_breakdown_width: usize,
}

impl Grid {
    pub fn new(columns: Columns) -> Self {
        Self {
            columns,
            rows: Vec::default(),
            overflows: Vec::default(),
            breakdown_values: HashSet::default(),
            minimum_breakdown_width: usize::MAX,
            maximum_breakdown_width: usize::MIN,
        }
    }

    pub fn add(&mut self, row: Row) {
        assert!(!row.cells.is_empty());
        let mut overflow_width: Option<usize> = None;
        let mut overflow_columns = Vec::default();

        for (j, cell) in row.cells.iter().enumerate() {
            if overflow_width.is_some() {
                if matches!(&cell.value, Value::Skip) {
                    overflow_columns.push(j);
                } else {
                    self.overflows.push(Overflow {
                        width: overflow_width.take().unwrap(),
                        columns: overflow_columns.clone(),
                    });
                }
            }

            let column = self
                .columns
                .get_mut(j)
                .expect(format!("All columns must be accounted for, missing: {j}.").as_str());

            match &mut column.column_type {
                ColumnType::String(width) => {
                    if let Some(cell_width) = cell.value.render_width() {
                        if matches!(&cell.value, Value::Overflow(_)) {
                            overflow_width.replace(cell_width);
                            overflow_columns = vec![j];
                        } else {
                            if &cell_width > width {
                                *width = cell_width;
                            }
                        }
                    }
                }
                ColumnType::Breakdown => {
                    if let Value::String(value) = &cell.value {
                        self.breakdown_values.insert(value.clone());
                    }

                    if let Some(cell_width) = cell.value.render_width() {
                        if &cell_width < &self.minimum_breakdown_width {
                            self.minimum_breakdown_width = cell_width;
                        }

                        if &cell_width > &self.maximum_breakdown_width {
                            self.maximum_breakdown_width = cell_width;
                        }
                    }
                }
                ColumnType::Count => {
                    // do nothing
                }
            }
        }

        if overflow_width.is_some() {
            self.overflows.push(Overflow {
                width: overflow_width.take().unwrap(),
                columns: overflow_columns.clone(),
            });
        }

        self.rows
            .push(row.cells.into_iter().map(|c| (c.column, c)).collect());
    }

    /// Build the overflow overrides, which is a map from column index to "override width".
    fn build_overflow_overrides(&mut self) -> HashMap<usize, usize> {
        let mut overflow_overrides = HashMap::default();

        for overflow in &self.overflows {
            let mut remainder = overflow.width;
            let mut excess = 0;

            for column in &overflow.columns {
                if let ColumnType::String(width) = self.columns.get(*column).column_type {
                    if remainder >= width {
                        remainder -= width;
                    } else if remainder == 0 {
                        excess += width;
                    } else {
                        excess += width - remainder;
                        remainder = 0;
                    }
                }
            }

            if remainder > 0 {
                assert_eq!(excess, 0);
                // The overflow is longer than the combination of the columns it spans.
                // Our simple algorithm is to always just add the remainder to the last column.
                let column = self
                    .columns
                    .get_mut(*overflow.columns.last().unwrap())
                    .unwrap();

                if let ColumnType::String(ref mut width) = column.column_type {
                    *width += remainder;
                }
            } else if excess > 0 {
                assert_eq!(remainder, 0);
                // The overflow is shorter than the combination of the columns it spans.
                // Remember the override to use it when printing the overflow itself.
                overflow_overrides
                    .insert(*overflow.columns.first().unwrap(), overflow.width + excess);
            } else {
                // They are dead even - do nothing!
                assert_eq!(excess, 0);
                assert_eq!(remainder, 0);
            }
        }

        overflow_overrides
    }
}

/// The textual representation of data.
/// This is always produced by calling `.render(config)` on a widget.
/// Use [`std::fmt::Display`] to materialize the flat rendering.
///
/// ```
/// use flat::*;
///
/// let schema = Schemas::one("Animal");
/// let builder = Dataset::builder(schema)
///     .add(("whale".to_string(), ))
///     .add(("shark".to_string(), ))
///     .add(("shark".to_string(), ))
///     .add(("tiger".to_string(), ))
///     .add(("tiger".to_string(), ))
///     .add(("tiger".to_string(), ));
/// let view = builder.view_count();
/// let flat = BarChart::new(&view).render(Render::default());
/// assert_eq!(
///     format!("\n{}", flat.to_string()),
///     r#"
/// Animal  |Sum(Count)
/// shark   |**
/// tiger   |***
/// whale   |*"#);
/// ```
#[derive(Debug)]
pub struct Flat {
    config: Config,
    value_range: Range<f64>,
    grid: Grid,
    overflow_overrides: HashMap<usize, usize>,
}

impl Flat {
    pub(crate) fn new<C>(render: Render<C>, value_range: Range<f64>, mut grid: Grid) -> Self {
        let overflow_overrides = grid.build_overflow_overrides();

        Self {
            config: render.into(),
            value_range,
            grid,
            overflow_overrides,
        }
    }
}

impl Display for Flat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut view_width = self.config.width_hint.saturating_sub(
            self.grid
                .columns
                .types
                .iter()
                .filter_map(|c| match &c.column_type {
                    ColumnType::String(width) => Some(width),
                    ColumnType::Count | ColumnType::Breakdown => None,
                })
                .sum(),
        );
        let view_columns = self
            .grid
            .columns
            .types
            .iter()
            .map(|c| match &c.column_type {
                ColumnType::String(_) => 0,
                ColumnType::Count | ColumnType::Breakdown => 1,
            })
            .sum();
        view_width = view_width.saturating_div(view_columns);

        if view_width < 2 {
            view_width = 2;
        }

        let mut value_width = std::cmp::max(
            self.value_range.start.abs().round() as i128,
            self.value_range.end.abs().round() as i128,
        );

        if value_width == 0 {
            value_width = 1;
        }

        let mut scale = view_width as f64 / value_width as f64;
        let width: usize = if scale >= 1.0 {
            scale = 1.0;
            value_width as usize
        } else {
            view_width
        };

        let (abbreviation_width, breakdown_abbreviations) = match (
            self.config.abbreviate_breakdown,
            view_width > self.grid.maximum_breakdown_width,
        ) {
            // We're supposed to abbreviate, but we don't need to.
            (true, true) => find_abbreviations(
                self.grid.minimum_breakdown_width,
                self.grid.maximum_breakdown_width,
                &self.grid.breakdown_values,
            ),
            // We're supposed to abbreviate and we need to.
            (true, false) => find_abbreviations(
                view_width,
                self.grid.maximum_breakdown_width,
                &self.grid.breakdown_values,
            ),
            // We're not supposed to abbreviate.
            (false, _) => (self.grid.maximum_breakdown_width, HashMap::default()),
        };

        let width = std::cmp::max(width, abbreviation_width);
        let view = View {
            breakdown_abbreviations,
            scale,
            positive_marker: self.config.positive_marker,
            negative_marker: self.config.negative_marker,
        };

        for (i, row) in self.grid.rows.iter().enumerate() {
            let filled_row = filled(row);
            let filled_row_length = filled_row.len();

            for (j, wrapped_cell) in filled_row.into_iter() {
                match wrapped_cell {
                    WrappedCell::Cell(cell) => {
                        if j + 1 == filled_row_length {
                            self.grid
                                .columns
                                .get(j)
                                .write_final(f, cell, &view, width)?;
                        } else {
                            self.grid.columns.get(j).write(
                                f,
                                cell,
                                &view,
                                width,
                                self.overflow_overrides.get(&j),
                            )?;
                        }
                    }
                    WrappedCell::Skip => {
                        // Do nothing.
                    }
                    WrappedCell::Fill => {
                        self.grid.columns.get(j).fill(f)?;
                    }
                }
            }

            if i + 1 != self.grid.rows.len() {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct View {
    breakdown_abbreviations: HashMap<String, String>,
    scale: f64,
    positive_marker: char,
    negative_marker: char,
}

#[derive(Debug)]
enum WrappedCell<'a> {
    Cell(&'a Cell),
    Skip,
    Fill,
}

fn filled<'a>(rows: &'a HashMap<usize, Cell>) -> Vec<(usize, WrappedCell)> {
    let maximum_j: usize = *rows.keys().max().expect("Row must not be empty");
    let mut out = Vec::default();
    let mut candidates = Vec::default();

    for j in 0..=maximum_j {
        let optional_cell = rows.get(&&j);

        match &optional_cell {
            Some(cell) => match &cell {
                Cell {
                    value: Value::Empty,
                    ..
                } => {
                    candidates.push((j, WrappedCell::Cell(cell)));
                }
                Cell {
                    value: Value::Skip, ..
                } => {
                    candidates.push((j, WrappedCell::Skip));
                }
                Cell { .. } => {
                    candidates.drain(..).for_each(|item| out.push(item));
                    out.push((j, WrappedCell::Cell(cell)));
                }
            },
            None => {
                candidates.push((j, WrappedCell::Fill));
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_empty() {
        let view = View {
            breakdown_abbreviations: Default::default(),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };

        let value = Value::Empty;
        assert_eq!(value.render_width(), Some(0));
        assert_eq!(value.render(&view, false), "");
        assert_eq!(value.render(&view, true), "");
    }

    #[test]
    fn render_string() {
        let view = View {
            breakdown_abbreviations: Default::default(),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };

        let value = Value::String("abc".to_string());
        assert_eq!(value.render_width(), Some(3));
        assert_eq!(value.render(&view, false), "abc");
        assert_eq!(value.render(&view, true), "abc");

        let view = View {
            breakdown_abbreviations: HashMap::from([("abc".to_string(), "12345".to_string())]),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };
        assert_eq!(value.render_width(), Some(3));
        assert_eq!(value.render(&view, false), "abc");
        assert_eq!(value.render(&view, true), "12345");
    }

    #[test]
    fn render_overflow() {
        let view = View {
            breakdown_abbreviations: Default::default(),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };

        let value = Value::Overflow("abc".to_string());
        assert_eq!(value.render_width(), Some(3));
        assert_eq!(value.render(&view, false), "abc");
        assert_eq!(value.render(&view, true), "abc");

        let view = View {
            breakdown_abbreviations: HashMap::from([("abc".to_string(), "12345".to_string())]),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };
        assert_eq!(value.render_width(), Some(3));
        assert_eq!(value.render(&view, false), "abc");
        assert_eq!(value.render(&view, true), "abc");
    }

    #[test]
    fn render_plain() {
        let view = View {
            breakdown_abbreviations: Default::default(),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };

        let value = Value::Plain("abc".to_string());
        assert_eq!(value.render_width(), None);
        assert_eq!(value.render(&view, false), "abc");
        assert_eq!(value.render(&view, true), "abc");
    }

    #[test]
    fn render_value() {
        let view = View {
            breakdown_abbreviations: Default::default(),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };

        let value = Value::Value(1.49);
        assert_eq!(value.render_width(), None);
        assert_eq!(value.render(&view, false), "+");
        assert_eq!(value.render(&view, true), "+");

        let value = Value::Value(1.5);
        assert_eq!(value.render_width(), None);
        assert_eq!(value.render(&view, false), "++");
        assert_eq!(value.render(&view, true), "++");

        let view = View {
            breakdown_abbreviations: HashMap::default(),
            scale: 2.0,
            positive_marker: '+',
            negative_marker: '-',
        };
        let value = Value::Value(1.49);
        assert_eq!(value.render_width(), None);
        assert_eq!(value.render(&view, false), "++");
        assert_eq!(value.render(&view, true), "++");

        let value = Value::Value(-1.49);
        assert_eq!(value.render_width(), None);
        assert_eq!(value.render(&view, false), "--");
        assert_eq!(value.render(&view, true), "--");
    }

    #[test]
    fn render_width_skip() {
        let value = Value::Skip;
        assert_eq!(value.render_width(), None);
    }

    #[test]
    #[should_panic]
    fn render_skip() {
        let view = View {
            breakdown_abbreviations: Default::default(),
            scale: 1.0,
            positive_marker: '+',
            negative_marker: '-',
        };

        let value = Value::Skip;
        value.render(&view, true);
    }

    #[test]
    #[should_panic]
    fn grid_empty_row() {
        let columns = Columns::default();
        let mut grid = Grid::new(columns);
        let row = Row::default();
        grid.add(row);
    }

    #[test]
    #[should_panic]
    fn grid_unmatched_column() {
        let columns = Columns::default();
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::String("abc".to_string()));
        grid.add(row);
    }

    // ColumnType::String
    #[test]
    fn grid_add_string_string() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::String("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::String("abc".to_string()),
                }
            )])]
        );

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 3);
        } else {
            panic!("unexpected column type");
        }

        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 3);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_overflow() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert_eq!(
            grid.overflows,
            vec![Overflow {
                width: 3,
                columns: vec![0]
            }]
        );
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Overflow("abc".to_string()),
                }
            )])]
        );

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 0);
        } else {
            panic!("unexpected column type");
        }

        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 3);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_overflow_skip() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("abc".to_string()));
        row.push(Value::Skip);

        // Execute
        grid.add(row);

        // Verify
        assert_eq!(
            grid.overflows,
            vec![Overflow {
                width: 3,
                columns: vec![0, 1]
            }]
        );
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([
                (
                    0,
                    Cell {
                        column: 0,
                        value: Value::Overflow("abc".to_string()),
                    }
                ),
                (
                    1,
                    Cell {
                        column: 1,
                        value: Value::Skip,
                    }
                )
            ])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 0);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_overflow_skip_next() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        columns.push(Column::string(Alignment::Center));
        columns.push(Column::count(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("abc".to_string()));
        row.push(Value::Skip);
        row.push(Value::Value(0.0));

        // Execute
        grid.add(row);

        // Verify
        assert_eq!(
            grid.overflows,
            vec![Overflow {
                width: 3,
                columns: vec![0, 1]
            }]
        );
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([
                (
                    0,
                    Cell {
                        column: 0,
                        value: Value::Overflow("abc".to_string()),
                    }
                ),
                (
                    1,
                    Cell {
                        column: 1,
                        value: Value::Skip,
                    }
                ),
                (
                    2,
                    Cell {
                        column: 2,
                        value: Value::Value(0.0),
                    }
                )
            ])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 0);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_plain() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Plain("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Plain("abc".to_string()),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 0);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_empty() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Empty);

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Empty,
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 0);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_skip() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Skip);

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Skip,
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 0);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_value() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Value(0.0));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Value(0.0),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 0);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_overflow_remainder() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("qwerty".to_string()));

        // Execute
        grid.add(row);
        let mut row = Row::default();
        row.push(Value::String("abc".to_string()));
        grid.add(row);

        // Verify
        assert_eq!(
            grid.overflows,
            vec![Overflow {
                width: 6,
                columns: vec![0]
            }]
        );
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![
                HashMap::from([(
                    0,
                    Cell {
                        column: 0,
                        value: Value::Overflow("qwerty".to_string()),
                    }
                )]),
                HashMap::from([(
                    0,
                    Cell {
                        column: 0,
                        value: Value::String("abc".to_string()),
                    }
                )])
            ]
        );

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 3);
        } else {
            panic!("unexpected column type");
        }

        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 6);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_overflow_excess() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("abc".to_string()));

        // Execute
        grid.add(row);
        let mut row = Row::default();
        row.push(Value::String("qwerty".to_string()));
        grid.add(row);

        // Verify
        assert_eq!(
            grid.overflows,
            vec![Overflow {
                width: 3,
                columns: vec![0]
            }]
        );
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![
                HashMap::from([(
                    0,
                    Cell {
                        column: 0,
                        value: Value::Overflow("abc".to_string()),
                    }
                )]),
                HashMap::from([(
                    0,
                    Cell {
                        column: 0,
                        value: Value::String("qwerty".to_string()),
                    }
                )])
            ]
        );

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 6);
        } else {
            panic!("unexpected column type");
        }

        assert_eq!(grid.build_overflow_overrides(), HashMap::from([(0, 6)]));
        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 6);
        } else {
            panic!("unexpected column type");
        }
    }

    #[test]
    fn grid_add_string_overflow_deadeven() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::string(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("qwerty".to_string()));

        // Execute
        grid.add(row);
        let mut row = Row::default();
        row.push(Value::String("qwerty".to_string()));
        grid.add(row);

        // Verify
        assert_eq!(
            grid.overflows,
            vec![Overflow {
                width: 6,
                columns: vec![0]
            }]
        );
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![
                HashMap::from([(
                    0,
                    Cell {
                        column: 0,
                        value: Value::Overflow("qwerty".to_string()),
                    }
                )]),
                HashMap::from([(
                    0,
                    Cell {
                        column: 0,
                        value: Value::String("qwerty".to_string()),
                    }
                )])
            ]
        );

        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 6);
        } else {
            panic!("unexpected column type");
        }

        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
        if let ColumnType::String(size) = grid.columns.types[0].column_type {
            assert_eq!(size, 6);
        } else {
            panic!("unexpected column type");
        }
    }

    // ColumnType::Count
    #[test]
    fn grid_add_count_string() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::count(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::String("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::String("abc".to_string()),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_count_overflow() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::count(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Overflow("abc".to_string()),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_count_plain() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::count(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Plain("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Plain("abc".to_string()),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_count_empty() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::count(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Empty);

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Empty,
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_count_skip() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::count(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Skip);

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Skip,
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_count_value() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::count(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Value(0.0));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Value(0.0),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    // ColumnType::Breakdown
    #[test]
    fn grid_add_breakdown_string() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::breakdown(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::String("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert_eq!(grid.breakdown_values, HashSet::from(["abc".to_string()]));
        assert_eq!(grid.minimum_breakdown_width, 3);
        assert_eq!(grid.maximum_breakdown_width, 3);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::String("abc".to_string()),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_breakdown_overflow() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::breakdown(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Overflow("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 3);
        assert_eq!(grid.maximum_breakdown_width, 3);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Overflow("abc".to_string()),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_breakdown_plain() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::breakdown(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Plain("abc".to_string()));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Plain("abc".to_string()),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_breakdown_empty() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::breakdown(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Empty);

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 0);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Empty,
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_breakdown_skip() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::breakdown(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Skip);

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Skip,
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }

    #[test]
    fn grid_add_breakdown_value() {
        // Setup
        let mut columns = Columns::default();
        columns.push(Column::breakdown(Alignment::Center));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();
        row.push(Value::Value(0.0));

        // Execute
        grid.add(row);

        // Verify
        assert!(grid.overflows.is_empty());
        assert!(grid.breakdown_values.is_empty());
        assert_eq!(grid.minimum_breakdown_width, 18446744073709551615);
        assert_eq!(grid.maximum_breakdown_width, 0);
        assert_eq!(
            grid.rows,
            vec![HashMap::from([(
                0,
                Cell {
                    column: 0,
                    value: Value::Value(0.0),
                }
            )])]
        );
        assert_eq!(grid.build_overflow_overrides(), HashMap::default());
    }
}
