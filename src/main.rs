mod density;
mod utils;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() {
    // let lati: Decimal = dec!(32.78306);
    // let loni: Decimal = dec!(-96.80667);
    let grid_size: Decimal = dec!(1); //KM
    let radius: Decimal = dec!(5)   ; //KM

    let points: Vec<density::Point> = utils::csv_file_to_Points("./test/mesa_church.csv");
    let des: Vec<density::Point> = utils::csv_file_to_Points("./test/mesa_restaurant.csv");
    let undes: Vec<density::Point> = utils::csv_file_to_Points("test.csv");

    println!("{}", points.len()); 
    let scores = density::score(points, des, undes, radius, grid_size);
    println!("{}", scores.len());
}
