use crate::abbreviate::find_abbreviations;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::iter;

/// The general configuration for rendering a `flat` chart.
#[derive(Debug)]
pub struct Render<C> {
    /// The hint to use to determine the width of the rendering.
    /// [`Flat`] will try to make the rendering at most `width_hint` wide, with some exceptions:
    /// * If the rendering can reasonably fit in a smaller width, the `width_hint` is ignored.
    /// * If the rendering cannot reasonably fit the `width_hint`, then it is minimally extended (such that a reasonable rendering may be produced).
    pub width_hint: usize,
    /// Whether to show the absolute total for the *primary* dimension of the dataset.
    /// While the *rendered* data in `flat` may use a relative representation, this option extends the rendering to show the absolute values of the data.
    pub show_total: bool,
    /// Whether to abbreviate the column headings (which come from dimensional values) in the breakdown or not.
    /// Use this option when the breakdown dimensions have long `std::fmt::Display` forms.
    /// Abbreviation is attempted irrespective of the `width_hint`.
    pub abbreviate_breakdown: bool,
    /// The widget specific rendering configuration.
    pub widget_config: C,
}

impl<C: Default> Default for Render<C> {
    fn default() -> Self {
        Self {
            width_hint: 180,
            show_total: false,
            abbreviate_breakdown: false,
            widget_config: C::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Column {
    index: usize,
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
    pub fn string(index: usize, alignment: Alignment) -> Self {
        Self {
            index,
            alignment,
            column_type: ColumnType::String(0),
        }
    }

    pub fn count(index: usize, alignment: Alignment) -> Self {
        Self {
            index,
            alignment,
            column_type: ColumnType::Count,
        }
    }

    pub fn breakdown(index: usize, alignment: Alignment) -> Self {
        Self {
            index,
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
    fn write(&self, f: &mut Formatter<'_>, cell: &Cell, view: &View) -> std::fmt::Result {
        assert_eq!(self.index, cell.column);

        let width = match &self.column_type {
            ColumnType::String(width) => *width,
            ColumnType::Count | ColumnType::Breakdown => view.width,
        };

        match &self.alignment {
            Alignment::Left => {
                write!(f, "{:<width$}", cell.value.render(&view))
            }
            Alignment::Center => {
                write!(f, "{:^width$}", cell.value.render(&view))
            }
            Alignment::Right => {
                write!(f, "{:>width$}", cell.value.render(&view))
            }
        }
    }

    fn write_final(&self, f: &mut Formatter<'_>, cell: &Cell, view: &View) -> std::fmt::Result {
        assert_eq!(self.index, cell.column);
        write!(f, "{}", cell.value.render(view))
    }

    fn fill(&self, f: &mut Formatter<'_>, _view: &View) -> std::fmt::Result {
        let width = match &self.column_type {
            ColumnType::String(width) => *width,
            ColumnType::Count => 0,
            ColumnType::Breakdown => {
                unreachable!("should never fill a breakdown");
            }
        };

        write!(f, "{:width$}", "")
    }
}

#[derive(Debug)]
pub(crate) struct Cell {
    column: usize,
    value: Value,
}

#[derive(Debug)]
pub(crate) enum Value {
    Empty,
    String(String),
    Count(u64),
}

impl Value {
    fn render_width(&self) -> Option<usize> {
        match &self {
            Value::Empty => Some(0),
            Value::String(string) => Some(string.chars().count()),
            Value::Count(_) => None,
        }
    }

    fn render(&self, view: &View) -> String {
        match &self {
            Value::Empty => "".to_string(),
            Value::String(string) => match view.breakdown_abbreviations.get(string) {
                Some(abbr) => abbr.clone(),
                None => string.clone(),
            },
            Value::Count(count) => iter::repeat('*')
                .take(count.div_ceil(view.scale) as usize)
                .collect::<String>(),
        }
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

#[derive(Debug)]
pub(crate) struct Grid {
    columns: Vec<Column>,
    rows: Vec<HashMap<usize, Cell>>,
    breakdown_values: HashSet<String>,
    /// The minimum width of all the columns (before abbreviation).
    minimum_breakdown_width: usize,
    /// The maximum width of all the columns (before abbreviation).
    maximum_breakdown_width: usize,
}

impl Grid {
    pub fn new(mut columns: Vec<Column>) -> Self {
        columns.sort_by(|a, b| a.index.cmp(&b.index));
        Self {
            columns,
            rows: Vec::default(),
            breakdown_values: HashSet::default(),
            minimum_breakdown_width: usize::MAX,
            maximum_breakdown_width: 0,
        }
    }

    pub fn add(&mut self, row: Row) {
        assert!(!row.cells.is_empty());
        for (j, cell) in row.cells.iter().enumerate() {
            let column = self
                .columns
                .get_mut(j)
                .expect(format!("All columns must be accounted for, missing: {j}.").as_str());
            assert_eq!(column.index, j);

            match &mut column.column_type {
                ColumnType::String(width) => {
                    if let Some(cell_width) = cell.value.render_width() {
                        if &cell_width > width {
                            *width = cell_width;
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

        self.rows
            .push(row.cells.into_iter().map(|c| (c.column, c)).collect());
    }
}

/// The textual representation of data.
/// This is always produced by calling `.render(config)` on a widget.
/// Use [`std::fmt::Display`] to materialize the flat rendering.
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
#[derive(Debug)]
pub struct Flat {
    maximum_count: u64,
    width_hint: usize,
    abbreviate_breakdown: bool,
    grid: Grid,
}

impl Flat {
    pub(crate) fn new(
        maximum_count: u64,
        width_hint: usize,
        abbreviate_breakdown: bool,
        grid: Grid,
    ) -> Self {
        Self {
            maximum_count,
            width_hint,
            abbreviate_breakdown,
            grid,
        }
    }
}

impl Display for Flat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut view_width = self.width_hint.saturating_sub(
            self.grid
                .columns
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

        let scale: u64 = self.maximum_count.div_ceil(view_width as u64);
        let width: usize = if scale == 1 {
            self.maximum_count as usize
        } else {
            view_width
        };

        let (abbreviation_width, breakdown_abbreviations) = match (
            self.abbreviate_breakdown,
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

        let view = View {
            width: std::cmp::max(width, abbreviation_width),
            breakdown_abbreviations,
            scale,
        };

        for (i, row) in self.grid.rows.iter().enumerate() {
            let filled_row = filled(row);
            let filled_row_length = filled_row.len();

            for (j, optional_cell) in filled_row.into_iter() {
                match optional_cell {
                    Some(cell) => {
                        if j + 1 == filled_row_length {
                            self.grid.columns[j].write_final(f, cell, &view)?;
                        } else {
                            self.grid.columns[j].write(f, cell, &view)?;
                        }
                    }
                    None => {
                        self.grid.columns[j].fill(f, &view)?;
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
    width: usize,
    breakdown_abbreviations: HashMap<String, String>,
    scale: u64,
}

fn filled<'a>(rows: &'a HashMap<usize, Cell>) -> Vec<(usize, Option<&'a Cell>)> {
    let maximum_j: usize = *rows.keys().max().expect("Row must not be empty");
    let mut out = Vec::default();
    let mut candidates = Vec::default();

    for j in 0..=maximum_j {
        let cell = rows.get(&&j);

        if let Some(Cell {
            value: Value::Empty,
            ..
        }) = &cell
        {
            candidates.push((j, cell));
        } else if cell.is_some() {
            candidates.drain(..).for_each(|item| out.push(item));
            out.push((j, cell));
        } else {
            candidates.push((j, cell));
        }
    }

    out
}
