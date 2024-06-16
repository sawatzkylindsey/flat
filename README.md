# flat

Project multi-dimensional data onto the **flat** textual plane.

    use flat::*;
    
    let schema = Schemas::three("animal", "length", "stable");
    let dataset = Dataset::builder(schema)
        .add(("whale".to_string(), 4u32, true))
        .add(("shark".to_string(), 4u32, false))
        .add(("shark".to_string(), 1u32, false))
        .add(("shark".to_string(), 1u32, true))
        .add(("shark".to_string(), 1u32, true))
        .add(("shark".to_string(), 1u32, true))
        .add(("tiger".to_string(), 4u32, false))
        .add(("tiger".to_string(), 4u32, false))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 5u32, true))
        .add(("tiger".to_string(), 1u32, false))
        .add(("tiger".to_string(), 1u32, false))
        .add(("tiger".to_string(), 1u32, false))
        .build();
    let view = dataset.breakdown_3rd();
    let flat = BarChart::new(&view).render(Render::default());
    assert_eq!(
        format!("\n{}", flat.to_string()),
        r#"
                       Sum(Breakdown(stable))
    length    animal  |false   true |
    1       - shark   |  **    ***  |
    4       ┘
    1       ┐
    4       - tiger   |*****  ******|
    5       ┘
    4       - whale   |         *   |"#
    );
