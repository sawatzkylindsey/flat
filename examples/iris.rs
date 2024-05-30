use blarg::{derive::*, prelude::*, CommandLineParser, Parameter, Scalar, Switch};
use flat::*;
use ordered_float::OrderedFloat;
use serde::Deserialize;

fn main() {
    let parameters = Parameters::blarg_parse();
    let dataset: Vec<Flower> = serde_json::from_str(IRIS_JSON).unwrap();

    match parameters.widget {
        Widget::BarChart => {
            if parameters.breakdown {
                bar_chart_breakdown(&parameters, dataset);
            } else {
                bar_chart(&parameters, dataset);
            }
        }
        Widget::Histogram => {
            if parameters.breakdown {
                histogram_breakdown(&parameters, dataset);
            } else {
                histogram(&parameters, dataset);
            }
        }
    }
}

fn bar_chart(parameters: &Parameters, dataset: Vec<Flower>) {
    let schema = Schema::one("Species").values("Petal Length");
    let mut builder = BarChart::builder(schema);

    for flower in dataset {
        builder.update((flower.species.clone(),), flower.petal_length);
    }

    let flat = builder.render(Render {
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

fn bar_chart_breakdown(parameters: &Parameters, dataset: Vec<Flower>) {
    let schema = Schema::two("Attribute", "Species").breakdown_2nd();
    let mut builder = BarChart::builder(schema);

    for flower in dataset {
        builder.update(
            ("sepal_length", flower.species.clone()),
            flower.sepal_length,
        );
        builder.update(("sepal_width", flower.species.clone()), flower.sepal_width);
        builder.update(
            ("petal_length", flower.species.clone()),
            flower.petal_length,
        );
        builder.update(("petal_width", flower.species.clone()), flower.petal_width);
    }

    let flat = builder.render(Render {
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
    println!("Shows the relative attribute values of flowers in the dataset, broken down by their species.");
    println!();
    println!("{flat}");
}

fn histogram(parameters: &Parameters, dataset: Vec<Flower>) {
    let schema = Schema::one("Petal Length").values("Flower Count");
    let mut builder = Histogram::builder(schema, 10);

    for flower in dataset {
        builder.update((flower.petal_length,), 1);
    }

    let flat = builder.render(Render {
        aggregate: Aggregate::Sum,
        show_aggregate: parameters.verbose,
        ..Render::default()
    });
    println!("Shows the relative count of flowers in the dataset, organized into bins based off their petal length.");
    println!();
    println!("{flat}");
}

fn histogram_breakdown(parameters: &Parameters, dataset: Vec<Flower>) {
    let schema = Schema::two("Petal Length", "Petal Widths").breakdown_2nd();
    let mut builder = Histogram::builder(schema, 10);

    for flower in dataset {
        builder.update((flower.petal_length, OrderedFloat(flower.petal_width)), 1);
    }

    let flat = builder.render(Render {
        aggregate: Aggregate::Sum,
        show_aggregate: parameters.verbose,
        ..Render::default()
    });
    println!("Shows the relative count of flowers in the dataset, broken down by their petal widths, organized into bins based off their petal length.");
    println!();
    println!("{flat}");
}

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
    #[blarg(choices)]
    widget: Widget,
    breakdown: bool,
}

#[derive(Debug, Deserialize)]
struct Flower {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    species: String,
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
