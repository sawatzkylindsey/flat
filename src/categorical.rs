use crate::render::{Alignment, Column, Flat, Grid, Render, Row, Value};
use crate::schema::{Dimensions, Schematic};
use std::collections::{HashMap, HashSet};

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

    pub fn render(self, config: Render) -> Flat {
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
        let mut current_locus = None;

        for k in sort_keys.iter() {
            let chain = k.chain();
            let locus = chain[0].clone();

            match &mut current_locus {
                Some(l) => {
                    if l != &locus {
                        current_locus.replace(locus.clone());
                    }
                }
                None => {
                    current_locus = Some(locus.clone());
                }
            }

            let mut column_chunks_reversed: Vec<Vec<Value>> = Vec::default();
            let mut descendant_position = None;

            #[allow(unused_doc_comments)]
            /// Run through the chain in dag index ascending order, which
            /// is the "rendering" reverse order.
            ///
            /// For this example dag, we'll iterate as follows:
            /// a - b ┐
            /// c ┐   - d
            /// e - f ┘
            /// h ┘
            ///
            /// chain: ["d", "b", "a"]
            /// dag_index | j | part | partial
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "b"   | "d|b"
            /// 2        | 0 | "a"   | "d|b|a"
            ///
            /// chain: ["d", "f", "c"]
            /// dag_index | j | part | partial
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d|f"
            /// 2        | 0 | "c"   | "d|f|c"
            ///
            /// chain: ["d", "f", "e"]
            /// dag_index | j | part | partial
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d|f"
            /// 2        | 0 | "e"   | "d|f|e"
            ///
            /// etc..
            ///
            for (dag_index, part) in chain.iter().enumerate() {
                let j = chain.len() - dag_index - 1;
                let partial = ancestor_key(chain.as_slice(), dag_index);
                let group = column_groups.entry(j).or_default();

                if group.matches(&partial) {
                    group.increment();
                } else {
                    group.swap(partial.clone());
                }

                let count = ancestors[&partial];
                let position = (count as f64 / 2.0).ceil() as usize - 1;
                let mut column_chunks = Vec::default();

                let position = match position {
                    position if position > group.index => Position::Above,
                    position if position == group.index => Position::At,
                    _ => Position::Below,
                };

                if position == Position::At {
                    column_chunks.push(Value::String(format!("{part} ")));

                    if dag_index == 0 {
                        column_chunks.push(Value::String(format!("  ")));
                        column_chunks.push(Value::Count(
                            *values
                                .get(&locus)
                                .expect(format!("Locus {locus} must map to the value").as_str()),
                        ));
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

        Flat::new(maximum_count, config.render_width, grid)
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
