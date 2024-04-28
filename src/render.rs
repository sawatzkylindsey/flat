use std::fmt::{Display, Formatter};
use std::iter;

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
    pub(crate) rows: Vec<Row>,
    pub(crate) maximum_value: usize,
    pub(crate) config: Render,
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
