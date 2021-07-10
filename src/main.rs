mod density;
mod utils;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() {
    // let lati: Decimal = dec!(32.78306);
    // let loni: Decimal = dec!(-96.80667);
    let grid_size: Decimal = dec!(0.1); //KM
    let radius: Decimal = dec!(4.0); //KM

    let points: Vec<density::Point> = utils::csv_file_to_Points("test.csv");
    let undes: Vec<density::Point> = utils::csv_file_to_Points("test.csv");
    let des: Vec<density::Point> = utils::csv_file_to_Points("test.csv");
    let scores = density::score(points, undes, des, radius, grid_size);
    println!("{:?}", scores);
    //let circle: Vec<String> = density::calc_density(lati, loni, radius, grid_size);
    //println!("{}", circle.len());
    //utils::output_as_csv(circle);
}
