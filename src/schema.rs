use std::fmt::Display;
use std::marker::PhantomData;

pub trait Dimensions: Ord {
    fn chain(&self) -> Vec<String>;
}

pub trait Schematic {
    type Dims: Dimensions;

    fn width(&self) -> usize;

    fn headers(&self) -> Vec<String>;

    fn breakdown_header(&self) -> Option<String>;

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)>;

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>);
}

pub struct Schema;

pub struct Schema1<T> {
    one: PhantomData<T>,
    column_1: String,
}

pub struct Schema2<T, U> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    breakdown: Option<(Breakdown2, String)>,
}

pub struct Schema3<T, U, V> {
    one: PhantomData<T>,
    column_1: String,
    two: PhantomData<U>,
    column_2: String,
    three: PhantomData<V>,
    column_3: String,
    breakdown: Option<(Breakdown3, String)>,
}

// pub struct Schema4<T, U, V, W>;
// pub struct Schema5<T, U, V, W, X>;
// pub struct Schema6<T, U, V, W, X, Y>;
// pub struct Schema7<T, U, V, W, X, Y, Z>;

impl Schema {
    pub fn one<T>(column_1: impl Into<String>) -> Schema1<T> {
        Schema1 {
            one: PhantomData,
            column_1: column_1.into(),
        }
    }

    pub fn two<T, U>(column_1: impl Into<String>, column_2: impl Into<String>) -> Schema2<T, U> {
        Schema2 {
            one: PhantomData,
            column_1: column_1.into(),
            two: PhantomData,
            column_2: column_2.into(),
            breakdown: None,
        }
    }

    pub fn three<T, U, V>(
        column_1: impl Into<String>,
        column_2: impl Into<String>,
        column_3: impl Into<String>,
    ) -> Schema3<T, U, V> {
        Schema3 {
            one: PhantomData,
            column_1: column_1.into(),
            two: PhantomData,
            column_2: column_2.into(),
            three: PhantomData,
            column_3: column_3.into(),
            breakdown: None,
        }
    }
}

impl<T> Schematic for Schema1<T>
where
    T: Display + Ord,
{
    type Dims = (T,);

    fn width(&self) -> usize {
        1
    }

    fn headers(&self) -> Vec<String> {
        vec![self.column_1.clone()]
    }

    fn breakdown_header(&self) -> Option<String> {
        None
    }

    fn path(&self, chain: Vec<String>, _dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 1);
        vec![(0, chain[0].clone())]
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 1);
        (chain[0].clone(), None)
    }
}

pub enum Breakdown2 {
    Second,
}

impl Breakdown2 {
    fn as_dag_index(&self) -> usize {
        match &self {
            Breakdown2::Second => 1,
        }
    }
}

impl<T, U> Schematic for Schema2<T, U>
where
    T: Display + Ord,
    U: Display + Ord,
{
    type Dims = (T, U);

    fn width(&self) -> usize {
        2
    }

    fn headers(&self) -> Vec<String> {
        if let Some((breakdown, _)) = &self.breakdown {
            match breakdown {
                Breakdown2::Second => {
                    vec![self.column_1.clone()]
                }
            }
        } else {
            vec![self.column_1.clone(), self.column_2.clone()]
        }
    }

    fn breakdown_header(&self) -> Option<String> {
        self.breakdown.as_ref().map(|(_, h)| h.clone())
    }

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 2);
        let mut out = Vec::default();

        for i in 0..dag_index + 1 {
            if let Some((breakdown, _)) = &self.breakdown {
                // We want to skip the breakdown column
                if breakdown.as_dag_index() != i {
                    out.push((i, chain[i].clone()));
                }
            } else {
                out.push((i, chain[i].clone()));
            }
        }

        out
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 2);
        match &self.breakdown {
            Some((Breakdown2::Second, _)) => (chain[0].clone(), Some(chain[1].clone())),
            None => (chain[0].clone(), None),
        }
    }
}

impl<T, U> Schema2<T, U> {
    pub fn breakdown(mut self, breakdown: Breakdown2) -> Self {
        self.breakdown = match breakdown {
            Breakdown2::Second => Some((Breakdown2::Second, self.column_2.clone())),
        };
        self
    }
}

pub enum Breakdown3 {
    Second,
    Third,
}

impl Breakdown3 {
    fn as_dag_index(&self) -> usize {
        match &self {
            Breakdown3::Second => 1,
            Breakdown3::Third => 2,
        }
    }
}

impl<T, U, V> Schematic for Schema3<T, U, V>
where
    T: Display + Ord,
    U: Display + Ord,
    V: Display + Ord,
{
    type Dims = (T, U, V);

    fn width(&self) -> usize {
        3
    }

    fn headers(&self) -> Vec<String> {
        if let Some((breakdown, _)) = &self.breakdown {
            match breakdown {
                Breakdown3::Second => {
                    vec![self.column_1.clone(), self.column_3.clone()]
                }
                Breakdown3::Third => {
                    vec![self.column_1.clone(), self.column_2.clone()]
                }
            }
        } else {
            vec![
                self.column_1.clone(),
                self.column_2.clone(),
                self.column_3.clone(),
            ]
        }
    }

    fn breakdown_header(&self) -> Option<String> {
        self.breakdown.as_ref().map(|(_, h)| h.clone())
    }

    fn path(&self, chain: Vec<String>, dag_index: usize) -> Vec<(usize, String)> {
        assert_eq!(chain.len(), 3);
        let mut out = Vec::default();

        for i in 0..dag_index + 1 {
            if let Some((breakdown, _)) = &self.breakdown {
                // We want to skip the breakdown column
                if breakdown.as_dag_index() != i {
                    out.push((i, chain[i].clone()));
                }
            } else {
                out.push((i, chain[i].clone()));
            }
        }

        out
    }

    fn aggregate_key(&self, chain: Vec<String>) -> (String, Option<String>) {
        assert_eq!(chain.len(), 3);
        match &self.breakdown {
            Some((Breakdown3::Second, _)) => (chain[0].clone(), Some(chain[1].clone())),
            Some((Breakdown3::Third, _)) => (chain[0].clone(), Some(chain[2].clone())),
            None => (chain[0].clone(), None),
        }
    }
}

impl<T, U, V> Schema3<T, U, V> {
    pub fn breakdown(mut self, breakdown: Breakdown3) -> Self {
        self.breakdown = match breakdown {
            Breakdown3::Second => Some((Breakdown3::Second, self.column_2.clone())),
            Breakdown3::Third => Some((Breakdown3::Third, self.column_3.clone())),
        };
        self
    }
}

impl<T> Dimensions for (T,)
where
    T: Display + Ord,
{
    // fn locus(&self) -> String {
    //     self.0.to_string()
    // }

    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl<T, U> Dimensions for (T, U)
where
    T: Display + Ord,
    U: Display + Ord,
{
    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string()]
    }
}

impl<T, U, V> Dimensions for (T, U, V)
where
    T: Display + Ord,
    U: Display + Ord,
    V: Display + Ord,
{
    fn chain(&self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string(), self.2.to_string()]
    }
}
