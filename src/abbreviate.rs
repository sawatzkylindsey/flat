use std::collections::{HashMap, HashSet};

const ABBREVIATION_MONIKER: &str = "..";

pub(crate) fn find_abbreviations(
    minimum_length: usize,
    maximum_length: usize,
    values: &HashSet<String>,
) -> (usize, HashMap<String, String>) {
    assert!(minimum_length <= maximum_length);
    let shortest_length = values.iter().map(|v| v.chars().count()).min().unwrap();
    let longest_length = values.iter().map(|v| v.chars().count()).max().unwrap();
    let mut start = minimum_length;

    if shortest_length <= minimum_length && longest_length > minimum_length {
        start = std::cmp::max(minimum_length, shortest_length + ABBREVIATION_MONIKER.len());
    }

    for i in start..=maximum_length {
        if let Some(abbreviations) = generate_abbreviations(i, values) {
            return (i, abbreviations);
        }
    }

    // TODO: This fallback case could just be a `None` (with a signature change).
    let fallback_abbreviations = values.iter().map(|v| (v.clone(), v.clone())).collect();
    (longest_length, fallback_abbreviations)
}

fn generate_abbreviations(
    target_length: usize,
    values: &HashSet<String>,
) -> Option<HashMap<String, String>> {
    let left_abbreviations: HashMap<String, (String, bool)> = values
        .iter()
        .map(|v| {
            let value_length = v.chars().count();

            if value_length <= target_length {
                (v.clone(), (v.clone(), false))
            } else {
                (
                    v.clone(),
                    (
                        v.chars()
                            .take(target_length.saturating_sub(ABBREVIATION_MONIKER.len()))
                            .collect::<String>(),
                        true,
                    ),
                )
            }
        })
        .collect();

    let unique_abbreviations: HashSet<&String> =
        left_abbreviations.values().map(|(v, _)| v).collect();

    if unique_abbreviations.len() == left_abbreviations.len() {
        return Some(
            left_abbreviations
                .into_iter()
                .map(|(k, v)| {
                    if v.1 {
                        (k, v.0 + ABBREVIATION_MONIKER)
                    } else {
                        (k, v.0)
                    }
                })
                .collect(),
        );
    }

    let right_abbreviations: HashMap<String, (String, bool)> = values
        .iter()
        .map(|v| {
            let value_length = v.chars().count();

            if value_length <= target_length {
                (v.clone(), (v.clone(), false))
            } else {
                let reversed = v
                    .chars()
                    .rev()
                    .take(target_length.saturating_sub(ABBREVIATION_MONIKER.len()))
                    .collect::<String>();
                (
                    v.clone(),
                    (reversed.chars().rev().collect::<String>(), true),
                )
            }
        })
        .collect();

    let unique_abbreviations: HashSet<&String> =
        right_abbreviations.values().map(|(v, _)| v).collect();

    if unique_abbreviations.len() == right_abbreviations.len() {
        return Some(
            right_abbreviations
                .into_iter()
                .map(|(k, v)| {
                    if v.1 {
                        (k, ABBREVIATION_MONIKER.to_string() + &v.0)
                    } else {
                        (k, v.0)
                    }
                })
                .collect(),
        );
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_abbreviate() {
        let values = HashSet::from([
            "123dog".to_string(),
            "234dog".to_string(),
            "345dog".to_string(),
            "456dog".to_string(),
        ]);
        assert_eq!(
            generate_abbreviations(5, &values).unwrap(),
            HashMap::from([
                ("123dog".to_string(), "123..".to_string()),
                ("234dog".to_string(), "234..".to_string()),
                ("345dog".to_string(), "345..".to_string()),
                ("456dog".to_string(), "456..".to_string()),
            ])
        );

        let values = HashSet::from(["lick".to_string(), "metallick".to_string()]);
        assert_eq!(
            generate_abbreviations(6, &values).unwrap(),
            HashMap::from([
                ("lick".to_string(), "lick".to_string()),
                ("metallick".to_string(), "meta..".to_string()),
            ])
        );
    }

    #[test]
    fn right_abbreviate() {
        let values = HashSet::from([
            "dog123".to_string(),
            "dog234".to_string(),
            "dog345".to_string(),
            "dog456".to_string(),
        ]);
        assert_eq!(
            generate_abbreviations(5, &values).unwrap(),
            HashMap::from([
                ("dog123".to_string(), "..123".to_string()),
                ("dog234".to_string(), "..234".to_string()),
                ("dog345".to_string(), "..345".to_string()),
                ("dog456".to_string(), "..456".to_string()),
            ])
        );

        let values = HashSet::from(["cat".to_string(), "catostrophic".to_string()]);
        assert_eq!(
            generate_abbreviations(5, &values).unwrap(),
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("catostrophic".to_string(), "..hic".to_string()),
            ])
        );
    }

    #[test]
    fn not_abbreviate() {
        let values = HashSet::from([
            "dogfish".to_string(),
            "catfish".to_string(),
            "dogmouse".to_string(),
        ]);
        assert_eq!(generate_abbreviations(3, &values), None);
    }

    #[test]
    fn abbreviate() {
        let values = HashSet::from([
            "dog mercenary".to_string(),
            "implementation".to_string(),
            "wrinkled".to_string(),
            "green grass".to_string(),
        ]);

        assert_eq!(
            generate_abbreviations(4, &values).unwrap(),
            HashMap::from([
                ("dog mercenary".to_string(), "do..".to_string()),
                ("implementation".to_string(), "im..".to_string()),
                ("wrinkled".to_string(), "wr..".to_string()),
                ("green grass".to_string(), "gr..".to_string()),
            ])
        );
    }

    #[test]
    fn abbreviate_unicode() {
        let values = HashSet::from([
            "dog mercenary".to_string(),
            "implementation".to_string(),
            "wrinkled".to_string(),
            "green grass".to_string(),
        ]);
        assert_eq!(
            generate_abbreviations(4, &values).unwrap(),
            HashMap::from([
                ("dog mercenary".to_string(), "do..".to_string()),
                ("implementation".to_string(), "im..".to_string()),
                ("wrinkled".to_string(), "wr..".to_string()),
                ("green grass".to_string(), "gr..".to_string()),
            ])
        );

        let values = HashSet::from([
            "东西五东西五".to_string(),
            "i东西东西".to_string(),
            "东西东西d".to_string(),
            "东西一东西一".to_string(),
        ]);
        assert_eq!(
            generate_abbreviations(4, &values).unwrap(),
            HashMap::from([
                ("东西五东西五".to_string(), "..西五".to_string()),
                ("i东西东西".to_string(), "..东西".to_string()),
                ("东西东西d".to_string(), "..西d".to_string()),
                ("东西一东西一".to_string(), "..西一".to_string()),
            ])
        );
    }

    #[test]
    fn find() {
        let values = HashSet::from([
            "doggy123yggod".to_string(),
            "doggy234yggod".to_string(),
            "doggy345yggod".to_string(),
            "doggy456yggod".to_string(),
        ]);
        let (width, abbreviations) = find_abbreviations(5, 8, &values);
        assert_eq!(width, 8);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("doggy123yggod".to_string(), "doggy1..".to_string()),
                ("doggy234yggod".to_string(), "doggy2..".to_string()),
                ("doggy345yggod".to_string(), "doggy3..".to_string()),
                ("doggy456yggod".to_string(), "doggy4..".to_string()),
            ])
        );
    }

    #[test]
    fn find_fallback() {
        let values = HashSet::from([
            "doggy123yggod".to_string(),
            "doggy234yggod".to_string(),
            "doggy345yggod".to_string(),
            "doggy456yggod".to_string(),
        ]);
        let (width, abbreviations) = find_abbreviations(3, 4, &values);
        assert_eq!(width, 13);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("doggy123yggod".to_string(), "doggy123yggod".to_string()),
                ("doggy234yggod".to_string(), "doggy234yggod".to_string()),
                ("doggy345yggod".to_string(), "doggy345yggod".to_string()),
                ("doggy456yggod".to_string(), "doggy456yggod".to_string()),
            ])
        );
    }

    #[test]
    fn find_insufficient_improvement() {
        let values = HashSet::from([
            "ca1ac".to_string(),
            "ca2ac".to_string(),
            "ca3ac".to_string(),
            "ca4ac".to_string(),
        ]);
        let (width, abbreviations) = find_abbreviations(1, 5, &values);
        assert_eq!(width, 5);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("ca1ac".to_string(), "ca1ac".to_string()),
                ("ca2ac".to_string(), "ca2ac".to_string()),
                ("ca3ac".to_string(), "ca3ac".to_string()),
                ("ca4ac".to_string(), "ca4ac".to_string()),
            ])
        );
    }

    #[test]
    fn find_value_overlap() {
        let values = HashSet::from(["lick".to_string(), "metallick".to_string()]);
        let (width, abbreviations) = find_abbreviations(4, 9, &values);
        assert_eq!(width, 6);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("lick".to_string(), "lick".to_string()),
                ("metallick".to_string(), "meta..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(5, 9, &values);
        assert_eq!(width, 6);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("lick".to_string(), "lick".to_string()),
                ("metallick".to_string(), "meta..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(6, 9, &values);
        assert_eq!(width, 6);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("lick".to_string(), "lick".to_string()),
                ("metallick".to_string(), "meta..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(7, 9, &values);
        assert_eq!(width, 7);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("lick".to_string(), "lick".to_string()),
                ("metallick".to_string(), "metal..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(8, 9, &values);
        assert_eq!(width, 8);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("lick".to_string(), "lick".to_string()),
                ("metallick".to_string(), "metall..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(9, 9, &values);
        assert_eq!(width, 9);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("lick".to_string(), "lick".to_string()),
                ("metallick".to_string(), "metallick".to_string()),
            ])
        );

        let values = HashSet::from(["cat".to_string(), "cataclysm".to_string()]);
        let (width, abbreviations) = find_abbreviations(3, 9, &values);
        assert_eq!(width, 5);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("cataclysm".to_string(), "..ysm".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(4, 9, &values);
        assert_eq!(width, 5);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("cataclysm".to_string(), "..ysm".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(5, 9, &values);
        assert_eq!(width, 5);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("cataclysm".to_string(), "..ysm".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(6, 9, &values);
        assert_eq!(width, 6);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("cataclysm".to_string(), "cata..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(7, 9, &values);
        assert_eq!(width, 7);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("cataclysm".to_string(), "catac..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(8, 9, &values);
        assert_eq!(width, 8);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("cataclysm".to_string(), "catacl..".to_string()),
            ])
        );
        let (width, abbreviations) = find_abbreviations(9, 9, &values);
        assert_eq!(width, 9);
        assert_eq!(
            abbreviations,
            HashMap::from([
                ("cat".to_string(), "cat".to_string()),
                ("cataclysm".to_string(), "cataclysm".to_string()),
            ])
        );
    }
}
