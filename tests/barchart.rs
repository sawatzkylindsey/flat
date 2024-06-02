use flat::*;
use rstest::rstest;

#[test]
fn barchart_1d() {
    let schema = Schemas::one("anml", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(),), 0)
        .add(("shark".to_string(),), 1)
        .add(("shark".to_string(),), 3)
        .add(("tiger".to_string(),), 1)
        .add(("tiger".to_string(),), 3)
        .add(("tiger".to_string(),), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
anml   |header
shark  |****
tiger  |*******
whale  |"#
    );
}

#[test]
fn barchart_2d() {
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal  |header
1       - shark   |****
4       ┘
1       ┐
4       - tiger   |*******
5       ┘
4       - whale   |"#
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
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render {
        width_hint,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal  |header
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
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal Sum  |header
1       - shark  [4]  |****
4       ┘
1       ┐
4       - tiger  [7]  |*******
5       ┘
4       - whale  [0]  |"#
    );
}

#[test]
fn barchart_2d_show_sum_widget() {
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
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
length Sum   animal  |header
1      [3] - shark   |****
4      [1] ┘
1      [3] ┐
4      [1] - tiger   |*******
5      [3] ┘
4      [0] - whale   |"#
    );
}

#[test]
fn barchart_2d_show_sum_both() {
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
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
length Sum   animal Sum  |header
1      [3] - shark  [4]  |****
4      [1] ┘
1      [3] ┐
4      [1] - tiger  [7]  |*******
5      [3] ┘
4      [0] - whale  [0]  |"#
    );
}

#[test]
fn barchart_2d_show_average() {
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length    animal Average  |header
1       - shark  [  2]    |**
4       ┘
1       ┐
4       - tiger  [2.3]    |**
5       ┘
4       - whale  [  0]    |"#
    );
}

#[test]
fn barchart_2d_show_average_widget() {
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
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
length Average   animal  |header
1      [3]     - shark   |**
4      [1]     ┘
1      [3]     ┐
4      [1]     - tiger   |**
5      [3]     ┘
4      [0]     - whale   |"#
    );
}

#[test]
fn barchart_2d_show_average_both() {
    let schema = Schemas::two("animal", "length", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view();
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
length Average   animal Average  |header
1      [3]     - shark  [  2]    |**
4      [1]     ┘
1      [3]     ┐
4      [1]     - tiger  [2.3]    |**
5      [3]     ┘
4      [0]     - whale  [  0]    |"#
    );
}

#[test]
fn barchart_2d_breakdown() {
    let schema = Schemas::two("animal", "length", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 3)
        .add(("tiger".to_string(), 5u32), 2)
        .add(("tiger".to_string(), 1u32), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
         length
animal  | 1   4   5 |
shark   |***  *     |
tiger   |*** *** ** |
whale   |           |"#
    );
}

#[test]
fn barchart_3d() {
    let schema = Schemas::three("animal", "length", "stable", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable    length    animal  |header
false   - 1       ┐
true    ┘         - shark   |*****
false   - 4       ┘
false   - 1       ┐
false   - 4       - tiger   |***********
true    - 5       ┘
true    - 4       - whale   |"#
    );
}

#[test]
fn barchart_3d_show_sum() {
    let schema = Schemas::three("animal", "length", "stable", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable    length    animal Sum   |header
false   - 1       ┐
true    ┘         - shark  [ 5]  |*****
false   - 4       ┘
false   - 1       ┐
false   - 4       - tiger  [11]  |***********
true    - 5       ┘
true    - 4       - whale  [ 0]  |"#
    );
}

#[test]
fn barchart_3d_show_sum_widget() {
    let schema = Schemas::three("animal", "length", "stable", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view();
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
stable Sum   length Sum   animal  |header
false  [1] - 1      [4] ┐
true   [3] ┘            - shark   |*****
false  [1] - 4      [1] ┘
false  [3] - 1      [3] ┐
false  [2] - 4      [2] - tiger   |***********
true   [6] - 5      [6] ┘
true   [0] - 4      [0] - whale   |"#
    );
}

#[test]
fn barchart_3d_show_sum_both() {
    let schema = Schemas::three("animal", "length", "stable", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view();
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
stable Sum   length Sum   animal Sum   |header
false  [1] - 1      [4] ┐
true   [3] ┘            - shark  [ 5]  |*****
false  [1] - 4      [1] ┘
false  [3] - 1      [3] ┐
false  [2] - 4      [2] - tiger  [11]  |***********
true   [6] - 5      [6] ┘
true   [0] - 4      [0] - whale  [ 0]  |"#
    );
}

