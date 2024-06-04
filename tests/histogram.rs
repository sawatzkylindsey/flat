use flat::*;
use rstest::rstest;

#[test]
fn histogram() {
    let schema = Schemas::one("length");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update(((i % 10) as f64,));
        }
    }

    let view = builder.view();
    let flat = Histogram::new(&view, 5).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length                    |Sum(length)
[1, 2.6)                  |****
[2.6, 4.2)                |**********************
[4.2, 5.800000000000001)  |**********************
[5.800000000000001, 7.4)  |*****************************************************************************
[7.4, 9]                  |*************************************************************************************************************************************"#
    );
}

#[test]
fn histogram_u64() {
    let schema = Schemas::one("length");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update((i % 10,));
        }
    }

    let view = builder.view();
    let flat = Histogram::new(&view, 5).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length   |Sum(length)
[1, 3)   |*****
[3, 5)   |*************************
[5, 7)   |*************************************************************
[7, 9)   |*****************************************************************************************************************
[9, 11]  |*********************************************************************************"#
    );
}

#[rstest]
#[case(12)]
#[case(13)]
#[case(14)]
#[case(15)]
// #[case(16)]
fn histogram_squish(#[case] width_hint: usize) {
    let schema = Schemas::one("length");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update(((i % 8) as f64,));
        }
    }

    let view = builder.view();
    let flat = Histogram::new(&view, 5).render(Render {
        width_hint,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length                    |Sum(length)
[0, 1.4)                  |
[1.4, 2.8)                |
[2.8, 4.199999999999999)  |
[4.199999999999999, 5.6)  |
[5.6, 7]                  |**"#
    );
}

#[test]
fn histogram_show_sum() {
    let schema = Schemas::one("length");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update(((i % 10) as f64,));
        }
    }

    let view = builder.view();
    let flat = Histogram::new(&view, 5).render(Render {
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length                   Sum    |Sum(length)
[1, 2.6)                 [  5]  |****
[2.6, 4.2)               [ 25]  |*********************
[4.2, 5.800000000000001) [ 25]  |*********************
[5.800000000000001, 7.4) [ 85]  |**************************************************************************
[7.4, 9]                 [145]  |*******************************************************************************************************************************"#
    );
}

#[test]
fn histogram_show_average() {
    let schema = Schemas::one("length");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update(((i % 10) as f64,));
        }
    }

    let view = builder.view();
    let flat = Histogram::new(&view, 5).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length                   Average  |Average(length)
[1, 2.6)                 [1.7]    |**
[2.6, 4.2)               [3.6]    |****
[4.2, 5.800000000000001) [  5]    |*****
[5.800000000000001, 7.4) [6.5]    |*******
[7.4, 9]                 [8.5]    |*********"#
    );
}

#[rstest]
#[case(17)]
#[case(18)]
#[case(19)]
#[case(20)]
// #[case(21)]
fn histogram_show_sum_squish(#[case] width_hint: usize) {
    let schema = Schemas::one("length");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update(((i % 8) as f64,));
        }
    }

    let view = builder.view();
    let flat = Histogram::new(&view, 5).render(Render {
        width_hint,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length                   Sum   |Sum(length)
[0, 1.4)                 [10]  |
[1.4, 2.8)               [ 4]  |
[2.8, 4.199999999999999) [25]  |
[4.199999999999999, 5.6) [25]  |
[5.6, 7]                 [85]  |**"#
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
// #[case(24)]
fn histogram_show_average_squish(#[case] width_hint: usize) {
    let schema = Schemas::one("length");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update(((i % 8) as f64,));
        }
    }

    let view = builder.view();
    let flat = Histogram::new(&view, 5).render(Render {
        aggregate: Aggregate::Average,
        width_hint,
        show_aggregate: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
length                   Average  |Average(length)
[0, 1.4)                 [0.6]    |
[1.4, 2.8)               [  2]    |
[2.8, 4.199999999999999) [3.6]    |*
[4.199999999999999, 5.6) [  5]    |*
[5.6, 7]                 [6.5]    |**"#
    );
}

#[test]
fn histogram_breakdown() {
    let pets = vec!["ralf", "kipp", "orville"];
    let schema = Schemas::two("length", "pet");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        for _ in 0..i {
            builder.update(((i % 10) as f64, pets[i % 3]));
        }
    }

    let view = builder.view_breakdown2();
    let flat = Histogram::new(&view, 5).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                           Sum(Breakdown(pet))
length                    |  kipp     orville    ralf   |
[1, 2.6)                  |    *        **              |
[2.6, 4.2)                |  ****                 ***   |
[4.2, 5.800000000000001)  |            *****            |
[5.800000000000001, 7.4)  | *******             ******  |
[7.4, 9]                  |          ********  *********|"#
    );
}
