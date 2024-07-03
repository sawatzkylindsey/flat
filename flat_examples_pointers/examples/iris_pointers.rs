use blarg::{derive::*, prelude::*, CommandLineParser, Parameter, Scalar, Switch};
use flat::*;
use ordered_float::OrderedFloat;
use serde::Deserialize;
use std::ops::{Add, Deref, Div, Mul, Sub};

fn main() {
    let parameters = Parameters::blarg_parse();
    let json_data: Vec<FlowerJson> = serde_json::from_str(IRIS_JSON).unwrap();
    let mut builder = Dataset::builder(Schemas::two("Species", parameters.field.print_string()));

    for flower in &json_data {
        let flower: Flower = flower.into();
        let attribute = match parameters.field {
            AttributeField::SepalLength => flower.sepal_length,
            AttributeField::SepalWidth => flower.sepal_width,
            AttributeField::PetalLength => flower.petal_length,
            AttributeField::PetalWidth => flower.petal_width,
        };
        builder.update((flower.species, attribute));
    }

    let dataset = builder.build();
    barchart_impl_view(&parameters, &dataset);

    let mut builder = Dataset::builder(Schemas::one(parameters.field.print_string()));

    for flower in &json_data {
        let flower: Flower = flower.into();
        let attribute = match parameters.field {
            AttributeField::SepalLength => flower.sepal_length,
            AttributeField::SepalWidth => flower.sepal_width,
            AttributeField::PetalLength => flower.petal_length,
            AttributeField::PetalWidth => flower.petal_width,
        };
        builder.update((attribute,));
    }

    let dataset = builder.build();
    histogram_impl_view(&parameters, &dataset);
}

fn barchart_impl_view(
    parameters: &Parameters,
    dataset: &Dataset<Schema2<Species, AttributeContainer>>,
) {
    let view = dataset.view_2nd();
    let flat = BarChart::new(&view).render(Render {
        aggregate: Aggregate::Average,
        show_aggregate: parameters.verbose,
        widget_config: {
            BarChartConfig {
                show_aggregate: parameters.verbose,
                ..BarChartConfig::default()
            }
        },
        ..Render::default()
    });
    println!(
        "Shows the '{}' of flowers based off their species.",
        parameters.field
    );
    println!("Produced via custom implementation of a `flat::View`.");
    println!();
    println!("{flat}");
    println!();
}

fn histogram_impl_view(parameters: &Parameters, dataset: &Dataset<Schema1<AttributeContainer>>) {
    let view = dataset.reflect_1st();
    let flat = Histogram::new(&view, 10).render(Render {
        aggregate: Aggregate::Sum,
        show_aggregate: parameters.verbose,
        widget_config: { HistogramConfig::default() },
        ..Render::default()
    });
    println!(
        "Shows the '{}' of flowers histogram-ed by their sepal length.",
        parameters.field
    );
    println!("Produced via custom implementation of a `flat::View`.");
    println!();
    println!("{flat}");
    println!();
}

#[derive(Default, BlargParser)]
struct Parameters {
    #[blarg(short = 'v')]
    verbose: bool,
    #[blarg(choices)]
    field: AttributeField,
}

#[derive(Debug, Clone, BlargChoices, PartialEq)]
enum AttributeField {
    #[blarg(help = "View the Sepal Length attribute.")]
    SepalLength,
    #[blarg(help = "View the Sepal Width attribute.")]
    SepalWidth,
    #[blarg(help = "View the Petal Length attribute.")]
    PetalLength,
    #[blarg(help = "View the Petal Width attribute.")]
    PetalWidth,
}

impl Default for AttributeField {
    fn default() -> Self {
        AttributeField::SepalLength
    }
}

impl std::fmt::Display for AttributeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeField::SepalLength => write!(f, "SepalLength"),
            AttributeField::SepalWidth => write!(f, "SepalWidth"),
            AttributeField::PetalLength => write!(f, "PetalLength"),
            AttributeField::PetalWidth => write!(f, "PetalWidth"),
        }
    }
}

impl std::str::FromStr for AttributeField {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "SepalLength" => Ok(AttributeField::SepalLength),
            "SepalWidth" => Ok(AttributeField::SepalWidth),
            "PetalLength" => Ok(AttributeField::PetalLength),
            "PetalWidth" => Ok(AttributeField::PetalWidth),
            _ => Err(format!("unknown: {}", value)),
        }
    }
}

