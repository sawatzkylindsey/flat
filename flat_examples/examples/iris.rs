use blarg::{derive::*, prelude::*, CommandLineParser, Parameter, Scalar, Switch};
use flat::*;
use ordered_float::OrderedFloat;
use serde::Deserialize;
use std::ops::{Add, Deref, Div, Mul, Sub};

fn main() {
    let parameters = Parameters::blarg_parse();
    let json_data: Vec<FlowerJson> = serde_json::from_str(IRIS_JSON).unwrap();
    let mut builder = Dataset::builder(FlowerSchema);

    for flower in &json_data {
        builder.update(flower.into());
    }

    let dataset = builder.build();
    barchart_impl_view(&parameters, &dataset);
    histogram_impl_view(&parameters, &dataset);
}

fn barchart_impl_view(parameters: &Parameters, dataset: &Dataset<FlowerSchema>) {
    let view = AttributeView {
        data: dataset.data(),
        field: parameters.field.clone(),
    };
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

fn histogram_impl_view(parameters: &Parameters, dataset: &Dataset<FlowerSchema>) {
    let view = SepalLengthView {
        data: dataset.data(),
        field: parameters.field.clone(),
    };
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

struct FlowerSchema;

impl Schema for FlowerSchema {
    type Dimensions = Flower;
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

struct AttributeView<'a> {
    data: &'a [<FlowerSchema as Schema>::Dimensions],
    field: AttributeField,
}

impl<'a> View<FlowerSchema> for AttributeView<'a> {
    type PrimaryDimension = Species;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = (Species,);

    fn data(&self) -> &[<FlowerSchema as Schema>::Dimensions] {
        &self.data
    }

    fn value(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> f64 {
        dims.value(&self.field)
    }

    fn value_label(&self) -> String {
        self.field.print_string()
    }

    fn primary_dim(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.species.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<FlowerSchema as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing
    }

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::DisplayDimensions {
        (dims.species.clone(),)
    }

    fn display_headers(&self) -> Vec<String> {
        vec!["Species".to_string()]
    }
}

struct SepalLengthView<'a> {
    data: &'a [<FlowerSchema as Schema>::Dimensions],
    field: AttributeField,
}

impl<'a> View<FlowerSchema> for SepalLengthView<'a> {
    type PrimaryDimension = SepalLength;
    type BreakdownDimension = Nothing;
    type DisplayDimensions = (SepalLength,);

    fn data(&self) -> &[<FlowerSchema as Schema>::Dimensions] {
        &self.data
    }

    fn value(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> f64 {
        dims.value(&self.field)
    }

    fn value_label(&self) -> String {
        self.field.print_string()
    }

    fn primary_dim(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::PrimaryDimension {
        dims.sepal_length.clone()
    }

    fn breakdown_dim(
        &self,
        _dims: &<FlowerSchema as Schema>::Dimensions,
    ) -> Self::BreakdownDimension {
        Nothing
    }

    fn breakdown_label(&self) -> Option<String> {
        None
    }

    fn display_dims(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::DisplayDimensions {
        (dims.sepal_length.clone(),)
    }

    fn display_headers(&self) -> Vec<String> {
        vec!["Sepal Length".to_string()]
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

impl Dimensions for Species {
    fn as_strings(&self) -> Vec<String> {
        vec![self.to_string()]
    }

    fn len(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SepalLength(OrderedFloat<f64>);

impl Deref for SepalLength {
    type Target = OrderedFloat<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for SepalLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

impl Add for SepalLength {
    type Output = SepalLength;

    fn add(self, rhs: Self) -> Self::Output {
        SepalLength(self.0.add(rhs.0))
    }
}

impl Sub for SepalLength {
    type Output = SepalLength;

    fn sub(self, rhs: Self) -> Self::Output {
        SepalLength(self.0.sub(rhs.0))
    }
}

impl Binnable for SepalLength {
    fn multiply(&self, rhs: usize) -> Self {
        SepalLength(OrderedFloat(self.0 .0.mul(rhs as f64)))
    }

    fn divide(&self, rhs: usize) -> Self {
        SepalLength(OrderedFloat(self.0 .0.div(rhs as f64)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SepalWidth(OrderedFloat<f64>);

impl Deref for SepalWidth {
    type Target = OrderedFloat<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for SepalWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PetalLength(OrderedFloat<f64>);

impl Deref for PetalLength {
    type Target = OrderedFloat<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for PetalLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PetalWidth(OrderedFloat<f64>);

impl Deref for PetalWidth {
    type Target = OrderedFloat<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for PetalWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

#[derive(Debug, Clone)]
struct Flower {
    species: Species,
    sepal_length: SepalLength,
    sepal_width: SepalWidth,
    petal_length: PetalLength,
    petal_width: PetalWidth,
}

impl From<&FlowerJson> for Flower {
    fn from(value: &FlowerJson) -> Self {
        Self {
            species: Species(value.species.clone()),
            sepal_length: SepalLength(OrderedFloat(value.sepal_length)),
            sepal_width: SepalWidth(OrderedFloat(value.sepal_width)),
            petal_length: PetalLength(OrderedFloat(value.petal_length)),
            petal_width: PetalWidth(OrderedFloat(value.petal_width)),
        }
    }
}

impl Dimensions for Flower {
    fn as_strings(&self) -> Vec<String> {
        vec![
            self.species.to_string(),
            self.sepal_length.to_string(),
            self.sepal_width.to_string(),
            self.petal_length.to_string(),
            self.petal_width.to_string(),
        ]
    }

    fn len(&self) -> usize {
        5
    }
}

impl Flower {
    fn value(&self, field: &AttributeField) -> f64 {
        match field {
            AttributeField::SepalLength => self.sepal_length.0 .0,
            AttributeField::SepalWidth => self.sepal_width.0 .0,
            AttributeField::PetalLength => self.petal_length.0 .0,
            AttributeField::PetalWidth => self.petal_width.0 .0,
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
