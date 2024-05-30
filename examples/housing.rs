use blarg::{derive::*, CommandLineParser, Parameter, Switch};
use flat::{BarChart, BarChartConfig, Render, Schema};

fn main() {
    let parameters = Parameters::blarg_parse();
    let schema = Schema::three("City", "Quadrant", "Green Rating").breakdown_3rd();
    let mut builder = BarChart::builder(schema);

    for house in generate_dataset() {
        builder.update((house.0, house.1, house.2), 1);
    }

    let flat = builder.render(Render {
        show_aggregate: parameters.verbose,
        widget_config: BarChartConfig {
            show_aggregate: parameters.verbose,
            ..BarChartConfig::default()
        },
        ..Render::default()
    });
    println!("{flat}");
}

#[derive(Default, BlargParser)]
struct Parameters {
    #[blarg(short = 'v')]
    verbose: bool,
}

struct House(City, Quadrant, GreenRating);

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum City {
    Quebec,
    Vancouver,
}

impl std::fmt::Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            City::Quebec => write!(f, "Quebec"),
            City::Vancouver => write!(f, "Vancouver"),
        }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum GreenRating {
    Bad,
    Good,
    Excellent,
}

impl std::fmt::Display for GreenRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreenRating::Bad => write!(f, "Bad"),
            GreenRating::Good => write!(f, "Good"),
            GreenRating::Excellent => write!(f, "Excellent"),
        }
    }
}

// Some very fake data.
fn generate_dataset() -> Vec<House> {
    vec![
        House(City::Quebec, Quadrant::NorthEast, GreenRating::Bad),
        House(City::Quebec, Quadrant::NorthEast, GreenRating::Good),
        House(City::Quebec, Quadrant::NorthEast, GreenRating::Excellent),
        House(City::Quebec, Quadrant::NorthEast, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthEast, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthEast, GreenRating::Good),
        House(City::Quebec, Quadrant::SouthEast, GreenRating::Excellent),
        House(City::Quebec, Quadrant::SouthEast, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthEast, GreenRating::Good),
        House(City::Quebec, Quadrant::SouthEast, GreenRating::Excellent),
        House(City::Quebec, Quadrant::SouthEast, GreenRating::Good),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Good),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Excellent),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Good),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Excellent),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Good),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Excellent),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Good),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Excellent),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Bad),
        House(City::Quebec, Quadrant::SouthWest, GreenRating::Good),
        House(City::Vancouver, Quadrant::NorthEast, GreenRating::Excellent),
        House(City::Vancouver, Quadrant::NorthEast, GreenRating::Bad),
        House(City::Vancouver, Quadrant::NorthEast, GreenRating::Good),
        House(City::Vancouver, Quadrant::NorthEast, GreenRating::Excellent),
        House(City::Vancouver, Quadrant::NorthEast, GreenRating::Bad),
        House(City::Vancouver, Quadrant::NorthEast, GreenRating::Good),
        House(City::Vancouver, Quadrant::NorthWest, GreenRating::Excellent),
        House(City::Vancouver, Quadrant::NorthWest, GreenRating::Bad),
        House(City::Vancouver, Quadrant::NorthWest, GreenRating::Good),
        House(City::Vancouver, Quadrant::NorthWest, GreenRating::Excellent),
        House(City::Vancouver, Quadrant::SouthWest, GreenRating::Bad),
        House(City::Vancouver, Quadrant::SouthWest, GreenRating::Good),
        House(City::Vancouver, Quadrant::SouthWest, GreenRating::Excellent),
    ]
}
