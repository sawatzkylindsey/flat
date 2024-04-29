use crate::render::{Alignment, Column, Flat, Grid, Render, Row, Value};
use crate::schema::{Dimensions, Schematic};
use std::collections::{HashMap, HashSet};

pub const DEFAULT_SEPARATORS: &[&str; 3] = &[">", "-", "~"];

pub struct Categorical<S>
where
    S: Schematic,
    <S as Schematic>::Dims: Dimensions,
{
    schema: S,
    data: Vec<(S::Dims, u64)>,
}

impl<S: Schematic> Categorical<S> {
    pub fn builder(schema: S) -> Categorical<S> {
        Self {
            schema,
            data: Vec::default(),
        }
    }

    pub fn add(mut self, key: S::Dims, value: u64) -> Categorical<S> {
        self.data.push((key, value));
        self
    }

    pub fn render(self, config: Render, separators: &[&str]) -> Flat {
        let mut values: HashMap<String, u64> = self
            .data
            .iter()
            .map(|(k, _)| (k.chain()[0].clone(), 0))
            .collect();
        let mut full_chains: HashSet<String> = HashSet::default();
        let mut sort_keys: Vec<&S::Dims> = Vec::default();
        let ancestor_key = |chain: &[String], dag_index: usize| chain[0..dag_index + 1].join("|");
        let mut ancestors: HashMap<String, usize> = HashMap::default();
        let mut chain_length = 0;
        let mut maximum_count: u64 = 0;

        for (k, v) in self.data.iter() {
            let chain = k.chain();
            let locus = chain[0].clone();
            let full_chain = chain.join("|");

            if !full_chains.contains(&full_chain) {
                full_chains.insert(full_chain);

                if chain.len() > chain_length {
                    chain_length = chain.len();
                }

                for dag_index in 0..chain.len() {
                    let partial = ancestor_key(chain.as_slice(), dag_index);
                    ancestors
                        .entry(partial)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }

            let aggregate = values
                .get_mut(&locus)
                .expect(format!("Key/locus must map to one of the aggregating rows").as_str());
            *aggregate += v;

            if *aggregate > maximum_count {
                maximum_count = *aggregate;
            }

            if !sort_keys.contains(&k) {
                sort_keys.push(k);
            }
        }

        sort_keys.sort();
        let mut columns = Vec::default();

        for _ in 0..chain_length {
            columns.push(Column::string(columns.len(), Alignment::Left));
            columns.push(Column::string(columns.len(), Alignment::Left));
        }

        columns.push(Column::count(columns.len(), Alignment::Left));
        let mut grid = Grid::new(columns);
        let mut row = Row::default();

        for name in self.schema.headers().iter().rev() {
            row.push(Value::String(name.clone()));
            row.push(Value::Empty);
        }

        grid.add(row);
        let mut column_groups: HashMap<usize, Group> = HashMap::default();
        let mut separator_index = 0;
        let mut current_locus = None;

        for k in sort_keys.iter() {
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
            let mut row = Row::default();

            for (j, part) in chain.iter().rev().enumerate() {
                let dag_index = chain.len() - j - 1;
                let partial = ancestor_key(chain.as_slice(), dag_index);
                let group = column_groups.entry(j).or_default();

                if group.matches(&partial) {
                    group.increment();
                } else {
                    group.swap(partial.clone());
                }

                let count = ancestors[&partial];

                if group.index == (count as f64 / 2.0).ceil() as usize - 1 {
                    row.push(Value::String(format!("{part} ")));

                    if dag_index == 0 {
                        row.push(Value::String(format!("  ")));
                        row.push(Value::Count(
                            *values
                                .get(&locus)
                                .expect(format!("Locus {locus} must map to the value").as_str()),
                        ));
                    } else {
                        row.push(Value::String(format!(" {separator} ")));
                    }
                }
            }

            grid.add(row);
        }

        Flat::new(maximum_count, config.render_width, grid)
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
