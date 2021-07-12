use super::density::{Density, Point, Score};
use csv;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::io;

pub fn csv_file_to_Points(file_path: &str) -> Vec<Point> {
    let mut rdr = csv::Reader::from_path(&file_path).expect("Could not get from path.");
    let mut output: Vec<Point> = Vec::new();
    for result in rdr.deserialize() {
        let record: Point = result.expect("Could not coerce to point.");
        output.push(record);
    }
    return output;
}

pub fn points_as_csv(data: Vec<Point>) -> Result<(), std::io::Error> {
    //let file = File::create(&file_name).expect("Failed to create outfile.");
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());
    wtr.write_record(&["latitude", "longitude", "name"])?;
    for p in data {
        wtr.serialize(p)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn density_as_csv(data: HashMap<String, usize>) -> Result<(), std::io::Error> {
    let mut places: Vec<Point> = Vec::new();
    for (coord_string, density) in data.iter() {
        let vec = coord_string.split("_").collect::<Vec<&str>>();
        places.push(Point {
            latitude: Decimal::from_str(vec[0]).unwrap(),
            longitude: Decimal::from_str(vec[1]).unwrap(),
            name: Some(density.to_string()),
            category: None,
            score: None,
        })
    }
    points_as_csv(places).expect("Could not get places");
    Ok(())
}

pub fn score_as_csv(
    point: &mut Vec<Point>,
    des: &mut Vec<Point>,
    undes: &mut Vec<Point>,
) -> Result<(), std::io::Error> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());
    wtr.write_record(&["latitude", "longitude", "name", "category", "desirable", "undesirable"])?;
    for p in point {
        p.category = Some("Point".to_string());
        wtr.serialize(p)?;
    }

    for d in des {
        d.category = Some("Desirable".to_string());
        d.score = Some(Score {
            desirable : 0, 
            undesirable : 0
        }); 
        wtr.serialize(d)?;
    }

    for u in undes {
        u.category = Some("Undesirable".to_string());
        u.score = Some(Score {
            desirable : 0, 
            undesirable : 0
        }); 
        wtr.serialize(u)?;
    }
    wtr.flush()?;
    Ok(())
}
