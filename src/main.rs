mod density;
mod utils;
use rust_decimal::prelude::*;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Proximity")
    .version("0.1.1")
    .author("Nate D.")
    .about("Proximity score based on desirable and undesirables.")
    .arg(Arg::with_name("FILE")
        .help("Path to CSV. If not supplied, will look to STDIN.")
        .required(false)
        .index(1))
    .arg(Arg::with_name("X")
        .short("x")
        .long("x")
        .value_name("x")
        .help("Name of x value in category column.")
        .takes_value(true))
    .arg(Arg::with_name("Y")
        .short("y")
        .long("y")
        .value_name("y")
        .help("Name of y value in category column.")
        .takes_value(true))
    .arg(Arg::with_name("GRID-SIZE")
        .short("g")
        .long("grid-size")
        .value_name("GRID-SIZE")
        .help("Set binning size (km) in circle area. Default is 1km.")
        .takes_value(true))
    .arg(Arg::with_name("RADIUS")
        .short("r")
        .long("radius")
        .value_name("RADIUS")
        .help("Set circle radius (km) for circle area. Default is 5km.")
        .takes_value(true))
    .arg(Arg::with_name("N")
        .short("n")
        .long("n")
        .value_name("N")
        .help("Top N records to return. If not supplied, will return all.")
        .takes_value(true))
    .get_matches();

    let grid_size: Decimal =
        Decimal::from_str(matches.value_of("GRID-SIZE").unwrap_or("1")).unwrap();
    let radius: Decimal = 
        Decimal::from_str(matches.value_of("RADIUS").unwrap_or("5")).unwrap();
    let x : &str = matches.value_of("X").unwrap(); 
    let y : &str = matches.value_of("Y").unwrap(); 

    let mut points: Vec<density::Point> = Vec::new();
    if matches.is_present("FILE") {
        let file = matches.value_of("FILE").unwrap();
        points.append(&mut utils::csv_file_to_points(&file));
    } else {
        points.append(&mut utils::stdin_to_points());
    }

    let mut scores = density::score(points, &x, &y, radius, grid_size);
    scores.sort_by(|a, b| {
        b.score.cmp(&a.score)
    }); 

    if matches.is_present("N") {
        let n: usize = matches.value_of("N").unwrap().parse::<usize>().unwrap();
        utils::points_as_csv(scores.into_iter().take(n).collect::<Vec<density::Point>>()).expect("Fail");
    } else {
        utils::points_as_csv(scores).expect("Fail");
    }
}