impl AttributeField {
    fn print_string(&self) -> String {
        match &self {
            AttributeField::SepalLength => "Sepal Length".to_string(),
            AttributeField::SepalWidth => "Sepal Width".to_string(),
            AttributeField::PetalLength => "Petal Length".to_string(),
            AttributeField::PetalWidth => "Petal Width".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct FlowerJson {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    species: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Species(String);

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct AttributeContainer(OrderedFloat<f64>);

impl Deref for AttributeContainer {
    type Target = OrderedFloat<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for AttributeContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

impl Add for AttributeContainer {
    type Output = AttributeContainer;

    fn add(self, rhs: Self) -> Self::Output {
        AttributeContainer(self.0.add(rhs.0))
    }
}

impl Sub for AttributeContainer {
    type Output = AttributeContainer;

    fn sub(self, rhs: Self) -> Self::Output {
        AttributeContainer(self.0.sub(rhs.0))
    }
}

impl Binnable for AttributeContainer {
    fn multiply(&self, rhs: usize) -> Self {
        AttributeContainer(OrderedFloat(self.0 .0.mul(rhs as f64)))
    }

    fn divide(&self, rhs: usize) -> Self {
        AttributeContainer(OrderedFloat(self.0 .0.div(rhs as f64)))
    }
}

#[derive(Debug, Clone)]
struct Flower {
    species: Species,
    sepal_length: AttributeContainer,
    sepal_width: AttributeContainer,
    petal_length: AttributeContainer,
    petal_width: AttributeContainer,
}

impl From<&FlowerJson> for Flower {
    fn from(value: &FlowerJson) -> Self {
        Self {
            species: Species(value.species.clone()),
            sepal_length: AttributeContainer(OrderedFloat(value.sepal_length)),
            sepal_width: AttributeContainer(OrderedFloat(value.sepal_width)),
            petal_length: AttributeContainer(OrderedFloat(value.petal_length)),
            petal_width: AttributeContainer(OrderedFloat(value.petal_width)),
        }
    }
}

const IRIS_JSON: &str = r#"[
    {
        "sepal_length": 5.1,
        "sepal_width": 3.5,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.9,
        "sepal_width": 3.0,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.7,
        "sepal_width": 3.2,
        "petal_length": 1.3,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.6,
        "sepal_width": 3.1,
        "petal_length": 1.5,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.6,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.4,
        "sepal_width": 3.9,
        "petal_length": 1.7,
        "petal_width": 0.4,
        "species": "setosa"
    },
    {
        "sepal_length": 4.6,
        "sepal_width": 3.4,
        "petal_length": 1.4,
        "petal_width": 0.3,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.4,
        "petal_length": 1.5,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.4,
        "sepal_width": 2.9,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.9,
        "sepal_width": 3.1,
        "petal_length": 1.5,
        "petal_width": 0.1,
        "species": "setosa"
    },
    {
        "sepal_length": 5.4,
        "sepal_width": 3.7,
        "petal_length": 1.5,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.8,
        "sepal_width": 3.4,
        "petal_length": 1.6,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.8,
        "sepal_width": 3.0,
        "petal_length": 1.4,
        "petal_width": 0.1,
        "species": "setosa"
    },
    {
        "sepal_length": 4.3,
        "sepal_width": 3.0,
        "petal_length": 1.1,
        "petal_width": 0.1,
        "species": "setosa"
    },
    {
        "sepal_length": 5.8,
        "sepal_width": 4.0,
        "petal_length": 1.2,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 4.4,
        "petal_length": 1.5,
        "petal_width": 0.4,
        "species": "setosa"
    },
    {
        "sepal_length": 5.4,
        "sepal_width": 3.9,
        "petal_length": 1.3,
        "petal_width": 0.4,
        "species": "setosa"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 3.5,
        "petal_length": 1.4,
        "petal_width": 0.3,
        "species": "setosa"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 3.8,
        "petal_length": 1.7,
        "petal_width": 0.3,
        "species": "setosa"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 3.8,
        "petal_length": 1.5,
        "petal_width": 0.3,
        "species": "setosa"
    },
    {
        "sepal_length": 5.4,
        "sepal_width": 3.4,
        "petal_length": 1.7,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 3.7,
        "petal_length": 1.5,
        "petal_width": 0.4,
        "species": "setosa"
    },
    {
        "sepal_length": 4.6,
        "sepal_width": 3.6,
        "petal_length": 1.0,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 3.3,
        "petal_length": 1.7,
        "petal_width": 0.5,
        "species": "setosa"
    },
    {
        "sepal_length": 4.8,
        "sepal_width": 3.4,
        "petal_length": 1.9,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.0,
        "petal_length": 1.6,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.4,
        "petal_length": 1.6,
        "petal_width": 0.4,
        "species": "setosa"
    },
    {
        "sepal_length": 5.2,
        "sepal_width": 3.5,
        "petal_length": 1.5,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.2,
        "sepal_width": 3.4,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.7,
        "sepal_width": 3.2,
        "petal_length": 1.6,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.8,
        "sepal_width": 3.1,
        "petal_length": 1.6,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.4,
        "sepal_width": 3.4,
        "petal_length": 1.5,
        "petal_width": 0.4,
        "species": "setosa"
    },
    {
        "sepal_length": 5.2,
        "sepal_width": 4.1,
        "petal_length": 1.5,
        "petal_width": 0.1,
        "species": "setosa"
    },
    {
        "sepal_length": 5.5,
        "sepal_width": 4.2,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.9,
        "sepal_width": 3.1,
        "petal_length": 1.5,
        "petal_width": 0.1,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.2,
        "petal_length": 1.2,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.5,
        "sepal_width": 3.5,
        "petal_length": 1.3,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.9,
        "sepal_width": 3.1,
        "petal_length": 1.5,
        "petal_width": 0.1,
        "species": "setosa"
    },
    {
        "sepal_length": 4.4,
        "sepal_width": 3.0,
        "petal_length": 1.3,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 3.4,
        "petal_length": 1.5,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.5,
        "petal_length": 1.3,
        "petal_width": 0.3,
        "species": "setosa"
    },
    {
        "sepal_length": 4.5,
        "sepal_width": 2.3,
        "petal_length": 1.3,
        "petal_width": 0.3,
        "species": "setosa"
    },
    {
        "sepal_length": 4.4,
        "sepal_width": 3.2,
        "petal_length": 1.3,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.5,
        "petal_length": 1.6,
        "petal_width": 0.6,
        "species": "setosa"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 3.8,
        "petal_length": 1.9,
        "petal_width": 0.4,
        "species": "setosa"
    },
    {
        "sepal_length": 4.8,
        "sepal_width": 3.0,
        "petal_length": 1.4,
        "petal_width": 0.3,
        "species": "setosa"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 3.8,
        "petal_length": 1.6,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 4.6,
        "sepal_width": 3.2,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.3,
        "sepal_width": 3.7,
        "petal_length": 1.5,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 3.3,
        "petal_length": 1.4,
        "petal_width": 0.2,
        "species": "setosa"
    },
    {
        "sepal_length": 7.0,
        "sepal_width": 3.2,
        "petal_length": 4.7,
        "petal_width": 1.4,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.4,
        "sepal_width": 3.2,
        "petal_length": 4.5,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.9,
        "sepal_width": 3.1,
        "petal_length": 4.9,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.5,
        "sepal_width": 2.3,
        "petal_length": 4.0,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.5,
        "sepal_width": 2.8,
        "petal_length": 4.6,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 2.8,
        "petal_length": 4.5,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 3.3,
        "petal_length": 4.7,
        "petal_width": 1.6,
        "species": "versicolor"
    },
    {
        "sepal_length": 4.9,
        "sepal_width": 2.4,
        "petal_length": 3.3,
        "petal_width": 1.0,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.6,
        "sepal_width": 2.9,
        "petal_length": 4.6,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.2,
        "sepal_width": 2.7,
        "petal_length": 3.9,
        "petal_width": 1.4,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 2.0,
        "petal_length": 3.5,
        "petal_width": 1.0,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.9,
        "sepal_width": 3.0,
        "petal_length": 4.2,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.0,
        "sepal_width": 2.2,
        "petal_length": 4.0,
        "petal_width": 1.0,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.1,
        "sepal_width": 2.9,
        "petal_length": 4.7,
        "petal_width": 1.4,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.6,
        "sepal_width": 2.9,
        "petal_length": 3.6,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 3.1,
        "petal_length": 4.4,
        "petal_width": 1.4,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.6,
        "sepal_width": 3.0,
        "petal_length": 4.5,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.8,
        "sepal_width": 2.7,
        "petal_length": 4.1,
        "petal_width": 1.0,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.2,
        "sepal_width": 2.2,
        "petal_length": 4.5,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.6,
        "sepal_width": 2.5,
        "petal_length": 3.9,
        "petal_width": 1.1,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.9,
        "sepal_width": 3.2,
        "petal_length": 4.8,
        "petal_width": 1.8,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.1,
        "sepal_width": 2.8,
        "petal_length": 4.0,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 2.5,
        "petal_length": 4.9,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.1,
        "sepal_width": 2.8,
        "petal_length": 4.7,
        "petal_width": 1.2,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.4,
        "sepal_width": 2.9,
        "petal_length": 4.3,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.6,
        "sepal_width": 3.0,
        "petal_length": 4.4,
        "petal_width": 1.4,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.8,
        "sepal_width": 2.8,
        "petal_length": 4.8,
        "petal_width": 1.4,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 3.0,
        "petal_length": 5.0,
        "petal_width": 1.7,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.0,
        "sepal_width": 2.9,
        "petal_length": 4.5,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 2.6,
        "petal_length": 3.5,
        "petal_width": 1.0,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.5,
        "sepal_width": 2.4,
        "petal_length": 3.8,
        "petal_width": 1.1,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.5,
        "sepal_width": 2.4,
        "petal_length": 3.7,
        "petal_width": 1.0,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.8,
        "sepal_width": 2.7,
        "petal_length": 3.9,
        "petal_width": 1.2,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.0,
        "sepal_width": 2.7,
        "petal_length": 5.1,
        "petal_width": 1.6,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.4,
        "sepal_width": 3.0,
        "petal_length": 4.5,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.0,
        "sepal_width": 3.4,
        "petal_length": 4.5,
        "petal_width": 1.6,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 3.1,
        "petal_length": 4.7,
        "petal_width": 1.5,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 2.3,
        "petal_length": 4.4,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.6,
        "sepal_width": 3.0,
        "petal_length": 4.1,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.5,
        "sepal_width": 2.5,
        "petal_length": 4.0,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.5,
        "sepal_width": 2.6,
        "petal_length": 4.4,
        "petal_width": 1.2,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.1,
        "sepal_width": 3.0,
        "petal_length": 4.6,
        "petal_width": 1.4,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.8,
        "sepal_width": 2.6,
        "petal_length": 4.0,
        "petal_width": 1.2,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.0,
        "sepal_width": 2.3,
        "petal_length": 3.3,
        "petal_width": 1.0,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.6,
        "sepal_width": 2.7,
        "petal_length": 4.2,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 3.0,
        "petal_length": 4.2,
        "petal_width": 1.2,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 2.9,
        "petal_length": 4.2,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.2,
        "sepal_width": 2.9,
        "petal_length": 4.3,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.1,
        "sepal_width": 2.5,
        "petal_length": 3.0,
        "petal_width": 1.1,
        "species": "versicolor"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 2.8,
        "petal_length": 4.1,
        "petal_width": 1.3,
        "species": "versicolor"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 3.3,
        "petal_length": 6.0,
        "petal_width": 2.5,
        "species": "virginica"
    },
    {
        "sepal_length": 5.8,
        "sepal_width": 2.7,
        "petal_length": 5.1,
        "petal_width": 1.9,
        "species": "virginica"
    },
    {
        "sepal_length": 7.1,
        "sepal_width": 3.0,
        "petal_length": 5.9,
        "petal_width": 2.1,
        "species": "virginica"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 2.9,
        "petal_length": 5.6,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.5,
        "sepal_width": 3.0,
        "petal_length": 5.8,
        "petal_width": 2.2,
        "species": "virginica"
    },
    {
        "sepal_length": 7.6,
        "sepal_width": 3.0,
        "petal_length": 6.6,
        "petal_width": 2.1,
        "species": "virginica"
    },
    {
        "sepal_length": 4.9,
        "sepal_width": 2.5,
        "petal_length": 4.5,
        "petal_width": 1.7,
        "species": "virginica"
    },
    {
        "sepal_length": 7.3,
        "sepal_width": 2.9,
        "petal_length": 6.3,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 2.5,
        "petal_length": 5.8,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 7.2,
        "sepal_width": 3.6,
        "petal_length": 6.1,
        "petal_width": 2.5,
        "species": "virginica"
    },
    {
        "sepal_length": 6.5,
        "sepal_width": 3.2,
        "petal_length": 5.1,
        "petal_width": 2.0,
        "species": "virginica"
    },
    {
        "sepal_length": 6.4,
        "sepal_width": 2.7,
        "petal_length": 5.3,
        "petal_width": 1.9,
        "species": "virginica"
    },
    {
        "sepal_length": 6.8,
        "sepal_width": 3.0,
        "petal_length": 5.5,
        "petal_width": 2.1,
        "species": "virginica"
    },
    {
        "sepal_length": 5.7,
        "sepal_width": 2.5,
        "petal_length": 5.0,
        "petal_width": 2.0,
        "species": "virginica"
    },
    {
        "sepal_length": 5.8,
        "sepal_width": 2.8,
        "petal_length": 5.1,
        "petal_width": 2.4,
        "species": "virginica"
    },
    {
        "sepal_length": 6.4,
        "sepal_width": 3.2,
        "petal_length": 5.3,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 6.5,
        "sepal_width": 3.0,
        "petal_length": 5.5,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 7.7,
        "sepal_width": 3.8,
        "petal_length": 6.7,
        "petal_width": 2.2,
        "species": "virginica"
    },
    {
        "sepal_length": 7.7,
        "sepal_width": 2.6,
        "petal_length": 6.9,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 6.0,
        "sepal_width": 2.2,
        "petal_length": 5.0,
        "petal_width": 1.5,
        "species": "virginica"
    },
    {
        "sepal_length": 6.9,
        "sepal_width": 3.2,
        "petal_length": 5.7,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 5.6,
        "sepal_width": 2.8,
        "petal_length": 4.9,
        "petal_width": 2.0,
        "species": "virginica"
    },
    {
        "sepal_length": 7.7,
        "sepal_width": 2.8,
        "petal_length": 6.7,
        "petal_width": 2.0,
        "species": "virginica"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 2.7,
        "petal_length": 4.9,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 3.3,
        "petal_length": 5.7,
        "petal_width": 2.1,
        "species": "virginica"
    },
    {
        "sepal_length": 7.2,
        "sepal_width": 3.2,
        "petal_length": 6.0,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.2,
        "sepal_width": 2.8,
        "petal_length": 4.8,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.1,
        "sepal_width": 3.0,
        "petal_length": 4.9,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.4,
        "sepal_width": 2.8,
        "petal_length": 5.6,
        "petal_width": 2.1,
        "species": "virginica"
    },
    {
        "sepal_length": 7.2,
        "sepal_width": 3.0,
        "petal_length": 5.8,
        "petal_width": 1.6,
        "species": "virginica"
    },
    {
        "sepal_length": 7.4,
        "sepal_width": 2.8,
        "petal_length": 6.1,
        "petal_width": 1.9,
        "species": "virginica"
    },
    {
        "sepal_length": 7.9,
        "sepal_width": 3.8,
        "petal_length": 6.4,
        "petal_width": 2.0,
        "species": "virginica"
    },
    {
        "sepal_length": 6.4,
        "sepal_width": 2.8,
        "petal_length": 5.6,
        "petal_width": 2.2,
        "species": "virginica"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 2.8,
        "petal_length": 5.1,
        "petal_width": 1.5,
        "species": "virginica"
    },
    {
        "sepal_length": 6.1,
        "sepal_width": 2.6,
        "petal_length": 5.6,
        "petal_width": 1.4,
        "species": "virginica"
    },
    {
        "sepal_length": 7.7,
        "sepal_width": 3.0,
        "petal_length": 6.1,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 3.4,
        "petal_length": 5.6,
        "petal_width": 2.4,
        "species": "virginica"
    },
    {
        "sepal_length": 6.4,
        "sepal_width": 3.1,
        "petal_length": 5.5,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.0,
        "sepal_width": 3.0,
        "petal_length": 4.8,
        "petal_width": 1.8,
        "species": "virginica"
    },
    {
        "sepal_length": 6.9,
        "sepal_width": 3.1,
        "petal_length": 5.4,
        "petal_width": 2.1,
        "species": "virginica"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 3.1,
        "petal_length": 5.6,
        "petal_width": 2.4,
        "species": "virginica"
    },
    {
        "sepal_length": 6.9,
        "sepal_width": 3.1,
        "petal_length": 5.1,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 5.8,
        "sepal_width": 2.7,
        "petal_length": 5.1,
        "petal_width": 1.9,
        "species": "virginica"
    },
    {
        "sepal_length": 6.8,
        "sepal_width": 3.2,
        "petal_length": 5.9,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 3.3,
        "petal_length": 5.7,
        "petal_width": 2.5,
        "species": "virginica"
    },
    {
        "sepal_length": 6.7,
        "sepal_width": 3.0,
        "petal_length": 5.2,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 6.3,
        "sepal_width": 2.5,
        "petal_length": 5.0,
        "petal_width": 1.9,
        "species": "virginica"
    },
    {
        "sepal_length": 6.5,
        "sepal_width": 3.0,
        "petal_length": 5.2,
        "petal_width": 2.0,
        "species": "virginica"
    },
    {
        "sepal_length": 6.2,
        "sepal_width": 3.4,
        "petal_length": 5.4,
        "petal_width": 2.3,
        "species": "virginica"
    },
    {
        "sepal_length": 5.9,
        "sepal_width": 3.0,
        "petal_length": 5.1,
        "petal_width": 1.8,
        "species": "virginica"
    }
]"#;
