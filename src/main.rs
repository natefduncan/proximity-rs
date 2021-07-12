mod density;
mod utils;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::time::Instant;

fn main() {
    // let lati: Decimal = dec!(32.78306);
    // let loni: Decimal = dec!(-96.80667);
    let grid_size: Decimal = dec!(1); //KM
    let radius: Decimal = dec!(5); //KM

    let mut points: Vec<density::Point> = utils::csv_file_to_Points("./test/mesa_church.csv");
    let mut des: Vec<density::Point> = utils::csv_file_to_Points("./test/mesa_restaurant.csv");
    let mut undes: Vec<density::Point> = utils::csv_file_to_Points("test.csv");

    let now = Instant::now();
    let mut scores = density::score(&points, &des, &undes, radius, grid_size);
    scores.sort_by(|a, b| {
        b.score
            .as_ref()
            .unwrap()
            .desirable
            .cmp(&a.score.as_ref().unwrap().desirable)
    });

    let mut top_ten : Vec<density::Point> = scores.into_iter().take(10).collect::<Vec<density::Point>>();
    utils::score_as_csv(&mut top_ten, &mut des, &mut undes).expect("Fail");
}
