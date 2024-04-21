use flat::*;

#[test]
fn categorical() {
    let builder = Categorical::builder();
    let builder = builder
        .add(TwoD::new("abc".to_string(), 4u32), 1)
        .add(TwoD::new("abc".to_string(), 3u32), 2)
        .add(TwoD::new("def".to_string(), 4u32), 3);
    let flat = builder.render();
    assert_eq!(
        flat.to_string(),
        r#"
abc  4  ****
def

abc  3  *"#
    );
}

#[test]
fn histogram() {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 10) as f64, i);
    }

    let flat = builder.render(false);
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
fn histogram_show() {
    let mut builder = Histogram::builder(5);

    for i in 0..10 {
        builder = builder.add((i % 10) as f64, i);
    }

    let flat = builder.render(true);
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
