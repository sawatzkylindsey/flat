use flat::*;
use rstest::rstest;

#[test]
fn categorical_1d() {
    let schema = Schema::one("animal");
    let builder = Categorical::builder(schema)
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
animal
shark   ****
tiger   *******
whale   "#
    );
}

#[test]
fn categorical_2d() {
    let schema = Schema::two("animal", "length");
    let builder = Categorical::builder(schema)
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
fn categorical_2d_squish(#[case] render_width: usize) {
    let schema = Schema::two("animal", "length");
    let builder = Categorical::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render {
        render_width,
        show_total: false,
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
fn categorical_2d_show_total() {
    let schema = Schema::two("animal", "length");
    let builder = Categorical::builder(schema)
        .add(("whale".to_string(), 4u32), 0)
        .add(("shark".to_string(), 4u32), 1)
        .add(("shark".to_string(), 1u32), 3)
        .add(("tiger".to_string(), 4u32), 1)
        .add(("tiger".to_string(), 5u32), 3)
        .add(("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render {
        show_total: true,
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
fn categorical_2d_breakdown() {
    let schema = Schema::two("animal", "length").breakdown_2nd();
    let builder = Categorical::builder(schema)
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
fn categorical_3d() {
    let schema = Schema::three("animal", "length", "stable");
    let builder = Categorical::builder(schema)
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
fn categorical_3d_breakdown2() {
    let schema = Schema::three("animal", "length", "stable").breakdown_2nd();
    let builder = Categorical::builder(schema)
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
fn categorical_3d_breakdown2_squish(#[case] render_width: usize) {
    let schema = Schema::three("animal", "length", "stable").breakdown_2nd();
    let builder = Categorical::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 3)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render {
        render_width,
        show_total: false,
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
fn categorical_3d_breakdown2_show_total() {
    let schema = Schema::three("animal", "length", "stable").breakdown_2nd();
    let builder = Categorical::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 6)
        .add(("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render {
        show_total: true,
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
fn categorical_3d_breakdown3() {
    let schema = Schema::three("animal", "length", "stable").breakdown_3rd();
    let builder = Categorical::builder(schema)
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
