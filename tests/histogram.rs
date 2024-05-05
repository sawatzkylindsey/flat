use flat::*;
use rstest::rstest;

#[test]
fn histogram() {
    let schema = Schema::one("length");
    let mut builder = Histogram::builder(schema, 5);

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

#[test]
fn histogram_u64() {
    let schema = Schema::one("length");
    let mut builder = Histogram::builder(schema, 5);

    for i in 0..10 {
        builder = builder.add(i % 10, i);
    }

    let flat = builder.render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
[0, 2)   *
[2, 4)   *****
[4, 6)   *********
[6, 8)   *************
[8, 10]  *****************"#
    );
}

#[rstest]
#[case(12)]
#[case(13)]
#[case(14)]
// #[case(15)]
fn histogram_squish(#[case] render_width: usize) {
    let schema = Schema::one("length");
    let mut builder = Histogram::builder(schema, 5);

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
    let schema = Schema::one("length");
    let mut builder = Histogram::builder(schema, 5);

    for i in 0..10 {
        builder = builder.add((i % 10) as f64, i);
    }

    let flat = builder.render(Render {
        show_total: true,
        ..Render::default()
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
    let schema = Schema::one("length");
    let mut builder = Histogram::builder(schema, 5);

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
