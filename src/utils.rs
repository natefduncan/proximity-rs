use super::density::Point; 
use csv; 

pub fn csv_file_to_Points(file_path : &str) -> Vec<Point> {
    let mut rdr = csv::Reader::from_path(&file_path).expect("Could not get from path.");
    let mut output: Vec<Point> = Vec::new();
    for result in rdr.deserialize() {
        let record: Point = result.expect("Could not coerce to point.");
        output.push(record);
    }
    return output;
}