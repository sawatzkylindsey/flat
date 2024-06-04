use flat::*;
use rstest::rstest;

fn dataset_1d() -> Dataset<Schema1<String>> {
    let schema = Schemas::one("anml");
    Dataset::builder(schema)
        .add(("whale".to_string(),))
        .add(("shark".to_string(),))
        .add(("shark".to_string(),))
        .add(("tiger".to_string(),))
        .add(("tiger".to_string(),))
        .add(("tiger".to_string(),))
}

#[test]
fn barchart_1d() {
    let dataset = dataset_1d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
anml   |Sum(Count)
shark  |**
tiger  |***
whale  |*"#
    );
}

fn dataset_2d() -> Dataset<Schema2<String, u32>> {
    let schema = Schemas::two("animal", "length");
    Dataset::builder(schema)
        .add(("whale".to_string(), 4u32))
        .add(("shark".to_string(), 4u32))
        .add(("shark".to_string(), 1u32))
        .add(("shark".to_string(), 1u32))
        .add(("shark".to_string(), 1u32))
        .add(("tiger".to_string(), 4u32))
        .add(("tiger".to_string(), 5u32))
        .add(("tiger".to_string(), 5u32))
        .add(("tiger".to_string(), 5u32))
        .add(("tiger".to_string(), 1u32))
        .add(("tiger".to_string(), 1u32))
        .add(("tiger".to_string(), 1u32))
}

#[test]
fn barchart_2d() {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal  |Sum(Count)
1       - shark   |****
4       ┘
1       ┐
4       - tiger   |*******
5       ┘
4       - whale   |*"#
    );
}

#[rstest]
#[case(17)]
#[case(18)]
#[case(19)]
#[case(20)]
#[case(21)]
// #[case(22)]
fn barchart_2d_squish(#[case] width_hint: usize) {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        width_hint,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal  |Sum(Count)
1       - shark   |*
4       ┘
1       ┐
4       - tiger   |**
5       ┘
4       - whale   |"#
    );
}

#[test]
fn barchart_2d_show_sum() {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal Sum  |Sum(Count)
1       - shark  [4]  |****
4       ┘
1       ┐
4       - tiger  [7]  |*******
5       ┘
4       - whale  [1]  |*"#
    );
}

#[test]
fn barchart_2d_show_sum_widget() {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length Sum   animal  |Sum(Count)
1      [3] - shark   |****
4      [1] ┘
1      [3] ┐
4      [1] - tiger   |*******
5      [3] ┘
4      [1] - whale   |*"#
    );
}

#[test]
fn barchart_2d_show_sum_both() {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length Sum   animal Sum  |Sum(Count)
1      [3] - shark  [4]  |****
4      [1] ┘
1      [3] ┐
4      [1] - tiger  [7]  |*******
5      [3] ┘
4      [1] - whale  [1]  |*"#
    );
}

#[test]
fn barchart_2d_show_average() {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal Average  |Average(Count)
1       - shark  [1]      |*
4       ┘
1       ┐
4       - tiger  [1]      |*
5       ┘
4       - whale  [1]      |*"#
    );
}

#[test]
fn barchart_2d_show_average_widget() {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length Average   animal  |Average(Count)
1      [1]     - shark   |*
4      [1]     ┘
1      [1]     ┐
4      [1]     - tiger   |*
5      [1]     ┘
4      [1]     - whale   |*"#
    );
}

#[test]
fn barchart_2d_show_average_both() {
    let dataset = dataset_2d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length Average   animal Average  |Average(Count)
1      [1]     - shark  [1]      |*
4      [1]     ┘
1      [1]     ┐
4      [1]     - tiger  [1]      |*
5      [1]     ┘
4      [1]     - whale  [1]      |*"#
    );
}

#[test]
fn barchart_2d_breakdown() {
    let dataset = dataset_2d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
         Sum(Breakdown(length))
animal  | 1   4   5 |
shark   |***  *     |
tiger   |***  *  ***|
whale   |     *     |"#
    );
}

