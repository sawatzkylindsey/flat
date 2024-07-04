#[cfg(feature = "primitive_impls")]
mod tests {
    use flat::{DagChart, DatasetBuilder, Histogram, PathChart, Render, Schemas};

    #[test]
    fn abbreviate_dagchart_count_breakdown_hint1() {
        let schema = Schemas::two("animal", "dinosaur");
        let dataset = DatasetBuilder::new(schema)
            .add(("whale".to_string(), "tyrannosaurs".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .build();
        let view = dataset.count_breakdown_2nd();
        let flat = DagChart::new(&view).render(Render {
            width_hint: 1,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
         dinosaur
         Sum(Count)
animal  |pt.. tr.. ty..|
shark   |      *       |
tiger   | **           |
whale   |              |"#
        );
    }

    #[test]
    fn abbreviate_dagchart_count_breakdown_hint15() {
        let schema = Schemas::two("animal", "dinosaur");
        let dataset = DatasetBuilder::new(schema)
            .add(("whale".to_string(), "tyrannosaurs".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .build();
        let view = dataset.count_breakdown_2nd();
        let flat = DagChart::new(&view).render(Render {
            width_hint: 15,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
         dinosaur
         Sum(Count)
animal  |pt.. tr.. ty..|
shark   |      *       |
tiger   | **           |
whale   |              |"#
        );
    }

    #[test]
    fn abbreviate_dagchart_count_breakdown_hint30() {
        let schema = Schemas::two("animal", "dinosaur");
        let dataset = DatasetBuilder::new(schema)
            .add(("whale".to_string(), "tyrannosaurs".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .build();
        let view = dataset.count_breakdown_2nd();
        let flat = DagChart::new(&view).render(Render {
            width_hint: 30,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
         dinosaur
         Sum(Count)
animal  |pter.. tric.. tyra..|
shark   |         **         |
tiger   | ***                |
whale   |                *   |"#
        );
    }

    #[test]
    fn abbreviate_dagchart_count_breakdown_hint180() {
        let schema = Schemas::two("animal", "dinosaur");
        let dataset = DatasetBuilder::new(schema)
            .add(("whale".to_string(), "tyrannosaurs".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .build();
        let view = dataset.count_breakdown_2nd();
        let flat = DagChart::new(&view).render(Render {
            width_hint: 180,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
         dinosaur
         Sum(Count)
animal  |pterodactyl  triceratops  tyrannosaurs|
shark   |                  **                  |
tiger   |    ***                               |
whale   |                               *      |"#
        );
    }

    #[test]
    fn dagchart_3d_count_breakdown_abbreviate() {
        let schema = Schemas::three("animal", "length", "stable");
        let dataset = DatasetBuilder::new(schema)
            .add(("whale".to_string(), 4u32, true))
            .add(("shark".to_string(), 4u32, false))
            .add(("shark".to_string(), 1u32, true))
            .add(("shark".to_string(), 1u32, true))
            .add(("shark".to_string(), 1u32, true))
            .add(("tiger".to_string(), 4u32, false))
            .add(("tiger".to_string(), 5u32, true))
            .add(("tiger".to_string(), 5u32, true))
            .add(("tiger".to_string(), 5u32, true))
            .add(("tiger".to_string(), 1u32, false))
            .add(("tiger".to_string(), 1u32, false))
            .add(("tiger".to_string(), 1u32, false))
            .build();
        let view = dataset.count_breakdown_3rd();
        let flat = DagChart::new(&view).render(Render {
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                   stable
                   Sum(Count)
length    animal  |false true |
1       - shark   |  *    *** |
4       ┘
1       ┐
4       - tiger   |****   *** |
5       ┘
4       - whale   |        *  |"#
        );
    }

    #[test]
    fn histogram_count_breakdown_abbreviate() {
        let pets = vec!["ralf", "kipp", "orville"];
        let schema = Schemas::two("length", "pet");
        let mut builder = DatasetBuilder::new(schema);

        for i in 0..10 {
            for _ in 0..i {
                builder.update(((i % 10) as f64, pets[i % 3]));
            }
        }

        let dataset = builder.build();
        let view = dataset.count_breakdown_2nd();
        let flat = Histogram::new(&view, 5).render(Render {
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                           pet
                           Sum(Count)
length                    |  kipp     orvi..     ralf   |
[1, 2.6)                  |    *        **              |
[2.6, 4.2)                |  ****                 ***   |
[4.2, 5.800000000000001)  |            *****            |
[5.800000000000001, 7.4)  | *******             ******  |
[7.4, 9]                  |          ********  *********|"#
        );
    }

    #[test]
    fn histogram_count_breakdown_abbreviate_hint1() {
        let pets = vec!["ralf", "kipp", "orville"];
        let schema = Schemas::two("length", "pet");
        let mut builder = DatasetBuilder::new(schema);

        for i in 0..10 {
            for _ in 0..i {
                builder.update(((i % 10) as f64, pets[i % 3]));
            }
        }

        let dataset = builder.build();
        let view = dataset.count_breakdown_2nd();
        let flat = Histogram::new(&view, 5).render(Render {
            width_hint: 1,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                           pet
                           Sum(Count)
length                    |k.. o.. r..|
[1, 2.6)                  |           |
[2.6, 4.2)                |           |
[4.2, 5.800000000000001)  |     *     |
[5.800000000000001, 7.4)  | *       * |
[7.4, 9]                  |     *  ** |"#
        );
    }

    #[test]
    fn histogram_count_breakdown_abbreviate_hint15() {
        let pets = vec!["ralf", "kipp", "orville"];
        let schema = Schemas::two("length", "pet");
        let mut builder = DatasetBuilder::new(schema);

        for i in 0..10 {
            for _ in 0..i {
                builder.update(((i % 10) as f64, pets[i % 3]));
            }
        }

        let dataset = builder.build();
        let view = dataset.count_breakdown_2nd();
        let flat = Histogram::new(&view, 5).render(Render {
            width_hint: 15,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                           pet
                           Sum(Count)
length                    |k.. o.. r..|
[1, 2.6)                  |           |
[2.6, 4.2)                |           |
[4.2, 5.800000000000001)  |     *     |
[5.800000000000001, 7.4)  | *       * |
[7.4, 9]                  |     *  ** |"#
        );
    }

    #[test]
    fn histogram_count_breakdown_abbreviate_hint30() {
        let pets = vec!["ralf", "kipp", "orville"];
        let schema = Schemas::two("length", "pet");
        let mut builder = DatasetBuilder::new(schema);

        for i in 0..10 {
            for _ in 0..i {
                builder.update(((i % 10) as f64, pets[i % 3]));
            }
        }

        let dataset = builder.build();
        let view = dataset.count_breakdown_2nd();
        let flat = Histogram::new(&view, 5).render(Render {
            width_hint: 30,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                           pet
                           Sum(Count)
length                    |k.. o.. r..|
[1, 2.6)                  |           |
[2.6, 4.2)                |           |
[4.2, 5.800000000000001)  |     *     |
[5.800000000000001, 7.4)  | *       * |
[7.4, 9]                  |     *  ** |"#
        );
    }

    #[test]
    fn abbreviate_dagchart_count_breakdown_separation() {
        let schema = Schemas::two("pterodactyl", "dinosaur");
        let dataset = DatasetBuilder::new(schema)
            .add(("triceratops".to_string(), "tyrannosaurs".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .build();
        let view = dataset.count_breakdown_2nd();
        let flat = DagChart::new(&view).render(Render {
            width_hint: 1,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
              dinosaur
              Sum(Count)
pterodactyl  |pt.. tr.. ty..|
shark        |      *       |
tiger        | **           |
triceratops  |              |"#
        );
    }

    #[test]
    fn abbreviate_pathchart_count_breakdown_separation() {
        let schema = Schemas::two("pterodactyl", "dinosaur");
        let dataset = DatasetBuilder::new(schema)
            .add(("triceratops".to_string(), "tyrannosaurs".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .build();
        let view = dataset.count_breakdown_2nd();
        let flat = PathChart::new(&view).render(Render {
            width_hint: 1,
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
               dinosaur
               Sum(Count)
/pterodactyl  |pt.. tr.. ty..|
/shark        |      *       |
/tiger        | **           |
/triceratops  |              |"#
        );
    }

    #[test]
    fn abbreviate_non_breakdown() {
        let schema = Schemas::two("animal", "dinosaur");
        let dataset = DatasetBuilder::new(schema)
            .add(("whale".to_string(), "tyrannosaurs".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("shark".to_string(), "triceratops".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .add(("tiger".to_string(), "pterodactyl".to_string()))
            .build();
        let view = dataset.count();
        let flat = DagChart::new(&view).render(Render {
            abbreviate_breakdown: true,
            ..Render::default()
        });
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
dinosaur        animal  |Sum(Count)
triceratops   - shark   |**
pterodactyl   - tiger   |***
tyrannosaurs  - whale   |*"#
        );
    }
}
