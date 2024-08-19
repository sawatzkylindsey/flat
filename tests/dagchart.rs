#[cfg(feature = "primitive_impls")]
mod tests {
    use flat::*;
    use rstest::rstest;

    fn dataset_1d() -> Dataset<Schema1<String>> {
        let schema = Schemas::one("anml");
        DatasetBuilder::new(schema)
            .add(("whale".to_string(),))
            .add(("shark".to_string(),))
            .add(("shark".to_string(),))
            .add(("tiger".to_string(),))
            .add(("tiger".to_string(),))
            .add(("tiger".to_string(),))
            .build()
    }

    #[test]
    fn dagchart_1d() {
        let dataset = dataset_1d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
anml   |Sum(Count)
shark  |**
tiger  |***
whale  |*"#
        );
    }

    #[test]
    fn dagchart_1d_reflective() {
        let schema: Schema1<u64> = Schemas::one("anml");
        let dataset = DatasetBuilder::new(schema)
            .add((1,))
            .add((2,))
            .add((3,))
            .add((2,))
            .add((2,))
            .build();
        let view = dataset.reflect_1st();
        let flat = DagChart::new(&view).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
anml  |Sum(anml)
1     |*
2     |******
3     |***"#
        );
    }

    fn dataset_2d() -> Dataset<Schema2<String, u32>> {
        let schema = Schemas::two("animal", "length");
        DatasetBuilder::new(schema)
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
            .build()
    }

    #[test]
    fn dagchart_2d() {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render::default());
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
    fn dagchart_2d_squish(#[case] width_hint: usize) {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
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
    fn dagchart_2d_show_sum() {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
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
    fn dagchart_2d_show_sum_widget() {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
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
    fn dagchart_2d_show_sum_both() {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            show_aggregate: true,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
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
    fn dagchart_2d_show_average() {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
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
    fn dagchart_2d_show_average_widget() {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
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
    fn dagchart_2d_show_average_both() {
        let dataset = dataset_2d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            show_aggregate: true,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
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
    fn dagchart_2d_count_breakdown() {
        let dataset = dataset_2d();
        let view = dataset.count_breakdown_2nd();
        let flat = DagChart::new(&view).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
         length
         Sum(Count)
animal  | 1   4   5 |
shark   |***  *     |
tiger   |***  *  ***|
whale   |     *     |"#
        );
    }

    fn dataset_3d() -> Dataset<Schema3<String, bool, u32>> {
        let schema = Schemas::three("animal", "stable", "length");
        DatasetBuilder::new(schema)
            .add(("whale".to_string(), true, 4u32))
            .add(("shark".to_string(), false, 4u32))
            .add(("shark".to_string(), false, 1u32))
            .add(("shark".to_string(), true, 1u32))
            .add(("shark".to_string(), true, 1u32))
            .add(("shark".to_string(), true, 1u32))
            .add(("tiger".to_string(), false, 4u32))
            .add(("tiger".to_string(), false, 4u32))
            .add(("tiger".to_string(), true, 5u32))
            .add(("tiger".to_string(), true, 5u32))
            .add(("tiger".to_string(), true, 5u32))
            .add(("tiger".to_string(), true, 5u32))
            .add(("tiger".to_string(), true, 5u32))
            .add(("tiger".to_string(), true, 5u32))
            .add(("tiger".to_string(), false, 1u32))
            .add(("tiger".to_string(), false, 1u32))
            .add(("tiger".to_string(), false, 1u32))
            .build()
    }

    #[test]
    fn dagchart_3d() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
length    stable    animal  |Sum(Count)
1       - false   ┐
4       ┘         - shark   |*****
1       - true    ┘
1       - false   ┐
4       ┘         - tiger   |***********
5       - true    ┘
4       - true    - whale   |*"#
        );
    }

    #[test]
    fn dagchart_3d_show_sum() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
length    stable    animal Sum   |Sum(Count)
1       - false   ┐
4       ┘         - shark  [ 5]  |*****
1       - true    ┘
1       - false   ┐
4       ┘         - tiger  [11]  |***********
5       - true    ┘
4       - true    - whale  [ 1]  |*"#
        );
    }

    #[test]
    fn dagchart_3d_show_sum_widget() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
length Sum   stable Sum   animal  |Sum(Count)
1      [1] - false  [2] ┐
4      [1] ┘            - shark   |*****
1      [3] - true   [3] ┘
1      [3] - false  [5] ┐
4      [2] ┘            - tiger   |***********
5      [6] - true   [6] ┘
4      [1] - true   [1] - whale   |*"#
        );
    }

    #[test]
    fn dagchart_3d_show_sum_both() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            show_aggregate: true,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
length Sum   stable Sum   animal Sum   |Sum(Count)
1      [1] - false  [2] ┐
4      [1] ┘            - shark  [ 5]  |*****
1      [3] - true   [3] ┘
1      [3] - false  [5] ┐
4      [2] ┘            - tiger  [11]  |***********
5      [6] - true   [6] ┘
4      [1] - true   [1] - whale  [ 1]  |*"#
        );
    }

    #[test]
    fn dagchart_3d_show_average() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
length    stable    animal Average  |Average(Count)
1       - false   ┐
4       ┘         - shark  [1]      |*
1       - true    ┘
1       - false   ┐
4       ┘         - tiger  [1]      |*
5       - true    ┘
4       - true    - whale  [1]      |*"#
        );
    }

    #[test]
    fn dagchart_3d_show_average_widget() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
