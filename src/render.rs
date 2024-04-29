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
            render_width: 120,
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
enum ColumnType {
    String(usize),
    Count,
}

impl Column {
    pub(crate) fn string(index: usize, alignment: Alignment) -> Self {
        Self {
            index,
            alignment,
            column_type: ColumnType::String(0),
        }
    }

    pub(crate) fn count(index: usize, alignment: Alignment) -> Self {
        Self {
            index,
            alignment,
            column_type: ColumnType::Count,
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
    fn write(&self, f: &mut Formatter<'_>, cell: &Cell, scale: u64) -> std::fmt::Result {
        assert_eq!(self.index, cell.column);

        let width = match &self.column_type {
            ColumnType::String(width) => *width,
            ColumnType::Count => 0,
        };

        match &self.alignment {
            Alignment::Left => {
                write!(f, "{:<width$}", cell.value.render(scale))
            }
            Alignment::Center => {
                write!(f, "{:^width$}", cell.value.render(scale))
            }
            Alignment::Right => {
                write!(f, "{:>width$}", cell.value.render(scale))
            }
        }
    }

    fn write_final(&self, f: &mut Formatter<'_>, cell: &Cell, scale: u64) -> std::fmt::Result {
        assert_eq!(self.index, cell.column);
        write!(f, "{}", cell.value.render(scale))
    }

    fn fill(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = match &self.column_type {
            ColumnType::String(width) => *width,
            ColumnType::Count => 0,
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
    pub(crate) fn push(&mut self, value: Value) {
        self.cells.push(Cell {
            column: self.cells.len(),
            value,
        })
    }
}

#[derive(Debug)]
pub(crate) struct Grid {
    columns: Vec<Column>,
    rows: Vec<HashMap<usize, Cell>>,
}

impl Grid {
    pub(crate) fn new(mut columns: Vec<Column>) -> Self {
        columns.sort_by(|a, b| a.index.cmp(&b.index));
        Self {
            columns,
            rows: Vec::default(),
        }
    }

    pub(crate) fn add(&mut self, row: Row) {
        for (j, cell) in row.cells.iter().enumerate() {
            let column = self
                .columns
                .get_mut(j)
                .expect(format!("All columns must be accounted for, missing: {j}.").as_str());
            assert_eq!(column.index, j);

            if let ColumnType::String(width) = &mut column.column_type {
                if let Some(cell_width) = cell.value.render_width() {
                    if &cell_width > width {
                        *width = cell_width;
                    }
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
        let mut count_width = self.render_width.saturating_sub(
            self.grid
                .columns
                .iter()
                .filter_map(|c| match &c.column_type {
                    ColumnType::String(width) => Some(width),
                    ColumnType::Count => None,
                })
                .sum(),
        );

        if count_width < 2 {
            count_width = 2;
        }

        let scale: u64 = self.maximum_count.div_ceil(count_width as u64);

        for row in self.grid.rows.iter() {
            for (j, optional_cell) in filled(row) {
                match optional_cell {
                    Some(cell) => {
                        if j + 1 == row.len() {
                            self.grid.columns[j].write_final(f, cell, scale)?;
                        } else {
                            self.grid.columns[j].write(f, cell, scale)?;
                        }
                    }
                    None => {
                        self.grid.columns[j].fill(f)?;
                    }
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn filled<'a>(rows: &'a HashMap<usize, Cell>) -> Vec<(usize, Option<&'a Cell>)> {
    let maximum_j: usize = *rows.keys().max().expect("Row must not be empty");
    (0..=maximum_j).map(|j| (j, rows.get(&j))).collect()
}
