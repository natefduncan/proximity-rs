mod density;
mod utils;

use rust_decimal::prelude::*;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Proximity")
    .version("0.1.3")
    .author("Nate D.")
    .about("Proximity score: which X points are closest to the most Y points?")
    .arg(Arg::with_name("FILE_X")
        .help("Path to CSV. If not supplied, will look to STDIN.")
        .required(false)
        .index(1))
    .arg(Arg::with_name("FILE_Y")
        .help("Path to CSV. If not supplied, will assume x and y are in FILE1.")
        .required(false)
        .index(2))
    .arg(Arg::with_name("CATEGORY")
        .short("c")
        .long("c")
        .value_name("CATEGORY")
        .help("Category column name. Default is 'category'.")
        .takes_value(true))
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
    .arg(Arg::with_name("output-y")
        .long("output-y")
        .takes_value(false)
        .help("Boolean. True will include all y values with score zero. False will only return x values."))
    .get_matches();

    let grid_size: Decimal =
        Decimal::from_str(matches.value_of("GRID-SIZE").unwrap_or("1")).unwrap();
    let radius: Decimal = 
        Decimal::from_str(matches.value_of("RADIUS").unwrap_or("5")).unwrap();
    let category : Option<&str> = matches.value_of("CATEGORY"); 
    let x_cat : Option<&str> = matches.value_of("X"); 
    let output_y : bool = matches.is_present("output-y"); 

    //File
    let mut x: Vec<density::Point>;
    let mut y: Vec<density::Point>;
    if matches.is_present("FILE_X") {
        let file1 = matches.value_of("FILE_X").unwrap();
        if matches.is_present("FILE_Y") {
            let file2 = matches.value_of("FILE_Y").unwrap();
            x = utils::csv_file_to_points(&file1, category);
            y = utils::csv_file_to_points(&file2, category);
        } else {
            let s = utils::csv_file_to_points(&file1, category);
            let x_y = utils::get_x_y(s, x_cat.unwrap());
            x = x_y.0; 
            y = x_y.1; 
        }
    } else {
        let s = utils::stdin_to_points(category);
        let x_y = utils::get_x_y(s, x_cat.unwrap());
        x = x_y.0; 
        y = x_y.1; 
}

    let n : Option<usize>; 
    if matches.is_present("N") {
        n = Some(matches.value_of("N").unwrap().parse::<usize>().unwrap()); 
    } else {
        n = None; 
    }

    let scores = density::score(&mut x, &mut y, radius, grid_size, output_y, n);

    utils::points_as_csv(scores).expect("Fail");
}
