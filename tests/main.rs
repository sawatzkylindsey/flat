use flat::*;
use rstest::rstest;

#[test]
fn categorical_1d() {
    let builder = Categorical::builder()
        .add(OneD::new("whale".to_string()), 0)
        .add(OneD::new("shark".to_string()), 1)
        .add(OneD::new("shark".to_string()), 3)
        .add(OneD::new("tiger".to_string()), 1)
        .add(OneD::new("tiger".to_string()), 3)
        .add(OneD::new("tiger".to_string()), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
shark ᗒ  ****
tiger ᐅ  *******
whale ᗒ  "#
    );
}

#[test]
fn categorical_2d() {
    let builder = Categorical::builder()
        .add(TwoD::new("whale".to_string(), 4u32), 0)
        .add(TwoD::new("shark".to_string(), 4u32), 1)
        .add(TwoD::new("shark".to_string(), 1u32), 3)
        .add(TwoD::new("tiger".to_string(), 4u32), 1)
        .add(TwoD::new("tiger".to_string(), 5u32), 3)
        .add(TwoD::new("tiger".to_string(), 1u32), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
1 ᗒ shark ᗒ  ****
4 ᗒ
1 ᐅ
4 ᐅ tiger ᐅ  *******
5 ᐅ
4 ᗒ whale ᗒ  "#
    );
}

#[test]
fn categorical_3d() {
    let builder = Categorical::builder()
        .add(ThreeD::new("whale".to_string(), 4u32, true), 0)
        .add(ThreeD::new("shark".to_string(), 4u32, false), 1)
        .add(ThreeD::new("shark".to_string(), 1u32, true), 3)
        .add(ThreeD::new("tiger".to_string(), 4u32, false), 1)
        .add(ThreeD::new("tiger".to_string(), 5u32, true), 3)
        .add(ThreeD::new("tiger".to_string(), 1u32, false), 3);
    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
true  ᗒ 1 ᗒ shark ᗒ  ****
false ᗒ 4 ᗒ
false ᐅ 1 ᐅ
false ᐅ 4 ᐅ tiger ᐅ  *******
true  ᐅ 5 ᐅ
true  ᗒ 4 ᗒ whale ᗒ  "#
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
        show_values: false,
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
fn histogram_show() {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 10) as f64, i);
    }

    let flat = builder.render(Render {
        render_width: 120,
        show_values: true,
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
fn histogram_show_squish(#[case] render_width: usize) {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 8) as f64, i);
    }

    // Make sure one of the bins has a count zero (0).
    builder = builder.add(9.0, 0);

    let flat = builder.render(Render {
        render_width,
        show_values: true,
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
