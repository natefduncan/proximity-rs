//Grid size: distance in kilometers between the 
//grid lines that will support discretization 
//of data and density reference

//Radius: distance in kilometers that represents
//the local neighborhood where an event
//adds density

use itertools_num::structs::Linspace; 
use itertools_num::linspace; 

struct Point {
    latitude : f64, 
    longitude : f64
}

fn round_float(float: f64, decimal_places: i32) -> f64 {
    let mut mult: i32 = 1;
    for _ in 1..decimal_places {
        mult = mult * 10
    }
    let mult_float = f64::from(mult);
    return (float * mult_float).round() / mult_float;
}

fn point_density(points : Vec<Point>, radius : u32, grid_size : u32)  {
    let grid_size : f64 = round_float(grid_size/111.2, 3); 
    let rad_km = radius; // initial radius measurement in km
    let rad_dg = rad_km / 111.2; // radius as a latitudinal distance
    let rad_steps = floor(rad_dg/grid_size); 

    //Round all latitude data to the nearest grid. 
    let lat : Vec<f64> = points.iter().map(|&point| round_float(round_float(point.latitude / (1 / grid_size), 0) * grid_size, 3))
    let lon : Vec<f64> = points.iter().map(|&point| round_float(round_float(point.longitude / (1 / grid_size), 0) * grid_size, 3))
    println!("{:?}", points);
    println!("{:?}")
}

fn calc_density(lati : f64, loni : f64, radius : u32, grid_size : u32) ->  Linspace<f64> {
    let mut lat_vec = linspace(lati - radius * grid_size, lati + radius * grid_sizez, grid_size);
    let mut lat_vec_t = lat_vec.map(|&lat| ((radius * grid_size).cos() / (lat - lati).cos()).acos()); 
    lat_vec_t = lat_vec_t.map(|&lat| lat / (lat.))
    lat.vec.t <- lat.vec.t/cos(lat.vec * 2 * pi/360)
    lat.vec.t <- round(lat.vec.t/grid_size,0)*grid_size

}