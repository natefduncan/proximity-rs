mod density;
mod utils;

fn main() {
    let lati: f64 = 90.123456789;
    let loni: f64 = -32.123456789;
    let grid_size: f64 = 0.1;
    let radius: f64 = 1.0;

    let points : Vec<density::Point> = utils::csv_file_to_Points("test.csv");  
    density::point_density(&points, radius, grid_size); 
}
