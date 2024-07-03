#[cfg(feature = "pointer_impls")]
mod tests {
    use flat::{BarChart, Dataset, Histogram, Render, Schema3, Schemas};
    use ordered_float::OrderedFloat;

    fn dataset_3d() -> Dataset<Schema3<String, bool, OrderedFloat<f64>>> {
        let schema = Schemas::three("animal", "stable", "length");
        Dataset::builder(schema)
            .add(("whale".to_string(), true, OrderedFloat(4.0)))
            .add(("shark".to_string(), false, OrderedFloat(4.0)))
            .add(("shark".to_string(), false, OrderedFloat(1.0)))
            .add(("shark".to_string(), true, OrderedFloat(1.0)))
            .add(("shark".to_string(), true, OrderedFloat(1.0)))
            .add(("shark".to_string(), true, OrderedFloat(1.0)))
            .add(("tiger".to_string(), false, OrderedFloat(4.0)))
            .add(("tiger".to_string(), false, OrderedFloat(4.0)))
            .add(("tiger".to_string(), true, OrderedFloat(5.0)))
            .add(("tiger".to_string(), true, OrderedFloat(5.0)))
            .add(("tiger".to_string(), true, OrderedFloat(5.0)))
            .add(("tiger".to_string(), true, OrderedFloat(5.0)))
            .add(("tiger".to_string(), true, OrderedFloat(5.0)))
            .add(("tiger".to_string(), true, OrderedFloat(5.0)))
            .add(("tiger".to_string(), false, OrderedFloat(1.0)))
            .add(("tiger".to_string(), false, OrderedFloat(1.0)))
            .add(("tiger".to_string(), false, OrderedFloat(1.0)))
            .build()
    }

    #[test]
    fn pointer_barchart_3d() {
        let dataset = dataset_3d();
        let view = dataset.count();
        let flat = BarChart::new(&view).render(Render::default());
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
    fn histogram_count_breakdown() {
        let pets = vec!["ralf", "kipp", "orville"];
        let schema = Schemas::two("length", "pet");
        let mut builder = Dataset::builder(schema);

        for i in 0..10 {
            for _ in 0..i {
                builder.update(((i % 10) as f64, pets[i % 3]));
            }
        }

        let dataset = builder.build();
        let view = dataset.count_breakdown_2nd();
        let flat = Histogram::new(&view, 5).render(Render::default());
        assert_eq!(
            format!("\n{}", flat.to_string()),
            r#"
                           pet
                           Sum(Count)
length                    |  kipp     orville    ralf   |
[1, 2.6)                  |    *        **              |
[2.6, 4.2)                |  ****                 ***   |
[4.2, 5.800000000000001)  |            *****            |
[5.800000000000001, 7.4)  | *******             ******  |
[7.4, 9]                  |          ********  *********|"#
        );
    }
}