#[test]
fn barchart_3d_show_average() {
    let schema = Schemas::three("animal", "length", "stable", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable    length    animal Average  |header
false   - 1       ┐
true    ┘         - shark  [1.7]    |**
false   - 4       ┘
false   - 1       ┐
false   - 4       - tiger  [2.8]    |***
true    - 5       ┘
true    - 4       - whale  [  0]    |"#
    );
}

#[test]
fn barchart_3d_show_average_widget() {
    let schema = Schemas::three("animal", "length", "stable", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view();
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
stable Average   length Average   animal  |header
false  [1]     - 1      [2]     ┐
true   [3]     ┘                - shark   |**
false  [1]     - 4      [1]     ┘
false  [3]     - 1      [3]     ┐
false  [1]     - 4      [1]     - tiger   |***
true   [6]     - 5      [6]     ┘
true   [0]     - 4      [0]     - whale   |"#
    );
}

#[test]
fn barchart_3d_show_average_both() {
    let schema = Schemas::three("animal", "length", "stable", "header");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view();
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
stable Average   length Average   animal Average  |header
false  [1]     - 1      [2]     ┐
true   [3]     ┘                - shark  [1.7]    |**
false  [1]     - 4      [1]     ┘
false  [3]     - 1      [3]     ┐
false  [1]     - 4      [1]     - tiger  [2.8]    |***
true   [6]     - 5      [6]     ┘
true   [0]     - 4      [0]     - whale  [  0]    |"#
    );
}

#[test]
fn barchart_3d_breakdown2() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   length
stable    animal  |  1      4      5   |
false   - shark   | ****    *          |
true    ┘
false   - tiger   | ***     **   ******|
true    ┘
true    - whale   |                    |"#
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
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        width_hint,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   length
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
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                        length
stable    animal Sum   |  1      4      5   |
false   - shark  [ 5]  | ****    *          |
true    ┘
false   - tiger  [11]  | ***     **   ******|
true    ┘
true    - whale  [ 0]  |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_sum_widget() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
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
                      length
stable Sum   animal  |  1      4      5   |
false  [2] - shark   | ****    *          |
true   [3] ┘
false  [5] - tiger   | ***     **   ******|
true   [6] ┘
true   [0] - whale   |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_sum_both() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
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
                           length
stable Sum   animal Sum   |  1      4      5   |
false  [2] - shark  [ 5]  | ****    *          |
true   [3] ┘
false  [5] - tiger  [11]  | ***     **   ******|
true   [6] ┘
true   [0] - whale  [ 0]  |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_average() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                           length
stable    animal Average  |  1      4      5   |
false   - shark  [  1]    |  **     *          |
true    ┘
false   - tiger  [3.3]    | ***     *    ******|
true    ┘
true    - whale  [  0]    |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_average_widget() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
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
                          length
stable Average   animal  |  1      4      5   |
false  [  1]   - shark   |  **     *          |
true   [  3]   ┘
false  [1.7]   - tiger   | ***     *    ******|
true   [  6]   ┘
true   [  0]   - whale   |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_average_both() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
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
                                  length
stable Average   animal Average  |  1      4      5   |
false  [  1]   - shark  [  1]    |  **     *          |
true   [  3]   ┘
false  [1.7]   - tiger  [3.3]    | ***     *    ******|
true   [  6]   ┘
true   [  0]   - whale  [  0]    |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown3() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown3();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   stable
length    animal  |false   true |
1       - shark   |  **    ***  |
4       ┘
1       ┐
4       - tiger   |*****  ******|
5       ┘
4       - whale   |             |"#
    );
}

#[test]
fn abbreviate_barchart_1d() {
    let schema = Schemas::one("animal", "header");
    let builder = Dataset::builder(schema)
        .add(("whalewhalewhalewhale".to_string(),), 1)
        .add(("sharksharksharkshark".to_string(),), 2)
        .add(("tigertigertigertiger".to_string(),), 3);
    let view = builder.view();
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
animal  |header
shar..  |*
tige..  |**
whal..  |"#
    );
}

#[test]
fn abbreviate_barchart_2d() {
    let schema = Schemas::two("animal", "laminaanimal", "header");
    let builder = Dataset::builder(schema)
        .add(
            (
                "whalewhalewhalewhale".to_string(),
                "whalewhalewhalewhale".to_string(),
            ),
            1,
        )
        .add(
            (
                "sharksharksharkshark".to_string(),
                "whalewhalewhalewhale".to_string(),
            ),
            2,
        )
        .add(
            (
                "tigertigertigertiger".to_string(),
                "whalewhalewhalewhale".to_string(),
            ),
            3,
        );
    let view = builder.view();
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
laminaanimal    animal  |header
whalewhale..  - shar..  |*
whalewhale..  - tige..  |**
whalewhale..  - whal..  |"#
    );
}
