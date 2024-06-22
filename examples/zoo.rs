use auto_ops::impl_op_ex;
use flat::{
    minimal_precision_string, Aggregate, BarChart, Binnable, Dataset, Histogram, Render, Schemas,
};
use std::ops::Deref;

fn main() {
    let schema = Schemas::three("Quadrant", "Enclosure", "Animal");
    let mut builder = Dataset::builder(schema);

    for triple in enclosure_dataset() {
        builder.update((triple.0, triple.1, triple.2));
    }

    let dataset = builder.build();
    let view = dataset.counting_view();
    let flat = BarChart::new(&view).render(Render::default());
    println!("Density across the zoo enclosures:");
    println!("{flat}");
    println!();

    let view = dataset.breakdown_3rd();
    let flat = BarChart::new(&view).render(Render {
        abbreviate_breakdown: true,
        ..Render::default()
    });
    println!("Density across the zoo enclosures, broken down across the animal species:");
    println!("{flat}");
    println!();

    let schema = Schemas::two("Length (cm)", "Animal");
    let mut builder = Dataset::builder(schema);

    for double in attribute_dataset() {
        builder.update((double.0, double.1));
    }

    let dataset = builder.build();
    // let view = dataset.without_2nd();
    // let flat = Histogram::new(&view, 10).render(Render {
    //     show_aggregate: parameters.verbose,
    //     widget_config: HistogramConfig {
    //         ..HistogramConfig::default()
    //     },
    //     ..Render::default()
    // });
    // println!("abc..:");
    // println!("{flat}");
    // println!();

    let view = dataset.view_1st();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: true,
        width_hint: 30,
        ..Render::default()
    });
    println!("Animal lengths:");
    println!("{flat}");
    println!();

    let view = dataset.counting_view();
    let flat = Histogram::new(&view, 10).render(Render {
        ..Render::default()
    });
    println!("The spread of animals across the length category:");
    println!("{flat}");
    println!();

    let view = dataset.breakdown_2nd();
    let flat = Histogram::new(&view, 10).render(Render {
        abbreviate_breakdown: true,
        ..Render::default()
    });
    println!(
        "The spread of animals across the length category, broken down across the animal species:"
    );
    println!("{flat}");
    println!();
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Animal(String);

impl Animal {
    fn new(animal: impl Into<String>) -> Self {
        Self(animal.into())
    }
}

impl std::fmt::Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Quadrant {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl std::fmt::Display for Quadrant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quadrant::NorthEast => write!(f, "NorthEast"),
            Quadrant::NorthWest => write!(f, "NorthWest"),
            Quadrant::SouthEast => write!(f, "SouthEast"),
            Quadrant::SouthWest => write!(f, "SouthWest"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Enclosure(String);

impl Enclosure {
    fn new(enclosure: impl Into<String>) -> Self {
        Self(enclosure.into())
    }
}

impl std::fmt::Display for Enclosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// #[derive(Clone, PartialEq, PartialOrd)]
#[derive(Clone, PartialEq, PartialOrd)]
struct Length(f64);

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0))
    }
}

impl_op_ex!(+ |a: &Length, b: &Length| -> Length { Length(a.0 + b.0) } );
impl_op_ex!(-|a: &Length, b: &Length| -> Length { Length(a.0 - b.0) });

impl Deref for Length {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Binnable for Length {
    fn multiply(&self, rhs: usize) -> Self {
        Length(self.0 * rhs as f64)
    }

