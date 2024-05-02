use flat::*;
use rstest::rstest;

#[test]
fn categorical_1d() {
    let schema: Schema1<String> = Schema::one("animal");
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
    let schema: Schema2<String, u32> = Schema::two("animal", "length");
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

// TODO
#[ignore]
#[test]
fn categorical_2d_breakdown() {
    let schema: Schema2<String, u32> =
        Schema::two("animal", "length").breakdown(Breakdown2::Second);
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
length   animal     1  4  5
1      > shark  >  []
4      >
1      -
4      - tiger  -  []
5      -
4      ~ whale  ~  []"#
    );
}

#[test]
fn categorical_3d() {
    let schema: Schema3<String, u32, bool> = Schema::three("animal", "length", "stable");
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
fn histogram() {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 10) as f64, i);
    }

    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
[0, 1.8)    *
[1.8, 3.6)  *****
[3.6, 5.4)  *********
[5.4, 7.2)  *************
[7.2, 9]    *****************"#
    );
}

#[rstest]
#[case(12)]
#[case(13)]
#[case(14)]
// #[case(15)]
fn histograsm_squish(#[case] render_width: usize) {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 8) as f64, i);
    }

    // Make sure one of the bins has a count zero (0).
    builder = builder.add(9.0, 0);

    let flat = builder.render(Render {
        render_width,
        show_total: false,
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
[0, 1.8)    **
[1.8, 3.6)  *
[3.6, 5.4)  *
[5.4, 7.2)  **
[7.2, 9]    "#
    );
}

#[test]
fn histogram_show_total() {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 10) as f64, i);
    }

    let flat = builder.render(Render {
        render_width: 120,
        show_total: true,
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
[0, 1.8)    [ 1] *
[1.8, 3.6)  [ 5] *****
[3.6, 5.4)  [ 9] *********
[5.4, 7.2)  [13] *************
[7.2, 9]    [17] *****************"#
    );
}

#[rstest]
#[case(17)]
#[case(18)]
#[case(19)]
// #[case(20)]
fn histogram_show_total_squish(#[case] render_width: usize) {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 8) as f64, i);
    }

    // Make sure one of the bins has a count zero (0).
    builder = builder.add(9.0, 0);

    let flat = builder.render(Render {
        render_width,
        show_total: true,
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
[0, 1.8)    [18] **
[1.8, 3.6)  [ 5] *
[3.6, 5.4)  [ 9] *
[5.4, 7.2)  [13] **
[7.2, 9]    [ 0] "#
    );
}
