use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

/// The types of value aggregation supported by `flat`.
#[derive(Debug)]
pub enum Aggregate {
    /// The average aggregation variant (ex: `[1, 2, 3] -> 2`).
    Average,
    /// The max aggregation variant (ex: `[1, 2, 3] -> 3`).
    Max,
    /// The min aggregation variant (ex: `[1, 2, 3] -> 1`).
    Min,
    /// The sum aggregation variant (ex: `[1, 2, 3] -> 6`).
    Sum,
}

impl Aggregate {
    pub(crate) fn apply(&self, values: &[f64]) -> f64 {
        match &self {
            Aggregate::Average => {
                if values.is_empty() {
                    0.0
                } else {
                    values.iter().sum::<f64>() / values.len() as f64
                }
            }
            Aggregate::Max => match values.iter().map(|v| OrderedFloat(*v)).max() {
                Some(m) => m.0,
                None => 0.0,
            },
            Aggregate::Min => match values.iter().map(|v| OrderedFloat(*v)).min() {
                Some(m) => m.0,
                None => 0.0,
            },
            Aggregate::Sum => values.iter().sum(),
        }
    }
}

impl Display for Aggregate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Aggregate::Average => write!(f, "Average"),
            Aggregate::Max => write!(f, "Max"),
            Aggregate::Min => write!(f, "Min"),
            Aggregate::Sum => write!(f, "Sum"),
        }
    }
}

pub(crate) fn aggregate_apply<T: Eq + Hash>(
    aggregate: &Aggregate,
    aggregate_values: &HashMap<T, Vec<f64>>,
    aggregate_dims: &T,
    minimum_value: &mut f64,
    maximum_value: &mut f64,
) -> f64 {
    let value = match aggregate_values.get(aggregate_dims) {
        Some(values) => aggregate.apply(values.as_slice()),
        None => aggregate.apply(&[]),
    };

    if value < *minimum_value {
        *minimum_value = value;
    }

    if value > *maximum_value {
        *maximum_value = value;
    }

    value
}