    fn divide(&self, rhs: usize) -> Self {
        Length(self.0 / rhs as f64)
    }
}

// Some fake enclosure data.
fn enclosure_dataset() -> Vec<(Quadrant, Enclosure, Animal)> {
    vec![
        (
            Quadrant::NorthEast,
            Enclosure::new("Pen01"),
            Animal::new("Sea Otter"),
        ),
        (
            Quadrant::NorthEast,
            Enclosure::new("Pen01"),
            Animal::new("Sea Otter"),
        ),
        (
            Quadrant::NorthEast,
            Enclosure::new("Pen01"),
            Animal::new("Sea Otter"),
        ),
        (
            Quadrant::NorthEast,
            Enclosure::new("Pen02"),
            Animal::new("Falcon"),
        ),
        (
            Quadrant::NorthEast,
            Enclosure::new("Pen02"),
            Animal::new("Falcon"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Pen03"),
            Animal::new("Tiger"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Pen03"),
            Animal::new("Tiger"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Pen04"),
            Animal::new("Stork"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Pen04"),
            Animal::new("Crane"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Pen04"),
            Animal::new("Kingfisher"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Open"),
            Animal::new("Flamingo"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Open"),
            Animal::new("Flamingo"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Open"),
            Animal::new("Flamingo"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Open"),
            Animal::new("Flamingo"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Open"),
            Animal::new("Duck"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Open"),
            Animal::new("Duck"),
        ),
        (
            Quadrant::NorthWest,
            Enclosure::new("Open"),
            Animal::new("Duck"),
        ),
        (
            Quadrant::SouthEast,
            Enclosure::new("Pen05"),
            Animal::new("Black Bear"),
        ),
        (
            Quadrant::SouthEast,
            Enclosure::new("Pen05"),
            Animal::new("Black Bear"),
        ),
        (
            Quadrant::SouthWest,
            Enclosure::new("Pen06"),
            Animal::new("Grizzly Bear"),
        ),
        (
            Quadrant::SouthWest,
            Enclosure::new("Pen07"),
            Animal::new("Mountain Goat"),
        ),
        (
            Quadrant::SouthWest,
            Enclosure::new("Pen07"),
            Animal::new("Mountain Goat"),
        ),
        (
            Quadrant::SouthWest,
            Enclosure::new("Pen07"),
            Animal::new("Mountain Goat"),
        ),
    ]
}

// Some fake attribute data.
fn attribute_dataset() -> Vec<(f64, Animal)> {
    vec![
        // Length in cm.
        (120.0, Animal::new("Sea Otter")),
        (130.0, Animal::new("Sea Otter")),
        (150.0, Animal::new("Sea Otter")),
        (060.0, Animal::new("Falcon")),
        (050.0, Animal::new("Falcon")),
        (280.0, Animal::new("Tiger")),
        (290.0, Animal::new("Tiger")),
        (060.0, Animal::new("Stork")),
        (060.0, Animal::new("Crane")),
        (015.0, Animal::new("Kingfisher")),
        (090.0, Animal::new("Flamingo")),
        (090.0, Animal::new("Flamingo")),
        (080.0, Animal::new("Flamingo")),
        (080.0, Animal::new("Flamingo")),
        (025.0, Animal::new("Duck")),
        (032.0, Animal::new("Duck")),
        (030.0, Animal::new("Duck")),
        (030.0, Animal::new("Black Bear")),
        (120.0, Animal::new("Black Bear")),
        (220.0, Animal::new("Grizzly Bear")),
        (110.0, Animal::new("Mountain Goat")),
        (100.0, Animal::new("Mountain Goat")),
        (130.0, Animal::new("Mountain Goat")),
    ]
}

// // Some fake attribute data.
// fn attribute_dataset() -> Vec<(Length, Animal)> {
//     vec![
//         (Length(1.2), Animal::new("Sea Otter")),
//         (Length(1.3), Animal::new("Sea Otter")),
//         (Length(1.5), Animal::new("Sea Otter")),
//         (Length(0.6), Animal::new("Falcon")),
//         (Length(0.5), Animal::new("Falcon")),
//         (Length(2.8), Animal::new("Tiger")),
//         (Length(2.9), Animal::new("Tiger")),
//         (Length(0.6), Animal::new("Stork")),
//         (Length(0.6), Animal::new("Crane")),
//         (Length(0.15), Animal::new("Kingfisher")),
//         (Length(0.9), Animal::new("Flamingo")),
//         (Length(0.9), Animal::new("Flamingo")),
//         (Length(0.8), Animal::new("Flamingo")),
//         (Length(0.8), Animal::new("Flamingo")),
//         (Length(0.25), Animal::new("Duck")),
//         (Length(0.32), Animal::new("Duck")),
//         (Length(0.3), Animal::new("Duck")),
//         (Length(0.3), Animal::new("Black Bear")),
//         (Length(1.2), Animal::new("Black Bear")),
//         (Length(2.2), Animal::new("Grizzly Bear")),
//         (Length(1.1), Animal::new("Mountain Goat")),
//         (Length(1.0), Animal::new("Mountain Goat")),
//         (Length(1.3), Animal::new("Mountain Goat")),
//     ]
// }