length Average   stable Average   animal  |Average(Count)
1      [1]     - false  [1]     ┐
4      [1]     ┘                - shark   |*
1      [1]     - true   [1]     ┘
1      [1]     - false  [1]     ┐
4      [1]     ┘                - tiger   |*
5      [1]     - true   [1]     ┘
4      [1]     - true   [1]     - whale   |*"#
        );
    }

    #[test]
    fn dagchart_3d_show_average_both() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            show_aggregate: true,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
length Average   stable Average   animal Average  |Average(Count)
1      [1]     - false  [1]     ┐
4      [1]     ┘                - shark  [1]      |*
1      [1]     - true   [1]     ┘
1      [1]     - false  [1]     ┐
4      [1]     ┘                - tiger  [1]      |*
5      [1]     - true   [1]     ┘
4      [1]     - true   [1]     - whale  [1]      |*"#
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
    fn dagchart_3d_breakdown_squish(#[case] width_hint: usize) {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            width_hint,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                   Sum(length)
stable    animal  |1  4  5 |
false   - shark   |        |
true    ┘
false   - tiger   |      **|
true    ┘
true    - whale   |        |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown_show_sum() {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                        Sum(length)
stable    animal Sum   |              1                              4                              5               |
false   - shark  [ 8]  |             ****                           ****                                            |
true    ┘
false   - tiger  [41]  |             ***                          ********            ******************************|
true    ┘
true    - whale  [ 4]  |                                            ****                                            |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown_show_sum_widget() {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                       Sum(length)
stable Sum    animal  |              1                              4                              5               |
false  [ 5] - shark   |             ****                           ****                                            |
true   [ 3] ┘
false  [11] - tiger   |             ***                          ********            ******************************|
true   [30] ┘
true   [ 4] - whale   |                                            ****                                            |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown_show_sum_both() {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            show_aggregate: true,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                            Sum(length)
stable Sum    animal Sum   |              1                              4                              5               |
false  [ 5] - shark  [ 8]  |             ****                           ****                                            |
true   [ 3] ┘
false  [11] - tiger  [41]  |             ***                          ********            ******************************|
true   [30] ┘
true   [ 4] - whale  [ 4]  |                                            ****                                            |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown_show_average() {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            show_aggregate: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                           Average(length)
stable    animal Average  |  1     4     5  |
false   - shark  [1.7]    |  *   ****       |
true    ┘
false   - tiger  [3.3]    |  *   ****  *****|
true    ┘
true    - whale  [1.3]    |      ****       |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown_show_average_widget() {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                          Average(length)
stable Average   animal  |  1     4     5  |
false  [2.5]   - shark   |  *   ****       |
true   [  1]   ┘
false  [2.2]   - tiger   |  *   ****  *****|
true   [  5]   ┘
true   [  4]   - whale   |      ****       |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown_show_average_both() {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            aggregate: Aggregate::Average,
            show_aggregate: true,
            widget_config: {
                DagChartConfig {
                    show_aggregate: true,
                    ..DagChartConfig::default()
                }
            },
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                                  Average(length)
stable Average   animal Average  |  1     4     5  |
false  [2.5]   - shark  [1.7]    |  *   ****       |
true   [  1]   ┘
false  [2.2]   - tiger  [3.3]    |  *   ****  *****|
true   [  5]   ┘
true   [  4]   - whale  [1.3]    |      ****       |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown() {
        let dataset = dataset_3d();
        let view = dataset.breakdown_3rd();
        let flat = DagChart::new(&view).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                   Sum(length)
stable    animal  |              1                              4                              5               |
false   - shark   |             ****                           ****                                            |
true    ┘
false   - tiger   |             ***                          ********            ******************************|
true    ┘
true    - whale   |                                            ****                                            |"#
        );
    }

    #[test]
    fn dagchart_3d_breakdown_view() {
        let dataset = dataset_3d();
        let view = dataset.view_3rd_breakdown_2nd();
        let flat = DagChart::new(&view).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
         stable
         Sum(length)
animal  |            false                           true             |
shark   |            *****                           ***              |
tiger   |         ***********           ******************************|
whale   |                                            ****             |"#
        );
    }

    #[test]
    fn dagchart_4d_breakdown_view() {
        let schema = Schemas::four("animal", "stable", "length", "width");
        let dataset = DatasetBuilder::new(schema)
            .add(("whale".to_string(), true, 4u32, 5u32))
            .add(("shark".to_string(), false, 4u32, 5u32))
            .add(("shark".to_string(), true, 2u32, 3u32))
            .add(("tiger".to_string(), false, 1u32, 2u32))
            .build();
        let view = dataset.view_4th_breakdown_3rd();
        let flat = DagChart::new(&view).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                   length
                   Sum(width)
stable    animal  |  1     2     4  |
false   - shark   |       ***  *****|
true    ┘
false   - tiger   | **              |
true    - whale   |            *****|"#
        );
    }

    #[test]
    fn abbreviate_dagchart_1d() {
        let schema = Schemas::one("animal");
        let dataset = DatasetBuilder::new(schema)
            .add(("whalewhalewhalewhale".to_string(),))
            .add(("sharksharksharkshark".to_string(),))
            .add(("tigertigertigertiger".to_string(),))
            .build();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            width_hint: 1,
            widget_config: DagChartConfig {
                abbreviate: true,
                ..DagChartConfig::default()
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
    fn abbreviate_dagchart_2d() {
        let schema = Schemas::two("animal", "laminaanimal");
        let dataset = DatasetBuilder::new(schema)
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
            ))
            .build();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            width_hint: 1,
            widget_config: DagChartConfig {
                abbreviate: true,
                ..DagChartConfig::default()
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
}
