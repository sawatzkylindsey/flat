use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::iter;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct Render {
    pub render_width: usize,
    pub show_total: bool,
}

impl Default for Render {
    fn default() -> Self {
        Self {
            render_width: 180,
            show_total: false,
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
            ColumnType::Count => view.width,
            ColumnType::Breakdown => {
                if view.breakdown_width > view.width {
                    view.breakdown_width
                } else {
                    view.width
                }
            }
        };

        match &self.alignment {
            Alignment::Left => {
                write!(f, "{:<width$}", cell.value.render(view.scale))
            }
            Alignment::Center => {
                write!(f, "{:^width$}", cell.value.render(view.scale))
            }
            Alignment::Right => {
                write!(f, "{:>width$}", cell.value.render(view.scale))
            }
        }
    }

    fn write_final(&self, f: &mut Formatter<'_>, cell: &Cell, view: &View) -> std::fmt::Result {
        assert_eq!(self.index, cell.column);
        write!(f, "{}", cell.value.render(view.scale))
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
            Value::String(string) => Some(UnicodeWidthStr::width(string.as_str())),
            Value::Count(_) => None,
        }
    }

    fn render(&self, scale: u64) -> String {
        match &self {
            Value::Empty => "".to_string(),
            Value::String(string) => string.clone(),
            Value::Count(count) => iter::repeat('*')
                .take(count.div_ceil(scale) as usize)
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
    maximum_breakdown_width: usize,
}

impl Grid {
    pub fn new(mut columns: Vec<Column>) -> Self {
        columns.sort_by(|a, b| a.index.cmp(&b.index));
        Self {
            columns,
            rows: Vec::default(),
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
                    if let Some(cell_width) = cell.value.render_width() {
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

#[derive(Debug)]
pub struct Flat {
    maximum_count: u64,
    render_width: usize,
    grid: Grid,
}

impl Flat {
    pub(crate) fn new(maximum_count: u64, render_width: usize, grid: Grid) -> Self {
        Self {
            maximum_count,
            render_width,
            grid,
        }
    }
}

impl Display for Flat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut view_width = self.render_width.saturating_sub(
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
        let view = View {
            width,
            breakdown_width: self.grid.maximum_breakdown_width,
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
    breakdown_width: usize,
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
