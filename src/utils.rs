use super::density::{Point};
use csv;
use std::io;
use std::fs; 
use rust_decimal::prelude::*;

fn reader_to_points<R: io::Read>(rdr: &mut csv::Reader<R>, _category : &str) -> Vec<Point> {
    //Headers
    let headers = rdr.headers().expect("no headers");
    let (mut lati, mut loni, mut namei, mut cati) = (None, None, None, None); 
    for (i, header) in headers.iter().enumerate() {
        match header {
            "latitude" => lati = Some(i),
            "longitude" => loni = Some(i), 
            "name" => namei = Some(i), 
            _category => cati = Some(i), 
        }
    } 
    //Points
    let mut points : Vec<Point> = vec![];
    for result in rdr.records() {
        let temp = result.unwrap(); 
        points.push(Point {
            latitude : Decimal::from_str(&temp[lati.unwrap()]).expect("could not coerce to decimal"), 
            longitude : Decimal::from_str(&temp[loni.unwrap()]).expect("could not coerce to decimal"),
            name : Some(temp[namei.unwrap()].to_string()), 
            category : Some(temp[cati.unwrap()].to_string()),  
            score : None
        })
    }
    return points; 
}

pub fn csv_file_to_points(file_path: &str, category : &str) -> Vec<Point> {
    let path = fs::canonicalize(file_path).expect("Could not get file_path"); 
    let mut rdr = csv::Reader::from_path(path).expect("Could not get from path.");
    reader_to_points::<fs::File>(&mut rdr, category)
}

pub fn stdin_to_points(category : &str) -> Vec<Point> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    reader_to_points::<io::Stdin>(&mut rdr, category)
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