fn dataset_3d() -> Dataset<Schema3<String, u32, bool>> {
    let schema = Schemas::three("animal", "length", "stable");
    Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true))
        .add(("shark".to_string(), 4u32, false))
        .add(("shark".to_string(), 1u32, false))
        .add(("shark".to_string(), 1u32, true))
        .add(("shark".to_string(), 1u32, true))
        .add(("shark".to_string(), 1u32, true))
        .add(("tiger".to_string(), 4u32, false))
        .add(("tiger".to_string(), 4u32, false))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 1u32, false))
        .add(("tiger".to_string(), 1u32, false))
        .add(("tiger".to_string(), 1u32, false))
}

#[test]
fn barchart_3d() {
    let dataset = dataset_3d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable    length    animal  |Sum(Count)
false   - 1       ┐
true    ┘         - shark   |*****
false   - 4       ┘
false   - 1       ┐
false   - 4       - tiger   |***********
true    - 5       ┘
true    - 4       - whale   |*"#
    );
}

#[test]
fn barchart_3d_show_sum() {
    let dataset = dataset_3d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable    length    animal Sum   |Sum(Count)
false   - 1       ┐
true    ┘         - shark  [ 5]  |*****
false   - 4       ┘
false   - 1       ┐
false   - 4       - tiger  [11]  |***********
true    - 5       ┘
true    - 4       - whale  [ 1]  |*"#
    );
}

#[test]
fn barchart_3d_show_sum_widget() {
    let dataset = dataset_3d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable Sum   length Sum   animal  |Sum(Count)
false  [1] - 1      [4] ┐
true   [3] ┘            - shark   |*****
false  [1] - 4      [1] ┘
false  [3] - 1      [3] ┐
false  [2] - 4      [2] - tiger   |***********
true   [6] - 5      [6] ┘
true   [1] - 4      [1] - whale   |*"#
    );
}

#[test]
fn barchart_3d_show_sum_both() {
    let dataset = dataset_3d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable Sum   length Sum   animal Sum   |Sum(Count)
false  [1] - 1      [4] ┐
true   [3] ┘            - shark  [ 5]  |*****
false  [1] - 4      [1] ┘
false  [3] - 1      [3] ┐
false  [2] - 4      [2] - tiger  [11]  |***********
true   [6] - 5      [6] ┘
true   [1] - 4      [1] - whale  [ 1]  |*"#
    );
}

#[test]
fn barchart_3d_show_average() {
    let dataset = dataset_3d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable    length    animal Average  |Average(Count)
false   - 1       ┐
true    ┘         - shark  [1]      |*
false   - 4       ┘
false   - 1       ┐
false   - 4       - tiger  [1]      |*
true    - 5       ┘
true    - 4       - whale  [1]      |*"#
    );
}

#[test]
fn barchart_3d_show_average_widget() {
    let dataset = dataset_3d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable Average   length Average   animal  |Average(Count)
false  [1]     - 1      [1]     ┐
true   [1]     ┘                - shark   |*
false  [1]     - 4      [1]     ┘
false  [1]     - 1      [1]     ┐
false  [1]     - 4      [1]     - tiger   |*
true   [1]     - 5      [1]     ┘
true   [1]     - 4      [1]     - whale   |*"#
    );
}

#[test]
fn barchart_3d_show_average_both() {
    let dataset = dataset_3d();
    let view = dataset.view_count();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable Average   length Average   animal Average  |Average(Count)
false  [1]     - 1      [1]     ┐
true   [1]     ┘                - shark  [1]      |*
false  [1]     - 4      [1]     ┘
false  [1]     - 1      [1]     ┐
false  [1]     - 4      [1]     - tiger  [1]      |*
true   [1]     - 5      [1]     ┘
true   [1]     - 4      [1]     - whale  [1]      |*"#
    );
}

#[test]
fn barchart_3d_breakdown2() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   Sum(Breakdown(length))
stable    animal  |  1      4      5   |
false   - shark   | ****    *          |
true    ┘
false   - tiger   | ***     **   ******|
true    ┘
true    - whale   |         *          |"#
    );
}

