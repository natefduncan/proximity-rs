mod density;
mod utils;
use rust_decimal::prelude::*;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Proximity")
    .version("1.0")
    .author("Nate D.")
    .about("Proximity score based on desirable and undesirables.")
    .arg(Arg::with_name("POINT")
        .help("CSV path to points. Should include latitude, longitude, and name columns. If not supplied, will look in STDIN.")
        .required(false)
        .index(1))
    .arg(Arg::with_name("DESIRABLE")
         .short("d")
         .long("desirable")
         .value_name("DESIRABLE")
         .help("CSV path to desirables. Should include latitude, longitude, and name columns.")
         .takes_value(true))
    .arg(Arg::with_name("UNDESIRABLE")
        .short("u")
        .long("undesirable")
        .value_name("UNDESIRABLE")
        .help("CSV path to undesirables. Should include latitude, longitude, and name columns.")
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
        .long("N")
        .value_name("N")
        .help("Top N records to return. If not supplied, will return all.")
        .takes_value(true))
    .get_matches();

    let grid_size: Decimal =
        Decimal::from_str(matches.value_of("GRID-SIZE").unwrap_or("1")).unwrap();
    let radius: Decimal = Decimal::from_str(matches.value_of("RADIUS").unwrap_or("1")).unwrap();

    let mut points: Vec<density::Point> = Vec::new();
    if matches.is_present("POINT") {
        let file = matches.value_of("POINT").unwrap();
        points.append(&mut utils::csv_file_to_points(&file));
    } else {
        points.append(&mut utils::stdin_to_points());
    }

    let des_path = matches.value_of("DESIRABLE").unwrap();
    let mut des: Vec<density::Point> = utils::csv_file_to_points(des_path);

    let undes_path = matches.value_of("UNDESIRABLE").unwrap();
    let mut undes: Vec<density::Point> = utils::csv_file_to_points(undes_path);

    let mut scores = density::score(&points, &des, &undes, radius, grid_size);
    scores.sort_by(|a, b| {
        b.score
            .as_ref()
            .unwrap()
            .desirable
            .cmp(&a.score.as_ref().unwrap().desirable)
    });

    if matches.is_present("N") {
        let n: usize = matches.value_of("N").unwrap().parse::<usize>().unwrap();
        utils::score_as_csv(
            &mut scores.into_iter().take(n).collect::<Vec<density::Point>>(),
            &mut des,
            &mut undes,
        )
        .expect("Fail");
    } else {
        utils::score_as_csv(&mut scores, &mut des, &mut undes).expect("Fail");
    }
}
