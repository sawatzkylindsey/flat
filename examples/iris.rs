use blarg::{derive::*, prelude::*, CommandLineParser, Parameter, Switch};
use flat::*;
use ordered_float::OrderedFloat;
use serde::Deserialize;

fn main() {
    let parameters = Parameters::blarg_parse();
    let json_data: Vec<FlowerJson> = serde_json::from_str(IRIS_JSON).unwrap();
    let mut dataset = Dataset::builder(FlowerSchema);

    for flower in &json_data {
        dataset.update(flower.into());
    }

    bar_chart(&parameters, &dataset);

    // match parameters.widget {
    //     Widget::BarChart => {
    //         if parameters.breakdown {
    //             bar_chart_breakdown(&parameters, dataset);
    //         } else {
    //             bar_chart(&parameters, dataset);
    //         }
    //     }
    //     Widget::Histogram => {
    //         if parameters.breakdown {
    //             histogram_breakdown(&parameters, dataset);
    //         } else {
    //             histogram(&parameters, dataset);
    //         }
    //     }
    // }
}

fn bar_chart(parameters: &Parameters, dataset: &Dataset<FlowerSchema>) {
    // dataset.cast(Schemas::two("Species", "Sepal Length"), |flower| (flower.species.clone(), flower.sepal_length.clone()));
    // let ds: &Dataset<Schema2<Species, SepalLength>> = dataset.downcast_2d();
    // let view = View2::new(
    //     ds,
    //     Box::new(|(a, b)| *b.0),
    //     // Box::new(|flower: &Flower| flower.sepal_length.0 .0),
    // );
    // let view = SepalViewX { dataset };
    // let view = FlowerView2D {
    //     dataset,
    //     extractor: Box::new(|flower| flower.sepal_length.0 .0),
    //     t: Box::new(|flower| flower.species.0),
    //     u: Box::new(|flower| flower.sepal_width.0 .0),
    // };
    let ds = dataset.recast(Schemas::two("Species", "Sepal Length"), |flower| {
        (flower.species.clone(), flower.sepal_length.0 .0)
    });
    let view = ds.view_2nd();
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
    println!("Shows the relative lengths of flowers in the dataset, based off their species.");
    println!();
    println!("{flat}");
}
//
// fn bar_chart_breakdown(parameters: &Parameters, dataset: Vec<Flower>) {
//     let schema = Schemas::two("Attribute", "Species", "moot");
//     let mut builder = Dataset::builder(schema);
//
//     for flower in dataset {
//         builder.update(
//             ("sepal_length", flower.species.clone()),
//             flower.sepal_length,
//         );
//         builder.update(("sepal_width", flower.species.clone()), flower.sepal_width);
//         builder.update(
//             ("petal_length", flower.species.clone()),
//             flower.petal_length,
//         );
//         builder.update(("petal_width", flower.species.clone()), flower.petal_width);
//     }
//
//     let view = builder.view_breakdown2();
//     let flat = BarChart::new(&view).render(Render {
//         aggregate: Aggregate::Average,
//         show_aggregate: parameters.verbose,
//         widget_config: {
//             BarChartConfig {
//                 show_aggregate: parameters.verbose,
//                 ..BarChartConfig::default()
//             }
//         },
//         ..Render::default()
//     });
//     println!("Shows the relative attribute values of flowers in the dataset, broken down by their species.");
//     println!();
//     println!("{flat}");
// }
//
// fn histogram(parameters: &Parameters, dataset: Vec<Flower>) {
//     let schema = Schemas::one("Petal Length", "Flower Count");
//     let mut builder = Dataset::builder(schema);
//
//     for flower in dataset {
//         builder.update((flower.petal_length,), 1);
//     }
//
//     let view = builder.view();
//     let flat = Histogram::new(&view, 10).render(Render {
//         aggregate: Aggregate::Sum,
//         show_aggregate: parameters.verbose,
//         ..Render::default()
//     });
//     println!("Shows the relative count of flowers in the dataset, organized into bins based off their petal length.");
//     println!();
//     println!("{flat}");
// }
//
// fn histogram_breakdown(parameters: &Parameters, dataset: Vec<Flower>) {
//     let schema = Schemas::two("Petal Length", "Petal Widths", "moot");
//     let mut builder = Dataset::builder(schema);
//
//     for flower in dataset {
//         builder.update((flower.petal_length, OrderedFloat(flower.petal_width)), 1);
//     }
//
//     let view = builder.view_breakdown2();
//     let flat = Histogram::new(&view, 10).render(Render {
//         aggregate: Aggregate::Sum,
//         show_aggregate: parameters.verbose,
//         ..Render::default()
//     });
//     println!("Shows the relative count of flowers in the dataset, broken down by their petal widths, organized into bins based off their petal length.");
//     println!();
//     println!("{flat}");
// }