#[rstest]
#[case(17)]
#[case(18)]
#[case(19)]
#[case(20)]
#[case(21)]
#[case(22)]
#[case(23)]
#[case(24)]
#[case(25)]
#[case(26)]
#[case(27)]
#[case(28)]
#[case(30)]
// #[case(31)]
fn barchart_3d_breakdown2_squish(#[case] width_hint: usize) {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        width_hint,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   Sum(Breakdown(length))
stable    animal  |1  4  5 |
false   - shark   |*       |
true    ┘
false   - tiger   |*     **|
true    ┘
true    - whale   |        |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_sum() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                        Sum(Breakdown(length))
stable    animal Sum   |  1      4      5   |
false   - shark  [ 5]  | ****    *          |
true    ┘
false   - tiger  [11]  | ***     **   ******|
true    ┘
true    - whale  [ 1]  |         *          |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_sum_widget() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                      Sum(Breakdown(length))
stable Sum   animal  |  1      4      5   |
false  [2] - shark   | ****    *          |
true   [3] ┘
false  [5] - tiger   | ***     **   ******|
true   [6] ┘
true   [1] - whale   |         *          |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_sum_both() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                           Sum(Breakdown(length))
stable Sum   animal Sum   |  1      4      5   |
false  [2] - shark  [ 5]  | ****    *          |
true   [3] ┘
false  [5] - tiger  [11]  | ***     **   ******|
true   [6] ┘
true   [1] - whale  [ 1]  |         *          |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_average() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                           Average(Breakdown(length))
stable    animal Average  |1 4 5|
false   - shark  [0.7]    |* *  |
true    ┘
false   - tiger  [  1]    |* * *|
true    ┘
true    - whale  [0.3]    |  *  |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_average_widget() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                          Average(Breakdown(length))
stable Average   animal  |1 4 5|
false  [1]     - shark   |* *  |
true   [1]     ┘
false  [1]     - tiger   |* * *|
true   [1]     ┘
true   [1]     - whale   |  *  |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_average_both() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        widget_config: {
            BarChartConfig {
                show_aggregate: true,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                                  Average(Breakdown(length))
stable Average   animal Average  |1 4 5|
false  [1]     - shark  [0.7]    |* *  |
true   [1]     ┘
false  [1]     - tiger  [  1]    |* * *|
true   [1]     ┘
true   [1]     - whale  [0.3]    |  *  |"#
    );
}

#[test]
fn barchart_3d_breakdown3() {
    let dataset = dataset_3d();
    let view = dataset.view_breakdown3();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   Sum(Breakdown(stable))
length    animal  |false   true |
1       - shark   |  **    ***  |
4       ┘
1       ┐
4       - tiger   |*****  ******|
5       ┘
4       - whale   |         *   |"#
    );
}

#[test]
fn abbreviate_barchart_1d() {
    let schema = Schemas::one("animal");
    let builder = Dataset::builder(schema)
        .add(("whalewhalewhalewhale".to_string(),))
        .add(("sharksharksharkshark".to_string(),))
        .add(("tigertigertigertiger".to_string(),));
    let view = builder.view_count();
    let flat = BarChart::new(&view).render(Render {
        width_hint: 1,
        widget_config: BarChartConfig {
            abbreviate: true,
            ..BarChartConfig::default()
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
animal  |Sum(Count)
shar..  |*
tige..  |*
whal..  |*"#
    );
}

#[test]
fn abbreviate_barchart_2d() {
    let schema = Schemas::two("animal", "laminaanimal");
    let builder = Dataset::builder(schema)
        .add((
            "whalewhalewhalewhale".to_string(),
            "whalewhalewhalewhale".to_string(),
        ))
        .add((
            "sharksharksharkshark".to_string(),
            "whalewhalewhalewhale".to_string(),
        ))
        .add((
            "tigertigertigertiger".to_string(),
            "whalewhalewhalewhale".to_string(),
        ));
    let view = builder.view_count();
    let flat = BarChart::new(&view).render(Render {
        width_hint: 1,
        widget_config: BarChartConfig {
            abbreviate: true,
            ..BarChartConfig::default()
        },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
laminaanimal    animal  |Sum(Count)
whalewhale..  - shar..  |*
whalewhale..  - tige..  |*
whalewhale..  - whal..  |*"#
    );
}
