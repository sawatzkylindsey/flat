use crate::render::{Flat, Render, Row};
use crate::schema::{Dimensions, Schematic};
use std::collections::{HashMap, HashSet};

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
