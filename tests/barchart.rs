use flat::*;
use rstest::rstest;

#[test]
fn barchart_1d() {
    let schema = Schema::one("anml");
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(),), 0)
        .add(("shark".to_string(),), 1)
        .add(("shark".to_string(),), 3)
        .add(("tiger".to_string(),), 1)
        .add(("tiger".to_string(),), 3)
        .add(("tiger".to_string(),), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
anml
shark   ****
tiger   *******
whale   "#
    );
}

#[test]
fn barchart_2d() {
    let schema = Schema::two("animal", "length");
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length   animal
1      - shark   ****
4      ┘
1      ┐
4      - tiger   *******
5      ┘
4      - whale   "#
    );
}

#[rstest]
#[case(17)]
#[case(18)]
#[case(19)]
// #[case(20)]
fn barchart_2d_squish(#[case] width_hint: usize) {
    let schema = Schema::two("animal", "length");
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render {
        width_hint,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length   animal
1      - shark   *
4      ┘
1      ┐
4      - tiger   **
5      ┘
4      - whale   "#
    );
}

#[test]
fn barchart_2d_show_sum() {
    let schema = Schema::two("animal", "length");
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length   animal
1      - shark   [4] ****
4      ┘
1      ┐
4      - tiger   [7] *******
5      ┘
4      - whale   [0] "#
    );
}

#[test]
fn barchart_2d_show_average() {
    let schema = Schema::two("animal", "length");
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length   animal
1      - shark   [  2] **
4      ┘
1      ┐
4      - tiger   [2.3] **
5      ┘
4      - whale   [  0] "#
    );
}

#[test]
fn barchart_2d_breakdown() {
    let schema = Schema::two("animal", "length").breakdown_2nd();
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 3)
        .add(("tiger".to_string(), 5u32), 2)
        .add(("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
animal  | 1   4   5 |
shark   |***  *     |
tiger   |*** *** ** |
whale   |           |"#
    );
}

#[test]
fn barchart_3d() {
    let schema = Schema::three("animal", "length", "stable");
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 3)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable   length   animal
true   - 1      - shark   ****
false  - 4      ┘
false  - 1      ┐
false  - 4      - tiger   *******
true   - 5      ┘
true   - 4      - whale   "#
    );
}

#[test]
fn barchart_3d_breakdown2() {
    let schema = Schema::three("animal", "length", "stable").breakdown_2nd();
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 3)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable   animal  | 1   4   5 |
false  - shark   |***  *     |
true   ┘
false  - tiger   |***  *  ***|
true   ┘
true   - whale   |           |"#
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
// #[case(30)]
fn barchart_3d_breakdown2_squish(#[case] width_hint: usize) {
    let schema = Schema::three("animal", "length", "stable").breakdown_2nd();
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 3)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render {
        width_hint,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable   animal  |1  4  5 |
false  - shark   |** *    |
true   ┘
false  - tiger   |** *  **|
true   ┘
true   - whale   |        |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_sum() {
    let schema = Schema::three("animal", "length", "stable").breakdown_2nd();
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable   animal       |  1      4      5   |
false  - shark   [ 4] | ***     *          |
true   ┘
false  - tiger   [10] | ***     *    ******|
true   ┘
true   - whale   [ 0] |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown2_show_average() {
    let schema = Schema::three("animal", "length", "stable").breakdown_2nd();
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
stable   animal        |  1      4      5   |
false  - shark   [1.3] | ***     *          |
true   ┘
false  - tiger   [3.3] | ***     *    ******|
true   ┘
true   - whale   [  0] |                    |"#
    );
}

#[test]
fn barchart_3d_breakdown3() {
    let schema = Schema::three("animal", "length", "stable").breakdown_3rd();
    let builder = BarChart::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 3)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length   animal  |false true |
1      - shark   |  *    *** |
4      ┘
1      ┐
4      - tiger   |****   *** |
5      ┘
4      - whale   |           |"#
    );
}

#[test]
fn abbreviate_barchart_1d() {
    let schema = Schema::one("animal");
    let builder = BarChart::builder(schema)
        .add(("whalewhalewhalewhale".to_string(),), 1)
        .add(("sharksharksharkshark".to_string(),), 2)
        .add(("tigertigertigertiger".to_string(),), 3);
    let flat = builder.render(Render {
        width_hint: 1,
        widget_config: BarChartConfig { abbreviate: true },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
animal
shar..   **
tige..   ***
whal..   *"#
    );
}

#[test]
fn abbreviate_barchart_2d() {
    let schema = Schema::two("animal", "laminaanimal");
    let builder = BarChart::builder(schema)
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
    let flat = builder.render(Render {
        width_hint: 1,
        widget_config: BarChartConfig { abbreviate: true },
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
laminaanimal    animal
whalewhale..  - shar..   **
whalewhale..  - tige..   ***
whalewhale..  - whal..   *"#
    );
}
