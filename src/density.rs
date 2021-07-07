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
    let lat : Vec<f64> = points.iter().map(|&point| point.latitude / (1 / grid_size))
    let lon : Vec<f64> = Vec::new();
    for point in points {
        lat.push(point.latitude * (1/grid_size));
        lon.push(point.longitude * (1/grid_size));
    }

    lat_data <- unlist(df[,lat_col])
    lat <- lat_data * (1/grid_size)
    lat <- round(lat, 0)
    lat <- lat * (grid_size)
    lat <- round(lat,3)

    //Round all longitude data to the nearest grid. 
    lon_data <- unlist(df[,lon_col])
    lon <- lon_data * (1/grid_size)
    lon <- round(lon, 0)
    lon <- lon * (grid_size)
    lon <- round(lon,3)
}

fn calc_density(radius : u32, grid_size : u32) ->  Linspace<f64> {
    let lat_vec = linspace() seq(lati - radius * grid_size, lati + radius * grid_size, grid_size)
}