#[derive(BlargChoices, PartialEq)]
enum Widget {
    #[blarg(help = "Use the BarChart widget.")]
    BarChart,
    #[blarg(help = "Use the Histogram widget.")]
    Histogram,
}

impl Default for Widget {
    fn default() -> Self {
        Widget::BarChart
    }
}

impl std::fmt::Display for Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Widget::BarChart => write!(f, "barchart"),
            Widget::Histogram => write!(f, "histogram"),
        }
    }
}

impl std::str::FromStr for Widget {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "barchart" => Ok(Widget::BarChart),
            "histogram" => Ok(Widget::Histogram),
            _ => Err(format!("unknown: {}", value)),
        }
    }
}

#[derive(Default, BlargParser)]
struct Parameters {
    #[blarg(short = 'v')]
    verbose: bool,
    // #[blarg(choices)]
    // widget: Widget,
    // breakdown: bool,
}

struct FlowerSchema;

impl Schema for FlowerSchema {
    type Dimensions = Flower;
}

// struct SpeciesView<'a> {
//     dataset: &'a Dataset<FlowerSchema>,
// }
//
// impl<'a> View<FlowerSchema> for SpeciesView<'a> {
//     type Dimensions = Flower;
//     type PrimaryDimension = Species;
//     type BreakdownDimension = Nothing;
//     type SortDimensions = Species;
//
//     fn data(&self) -> &Dataset<FlowerSchema> {
//         &self.dataset
//     }
//
//     fn value(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> f64 {
//         1f64
//     }
//
//     fn primary_dim(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::PrimaryDimension {
//         dims.species.clone()
//     }
//
//     fn breakdown_dim(
//         &self,
//         dims: &<FlowerSchema as Schema>::Dimensions,
//     ) -> Self::BreakdownDimension {
//         Nothing
//     }
//
//     fn sort_dims(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::SortDimensions {
//         dims.species.clone()
//     }
//
//     fn headers(&self) -> Vec<String> {
//         vec!["Species".to_string()]
//     }
//
//     fn value_header(&self) -> String {
//         "dooo".to_string()
//     }
//
//     fn is_breakdown(&self) -> bool {
//         false
//     }
// }
//
// struct SepalView<'a> {
//     dataset: &'a Dataset<FlowerSchema>,
// }
//
// impl<'a> View<FlowerSchema> for SepalView<'a> {
//     type Dimensions = Flower;
//     type PrimaryDimension = Species;
//     type BreakdownDimension = Nothing;
//     type SortDimensions = (Species, SepalWidth);
//
//     fn data(&self) -> &Dataset<FlowerSchema> {
//         &self.dataset
//     }
//
//     fn value(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> f64 {
//         dims.sepal_length.0 .0
//     }
//
//     fn primary_dim(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::PrimaryDimension {
//         dims.species.clone()
//         // (dims.sepal_length.clone(), dims.sepal_width.clone())
//     }
//
//     fn breakdown_dim(
//         &self,
//         dims: &<FlowerSchema as Schema>::Dimensions,
//     ) -> Self::BreakdownDimension {
//         Nothing
//     }
//
//     fn sort_dims(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::SortDimensions {
//         (dims.species.clone(), dims.sepal_width.clone())
//     }
//
//     fn headers(&self) -> Vec<String> {
//         vec!["Species".to_string(), "Sepal Width".to_string()]
//     }
//
//     fn value_header(&self) -> String {
//         "Sepal Length".to_string()
//     }
//
//     fn is_breakdown(&self) -> bool {
//         false
//     }
// }
//
// struct SepalViewX<'a> {
//     dataset: &'a Dataset<FlowerSchema>,
// }
//
// impl<'a> View<FlowerSchema> for SepalViewX<'a> {
//     type Dimensions = Flower;
//     type PrimaryDimension = Species;
//     type BreakdownDimension = Nothing;
//     type SortDimensions = Species;
//
//     fn data(&self) -> &Dataset<FlowerSchema> {
//         &self.dataset
//     }
//
//     fn value(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> f64 {
//         dims.sepal_length.0 .0
//     }
//
//     fn primary_dim(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::PrimaryDimension {
//         dims.species.clone()
//         // (dims.sepal_length.clone(), dims.sepal_width.clone())
//     }
//
//     fn breakdown_dim(
//         &self,
//         dims: &<FlowerSchema as Schema>::Dimensions,
//     ) -> Self::BreakdownDimension {
//         Nothing
//     }
//
//     fn sort_dims(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::SortDimensions {
//         dims.species.clone()
//     }
//
//     fn headers(&self) -> Vec<String> {
//         vec!["Species".to_string()]
//     }
//
//     fn value_header(&self) -> String {
//         "Sepal Length".to_string()
//     }
//
//     fn is_breakdown(&self) -> bool {
//         false
//     }
// }
//
// struct FlowerView2D<'a, T, U> {
//     dataset: &'a Dataset<FlowerSchema>,
//     extractor: Box<dyn Fn(&Flower) -> f64>,
//     t: Box<dyn Fn(&Flower) -> T>,
//     u: Box<dyn Fn(&Flower) -> U>,
// }
//
// impl<'a, T, U> View<FlowerSchema> for FlowerView2D<'a, T, U> {
//     type Dimensions = Flower;
//     type PrimaryDimension = T;
//     type BreakdownDimension = Nothing;
//     type SortDimensions = (T, U);
//
//     fn data(&self) -> &Dataset<FlowerSchema> {
//         &self.dataset
//     }
//
//     fn value(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> f64 {
//         (self.extractor)(dims)
//     }
//
//     fn primary_dim(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::PrimaryDimension {
//         (self.t)(dims)
//         // dims.species.clone()
//         // (dims.sepal_length.clone(), dims.sepal_width.clone())
//     }
//
//     fn breakdown_dim(
//         &self,
//         dims: &<FlowerSchema as Schema>::Dimensions,
//     ) -> Self::BreakdownDimension {
//         Nothing
//     }
//
//     fn sort_dims(&self, dims: &<FlowerSchema as Schema>::Dimensions) -> Self::SortDimensions {
//         ((self.t)(dims), (self.u)(dims))
//     }
//
//     fn headers(&self) -> Vec<String> {
//         vec!["t".to_string(), "u".to_string()]
//     }
//
//     fn value_header(&self) -> String {
//         "xyz".to_string()
//     }
//
//     fn is_breakdown(&self) -> bool {
//         false
//     }
// }

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

impl std::fmt::Display for SepalLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SepalWidth(OrderedFloat<f64>);

impl std::fmt::Display for SepalWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PetalLength(OrderedFloat<f64>);

impl std::fmt::Display for PetalLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", minimal_precision_string(self.0 .0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PetalWidth(OrderedFloat<f64>);

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
