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
        if self.schema.breakdown_header().is_none() {
            self.render_total(config)
        } else {
            self.render_breakdown(config)
        }
    }

    fn render_total(self, config: Render) -> Flat {
        assert!(self.schema.breakdown_header().is_none());
        let get_locus = |chain: &Vec<String>| chain[0].clone();
        let get_path = |chain: &Vec<String>, dag_index: usize| chain[0..dag_index + 1].join("|");
        let get_full_path = |chain: Vec<String>| get_path(&chain, self.schema.width() - 1);
        let mut counts: HashMap<String, u64> = self
            .data
            .iter()
            .map(|(k, _)| (get_locus(&k.chain()), 0))
            .collect();
        let mut full_paths: HashSet<String> = HashSet::default();
        let mut sort_chains: Vec<&S::Dims> = Vec::default();
        let mut path_occurrences: HashMap<String, usize> = HashMap::default();
        let mut chain_length = 0;
        let mut maximum_count: u64 = 0;

        for (k, v) in self.data.iter() {
            let chain = k.chain();
            let locus = get_locus(&chain);
            let full_path = get_full_path(chain.clone());

            if !full_paths.contains(&full_path) {
                full_paths.insert(full_path);

                if chain.len() > chain_length {
                    chain_length = chain.len();
                }

                for dag_index in 0..chain.len() {
                    let path = get_path(&chain, dag_index);
                    path_occurrences
                        .entry(path)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }

            let count = counts.get_mut(&locus).expect(
                format!("locus '{locus}' must map to one of the aggregating rows").as_str(),
            );
            *count += v;

            if *count > maximum_count {
                maximum_count = *count;
            }

            if !sort_chains.contains(&k) {
                sort_chains.push(k);
            }
        }

        sort_chains.sort();
        let mut columns = Vec::default();

        for _ in 0..self.schema.headers().len() {
            columns.push(Column::string(columns.len(), Alignment::Left));
            columns.push(Column::string(columns.len(), Alignment::Left));
        }

        let mut row = Row::default();
        columns.push(Column::count(columns.len(), Alignment::Left));
        let mut grid = Grid::new(columns);

        for name in self.schema.headers().iter().rev() {
            row.push(Value::String(name.clone()));
            row.push(Value::Empty);
        }

        grid.add(row);
        let mut column_groups: HashMap<usize, Group> = HashMap::default();
        let mut current_locus = None;

        for k in sort_chains.iter() {
            let chain = k.chain();
            let locus = get_locus(&chain);

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
            /// dag_index | j | part | path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "b"   | "d|b"
            /// 2        | 0 | "a"   | "d|b|a"
            ///
            /// chain: ["d", "f", "c"]
            /// dag_index | j | part | path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d|f"
            /// 2        | 0 | "c"   | "d|f|c"
            ///
            /// chain: ["d", "f", "e"]
            /// dag_index | j | part | path
            /// ------------------------------
            /// 0        | 2 | "d"   | "d"
            /// 1        | 1 | "f"   | "d|f"
            /// 2        | 0 | "e"   | "d|f|e"
            ///
            /// etc..
            ///
            for (dag_index, part) in chain.clone().iter().enumerate() {
                let j = chain.len() - dag_index - 1;
                let path = get_path(&chain, dag_index);
                let group = column_groups.entry(j).or_default();

                if group.matches(&path) {
                    group.increment();
                } else {
                    group.swap(path.clone());
                }

                let occurrences = path_occurrences[&path];
                let position = (occurrences as f64 / 2.0).ceil() as usize - 1;
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
                            *counts
                                .get(&locus)
                                .expect(format!("locus '{locus}' must map to the value").as_str()),
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

    fn render_breakdown(self, config: Render) -> Flat {
        assert!(self.schema.breakdown_header().is_some());
        let get_locus = |chain: &Vec<String>| chain[0].clone();
        let get_count_key = |chain: Vec<String>| self.schema.aggregate_key(chain);
        let get_sub_path = |chain: Vec<String>, dag_index: usize| {
            self.schema
                .path(chain, dag_index)
                .iter()
                .fold(String::default(), |acc, (_, part)| acc + part + "|")
        };
        let get_full_chain = |chain: Vec<String>| self.schema.path(chain, self.schema.width() - 1);
        let get_full_path = |chain: Vec<String>| {
            get_full_chain(chain)
                .iter()
                .fold(String::default(), |acc, (_, part)| acc + part + "|")
        };
        let mut counts: HashMap<(String, Option<String>), u64> = HashMap::default();
        let mut full_paths: HashSet<String> = HashSet::default();
        let mut sort_chains: Vec<&S::Dims> = Vec::default();
        let mut sort_breakdown_keys: Vec<String> = Vec::default();
        let mut path_occurrences: HashMap<String, usize> = HashMap::default();
        let mut chain_length = 0;
        let mut maximum_count: u64 = 0;

        for (k, v) in self.data.iter() {
            let chain = k.chain();
            let count_key = get_count_key(chain.clone());
            let full_path = get_full_path(chain.clone());

            // Only count the occurrences once per 'full path'.
            // This is because we might have multiple entries, for example:
            // ```
            // Categorical::builder(schema)
            //     .add(("whale".to_string(), 4u32), 2)
            //     .add(("whale".to_string(), 4u32), 3)
            // ```
            if !full_paths.contains(&full_path) {
                full_paths.insert(full_path);

                if chain.len() > chain_length {
                    chain_length = chain.len();
                }

                for dag_index in 0..chain.len() {
                    let path = get_sub_path(chain.clone(), dag_index);
                    path_occurrences
                        .entry(path)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }

            let count = counts.entry(count_key.clone()).or_default();
            *count += v;

            if *count > maximum_count {
                maximum_count = *count;
            }

            if !sort_chains.contains(&k) {
                sort_chains.push(k);
            }

            let breakdown_key = count_key.1.unwrap();
            if !sort_breakdown_keys.contains(&breakdown_key) {
                sort_breakdown_keys.push(breakdown_key);
            }
        }

        sort_chains.sort();
        sort_breakdown_keys.sort();
        let mut columns = Vec::default();

        for _ in 0..self.schema.headers().len() {
            columns.push(Column::string(columns.len(), Alignment::Left));
            columns.push(Column::string(columns.len(), Alignment::Left));
        }

        columns.push(Column::string(columns.len(), Alignment::Center));

        for i in 0..sort_breakdown_keys.len() {
            columns.push(Column::breakdown(columns.len(), Alignment::Center));

            if i + 1 < sort_breakdown_keys.len() {
                columns.push(Column::string(columns.len(), Alignment::Left));
            }
        }

        columns.push(Column::string(columns.len(), Alignment::Center));
        println!("{columns:?}");
        let mut grid = Grid::new(columns);

        // TODO
        // let breakdown = self.schema.breakdown_header().unwrap();
        // for _ in self.schema.headers().iter() {
        //     row.push(Value::Empty);
        //     row.push(Value::Empty);
        // }
        //
        // row.push(Value::String(breakdown));
        // grid.add(row);
        let mut row = Row::default();

        for name in self.schema.headers().iter().rev() {
            row.push(Value::String(name.clone()));
            row.push(Value::Empty);
        }

        row.push(Value::String("|".to_string()));

        for (k, key) in sort_breakdown_keys.iter().enumerate() {
            row.push(Value::String(key.clone()));

            if k + 1 < sort_breakdown_keys.len() {
                row.push(Value::String(" ".to_string()));
            }
        }

        row.push(Value::String("|".to_string()));
        grid.add(row);
        let mut rendered: HashSet<String> = HashSet::default();
        let mut column_groups: HashMap<usize, Group> = HashMap::default();
        let mut current_locus = None;

        for k in sort_chains.iter() {
            let chain = k.chain();
            let locus = get_locus(&chain);

            if !rendered.contains(&locus) {
                rendered.insert(locus.clone());

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
                /// dag_index | j | part | path
                /// ------------------------------
                /// 0        | 2 | "d"   | "d"
                /// 1        | 1 | "b"   | "d|b"
                /// 2        | 0 | "a"   | "d|b|a"
                ///
                /// chain: ["d", "f", "c"]
                /// dag_index | j | part | path
                /// ------------------------------
                /// 0        | 2 | "d"   | "d"
                /// 1        | 1 | "f"   | "d|f"
                /// 2        | 0 | "c"   | "d|f|c"
                ///
                /// chain: ["d", "f", "e"]
                /// dag_index | j | part | path
                /// ------------------------------
                /// 0        | 2 | "d"   | "d"
                /// 1        | 1 | "f"   | "d|f"
                /// 2        | 0 | "e"   | "d|f|e"
                ///
                /// etc..
                ///
                for (dag_index, part) in get_full_chain(chain.clone()).iter() {
                    let j = chain.len() - dag_index - 1;
                    let path = get_sub_path(chain.clone(), *dag_index);
                    let group = column_groups.entry(j).or_default();

                    if group.matches(&path) {
                        group.increment();
                    } else {
                        group.swap(path.clone());
                    }

                    let occurrences = path_occurrences[&path];
                    let position = (occurrences as f64 / 2.0).ceil() as usize - 1;
                    let mut column_chunks = Vec::default();

                    let position = match position {
                        position if position > group.index => Position::Above,
                        position if position == group.index => Position::At,
                        _ => Position::Below,
                    };

                    if position == Position::At {
                        column_chunks.push(Value::String(format!("{part} ")));

                        if *dag_index == 0 {
                            column_chunks.push(Value::String(format!("  ")));
                            column_chunks.push(Value::String("|".to_string()));

                            for (k, breakdown_key) in sort_breakdown_keys.iter().enumerate() {
                                let count_key = (locus.clone(), Some(breakdown_key.clone()));

                                match counts.get(&count_key) {
                                    Some(count) => {
                                        column_chunks.push(Value::Count(*count));
                                    }
                                    None => {
                                        column_chunks.push(Value::Count(0));
                                    }
                                };

                                if k + 1 != sort_breakdown_keys.len() {
                                    column_chunks.push(Value::Empty);
                                }
                            }

                            column_chunks.push(Value::String("|".to_string()));
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

                println!("{column_chunks_reversed:?}");
                let mut row = Row::default();

                for column_chunks in column_chunks_reversed.into_iter().rev() {
                    for value in column_chunks.into_iter() {
                        println!("{value:?}");
                        row.push(value);
                    }

                    println!();
                }

                grid.add(row);
            }
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
