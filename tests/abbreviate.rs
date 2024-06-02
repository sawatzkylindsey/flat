use flat::{BarChart, Dataset, Histogram, Render, Schemas};

#[test]
fn abbreviate_barchart_breakdown_hint1() {
    let schema = Schemas::two("animal", "dinosaur", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), "tyrannosaurs".to_string()), 1)
        .add(("shark".to_string(), "triceratops".to_string()), 2)
        .add(("tiger".to_string(), "pterodactyl".to_string()), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        width_hint: 1,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
         dinosaur
animal  |pt.. tr.. ty..|
shark   |      *       |
tiger   | **           |
whale   |              |"#
    );
}

#[test]
fn abbreviate_barchart_breakdown_hint15() {
    let schema = Schemas::two("animal", "dinosaur", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), "tyrannosaurs".to_string()), 1)
        .add(("shark".to_string(), "triceratops".to_string()), 2)
        .add(("tiger".to_string(), "pterodactyl".to_string()), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        width_hint: 15,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
         dinosaur
animal  |pt.. tr.. ty..|
shark   |      *       |
tiger   | **           |
whale   |              |"#
    );
}

#[test]
fn abbreviate_barchart_breakdown_hint30() {
    let schema = Schemas::two("animal", "dinosaur", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), "tyrannosaurs".to_string()), 1)
        .add(("shark".to_string(), "triceratops".to_string()), 2)
        .add(("tiger".to_string(), "pterodactyl".to_string()), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        width_hint: 30,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
         dinosaur
animal  |pter.. tric.. tyra..|
shark   |         **         |
tiger   | ***                |
whale   |                *   |"#
    );
}

#[test]
fn abbreviate_barchart_breakdown_hint180() {
    let schema = Schemas::two("animal", "dinosaur", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), "tyrannosaurs".to_string()), 1)
        .add(("shark".to_string(), "triceratops".to_string()), 2)
        .add(("tiger".to_string(), "pterodactyl".to_string()), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        width_hint: 180,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
         dinosaur
animal  |pterodactyl  triceratops  tyrannosaurs|
shark   |                  **                  |
tiger   |    ***                               |
whale   |                               *      |"#
    );
}

#[test]
fn barchart_3d_breakdown2_abbreviate() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 3)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   length
stable    animal  | 1   4   5 |
false   - shark   |***  *     |
true    ┘
false   - tiger   |***  *  ***|
true    ┘
true    - whale   |           |"#
    );
}

#[test]
fn barchart_3d_breakdown3_abbreviate() {
    let schema = Schemas::three("animal", "length", "stable", "moot");
    let builder = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true), 0)
        .add(("shark".to_string(), 4u32, false), 1)
        .add(("shark".to_string(), 1u32, true), 3)
        .add(("tiger".to_string(), 4u32, false), 1)
        .add(("tiger".to_string(), 5u32, true), 3)
        .add(("tiger".to_string(), 1u32, false), 3);
    let view = builder.view_breakdown3();
    let flat = BarChart::new(&view).render(Render {
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                   stable
length    animal  |false true |
1       - shark   |  *    *** |
4       ┘
1       ┐
4       - tiger   |****   *** |
5       ┘
4       - whale   |           |"#
    );
}

#[test]
fn histogram_breakdown_abbreviate() {
    let pets = vec!["ralf", "kipp", "orville"];
    let schema = Schemas::two("length", "pet", "moot");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        builder.update(((i % 10) as f64, pets[i % 3]), i as f64);
    }

    let view = builder.view_breakdown2();
    let flat = Histogram::new(&view, 5).render(Render {
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
             pet
length      |  kipp     orvi..     ralf   |
[0, 1.8)    |    *                        |
[1.8, 3.6)  |             **        ***   |
[3.6, 5.4)  |  ****      *****            |
[5.4, 7.2)  | *******             ******  |
[7.2, 9]    |          ********  *********|"#
    );
}

#[test]
fn histogram_breakdown_abbreviate_hint1() {
    let pets = vec!["ralf", "kipp", "orville"];
    let schema = Schemas::two("length", "pet", "moot");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        builder.update(((i % 10) as f64, pets[i % 3]), i as f64);
    }

    let view = builder.view_breakdown2();
    let flat = Histogram::new(&view, 5).render(Render {
        width_hint: 1,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
             pet
length      |k.. o.. r..|
[0, 1.8)    |           |
[1.8, 3.6)  |           |
[3.6, 5.4)  |     *     |
[5.4, 7.2)  | *       * |
[7.2, 9]    |     *  ** |"#
    );
}

#[test]
fn histogram_breakdown_abbreviate_hint15() {
    let pets = vec!["ralf", "kipp", "orville"];
    let schema = Schemas::two("length", "pet", "moot");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        builder.update(((i % 10) as f64, pets[i % 3]), i as f64);
    }

    let view = builder.view_breakdown2();
    let flat = Histogram::new(&view, 5).render(Render {
        width_hint: 15,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
             pet
length      |k.. o.. r..|
[0, 1.8)    |           |
[1.8, 3.6)  |           |
[3.6, 5.4)  |     *     |
[5.4, 7.2)  | *       * |
[7.2, 9]    |     *  ** |"#
    );
}

#[test]
fn histogram_breakdown_abbreviate_hint30() {
    let pets = vec!["ralf", "kipp", "orville"];
    let schema = Schemas::two("length", "pet", "moot");
    let mut builder = Dataset::builder(schema);

    for i in 0..10 {
        builder.update(((i % 10) as f64, pets[i % 3]), i as f64);
    }

    let view = builder.view_breakdown2();
    let flat = Histogram::new(&view, 5).render(Render {
        width_hint: 30,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
             pet
length      | kipp  orvi..  ralf |
[0, 1.8)    |                    |
[1.8, 3.6)  |                *   |
[3.6, 5.4)  |  *      **         |
[5.4, 7.2)  | ***            **  |
[7.2, 9]    |        ***    **** |"#
    );
}

#[test]
fn abbreviate_barchart_breakdown_separation() {
    let schema = Schemas::two("pterodactyl", "dinosaur", "moot");
    let builder = Dataset::builder(schema)
        .add(("triceratops".to_string(), "tyrannosaurs".to_string()), 1)
        .add(("shark".to_string(), "triceratops".to_string()), 2)
        .add(("tiger".to_string(), "pterodactyl".to_string()), 3);
    let view = builder.view_breakdown2();
    let flat = BarChart::new(&view).render(Render {
        width_hint: 1,
        abbreviate_breakdown: true,
        ..Render::default()
    });
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
              dinosaur
pterodactyl  |pt.. tr.. ty..|
shark        |      *       |
tiger        | **           |
triceratops  |              |"#
    );
}