/// Generate a "minimal precision" string for the f64 value.
///
/// The intuition behind this format is to use as few decimal values as possible, without loss of visual accuracy.
/// We want to be able to distinguish distinct values.
/// Example:
///     1.111 vs 1.2 -> 1.1 vs 1.2
///     1.011 vs 1.2 -> 1.01 vs 1.2
///
/// See unit tests for more examples.
pub fn minimal_precision_string(value: f64) -> String {
    let value_string = value.to_string();

    let decimal_truncated = match value_string.split_once('.') {
        Some((left, right)) => {
            let mut leading_zeros = 0;

            for c in right.chars() {
                if c == '0' {
                    leading_zeros += 1;
                } else {
                    break;
                }
            }

            let decimal = f64::from_str(right).expect("decimal must parse as f64");
            let rounded = (decimal / 10f64.powi(decimal.log10().floor() as i32)).round();
            format!("{left}.{:0<leading_zeros$}{rounded}", "")
        }
        None => value_string,
    };

    match f64::from_str(&decimal_truncated) {
        Ok(value) => {
            if value >= 1e9 {
                format!("{value:.1e}")
            } else {
                decimal_truncated
            }
        }
        Err(_) => decimal_truncated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply() {
        assert_eq!(Aggregate::Average.apply(&[1.0, 2.0, 3.0]), 2.0);
        assert_eq!(Aggregate::Max.apply(&[1.0, 2.0, 3.0]), 3.0);
        assert_eq!(Aggregate::Min.apply(&[1.0, 2.0, 3.0]), 1.0);
        assert_eq!(Aggregate::Sum.apply(&[1.0, 2.0, 3.0]), 6.0);

        assert_eq!(Aggregate::Average.to_string(), "Average".to_string());
        assert_eq!(Aggregate::Max.to_string(), "Max".to_string());
        assert_eq!(Aggregate::Min.to_string(), "Min".to_string());
        assert_eq!(Aggregate::Sum.to_string(), "Sum".to_string());
    }

    #[test]
    fn aggregate_apply_average() {
        let values: HashMap<String, Vec<f64>> =
            HashMap::from([("abc".to_string(), vec![1.0, 2.0, 3.0])]);

        let mut min_watcher = f64::MAX;
        let mut max_watcher = f64::MIN;
        let result = aggregate_apply(
            &Aggregate::Average,
            &values,
            &"abc".to_string(),
            &mut min_watcher,
            &mut max_watcher,
        );
        assert_eq!(result, 2.0);
        assert_eq!(min_watcher, 2.0);
        assert_eq!(max_watcher, 2.0);

        let mut min_watcher = f64::MAX;
        let mut max_watcher = f64::MIN;
        let result = aggregate_apply(
            &Aggregate::Average,
            &values,
            &"def".to_string(),
            &mut min_watcher,
            &mut max_watcher,
        );
        assert_eq!(result, 0.0);
        assert_eq!(min_watcher, 0.0);
        assert_eq!(max_watcher, 0.0);
    }

    #[test]
    fn aggregate_apply_sum() {
        let values: HashMap<String, Vec<f64>> =
            HashMap::from([("abc".to_string(), vec![1.0, 2.0, 3.0])]);

        let mut min_watcher = f64::MAX;
        let mut max_watcher = f64::MIN;
        let result = aggregate_apply(
            &Aggregate::Sum,
            &values,
            &"abc".to_string(),
            &mut min_watcher,
            &mut max_watcher,
        );
        assert_eq!(result, 6.0);
        assert_eq!(min_watcher, 6.0);
        assert_eq!(max_watcher, 6.0);

        let mut min_watcher = f64::MAX;
        let mut max_watcher = f64::MIN;
        let result = aggregate_apply(
            &Aggregate::Sum,
            &values,
            &"def".to_string(),
            &mut min_watcher,
            &mut max_watcher,
        );
        assert_eq!(result, 0.0);
        assert_eq!(min_watcher, 0.0);
        assert_eq!(max_watcher, 0.0);
    }

    #[test]
    fn f64_string() {
        assert_eq!(minimal_precision_string(0.0), "0");
        assert_eq!(minimal_precision_string(1.0), "1");
        assert_eq!(minimal_precision_string(-1.0), "-1");

        assert_eq!(minimal_precision_string(0.1), "0.1");
        assert_eq!(minimal_precision_string(1.1), "1.1");
        assert_eq!(minimal_precision_string(-1.1), "-1.1");

        assert_eq!(minimal_precision_string(0.1234), "0.1");
        assert_eq!(minimal_precision_string(1.1234), "1.1");
        assert_eq!(minimal_precision_string(-1.1234), "-1.1");

        assert_eq!(minimal_precision_string(0.19), "0.2");
        assert_eq!(minimal_precision_string(1.19), "1.2");
        assert_eq!(minimal_precision_string(-1.19), "-1.2");

        assert_eq!(minimal_precision_string(0.001), "0.001");
        assert_eq!(minimal_precision_string(1.001), "1.001");
        assert_eq!(minimal_precision_string(-1.001), "-1.001");

        assert_eq!(minimal_precision_string(0.001234), "0.001");
        assert_eq!(minimal_precision_string(1.001234), "1.001");
        assert_eq!(minimal_precision_string(-1.001234), "-1.001");

        assert_eq!(minimal_precision_string(0.0019), "0.002");
        assert_eq!(minimal_precision_string(1.0019), "1.002");
        assert_eq!(minimal_precision_string(-1.0019), "-1.002");

        assert_eq!(minimal_precision_string(1.0 / 2.0), "0.5");
        assert_eq!(minimal_precision_string(1.0 / 3.0), "0.3");
        assert_eq!(minimal_precision_string(1.0 / 4.0), "0.3");
        assert_eq!(minimal_precision_string(1.0 / 5.0), "0.2");
        assert_eq!(minimal_precision_string(1.0 / 6.0), "0.2");

        assert_eq!(minimal_precision_string(2.0 / 2.0), "1");
        assert_eq!(minimal_precision_string(2.0 / 3.0), "0.7");
        assert_eq!(minimal_precision_string(2.0 / 4.0), "0.5");
        assert_eq!(minimal_precision_string(2.0 / 5.0), "0.4");
        assert_eq!(minimal_precision_string(2.0 / 6.0), "0.3");

        assert_eq!(minimal_precision_string(1.0 / 0.0), "inf");
        assert_eq!(minimal_precision_string(-1.0 / 0.0), "-inf");

        assert_eq!(minimal_precision_string(123456789.0), "123456789");
        assert_eq!(minimal_precision_string(1234567891.0), "1.2e9");
        assert_eq!(minimal_precision_string(12345678912.0), "1.2e10");
        assert_eq!(minimal_precision_string(123456789123.0), "1.2e11");
    }
}
