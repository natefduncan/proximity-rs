mod density;
mod utils;

fn main() {
    let lati: f64 = 32.78306;
    let loni: f64 = -96.80667;
    let grid_size: f64 = 0.1;
    let radius: f64 = 1.0;

    let points : Vec<density::Point> = utils::csv_file_to_Points("test.csv");  
    // density::point_density(&points, radius, grid_size); 
    let circle : Vec<density::Point> = density::calc_density(lati, loni, radius, grid_size);
    //println!("{}", circle.len()); 
    utils::output_as_csv(circle); 
}
