use super::density::Point; 
use csv; 
use std::io; 

pub fn csv_file_to_Points(file_path : &str) -> Vec<Point> {
    let mut rdr = csv::Reader::from_path(&file_path).expect("Could not get from path.");
    let mut output: Vec<Point> = Vec::new();
    for result in rdr.deserialize() {
        let record: Point = result.expect("Could not coerce to point.");
        output.push(record);
    }
    return output;
}

pub fn output_as_csv(data: Vec<Point>) -> Result<(), std::io::Error> {
    //let file = File::create(&file_name).expect("Failed to create outfile.");
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());
    wtr.write_record(&[
        "latitude",
        "longitude",
        "name",
    ])?;
    for p in data {
        wtr.serialize(p)?;
    }
    wtr.flush()?;
    Ok(())
}