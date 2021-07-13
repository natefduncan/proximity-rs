use super::density::{Density, Point};
use csv;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::io;

pub fn csv_file_to_points(file_path: &str) -> Vec<Point> {
    let mut rdr = csv::Reader::from_path(&file_path).expect("Could not get from path.");
    let mut output: Vec<Point> = Vec::new();
    for result in rdr.deserialize() {
        let record: Point = result.expect("Could not coerce to point.");
        output.push(record);
    }
    return output;
}

pub fn stdin_to_points() -> Vec<Point> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut output: Vec<Point> = Vec::new();
    for result in rdr.deserialize() {
        let record: Point = result.expect("Could not coerce to point.");
        output.push(record);
    }
    return output;
}

pub fn points_as_csv(data: Vec<Point>) -> Result<(), std::io::Error> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());
    wtr.write_record(&["latitude", "longitude", "name", "category", "score"])?;
    for p in data {
        wtr.serialize(p)?;
    }
    wtr.flush()?;
    Ok(())
}
