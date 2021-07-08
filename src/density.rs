//Grid size: distance in kilometers between the
//grid lines that will support discretization
//of data and density reference

//Radius: distance in kilometers that represents
//the local neighborhood where an event
//adds density

use ndarray;
use std::f64::consts::PI;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    latitude: f64,
    longitude: f64,
}

fn round_float(float: f64, decimal_places: i32) -> f64 {
    let mut mult: i32 = 10;
    for _ in 1..decimal_places {
        mult = mult * 10
    }
    let mult_float = f64::from(mult);
    return (float * mult_float).round() / mult_float;
}

pub fn point_density(points : &Vec<Point>, radius : f64, grid_size : f64) -> Result<(), Error> {
    //convert grid size and radius
    let grid_size: f64 = round_float(grid_size / 111.2, 3);
    let rad_dg: f64 = radius / 111.2; // radius as a latitudinal distance
    let rad_steps: f64 = (rad_dg / grid_size).round();
    let n = ((2.0 * rad_steps) + 1.0) as usize;
    println!("Grid Size: {}", grid_size);
    println!("rad_steps: {}", rad_steps);

    //round all latitude data to nearest grid
    let lat = points.into_iter().map(|point| return round_float(round_float(point.latitude * (1.0 / grid_size), 0) * grid_size, 3)).collect::<Vec<f64>>();
    let lon = points.into_iter().map(|point| return round_float(round_float(point.longitude * (1.0 / grid_size), 0) * grid_size, 3)).collect::<Vec<f64>>();
    
    let olat_olon : Vec<String> = lat.iter()
        .zip(lon.iter())
        .map(|coord| {
            let lati = coord.0; 
            let loni = coord.1; 
            return format!("{}_{}", lati.to_string(), loni.to_string()); 
        })
        .collect::<Vec<String>>(); 

    let rlat_rlon : Vec<String> = points.into_iter()
        .map(|point| {
            return format!("{}_{}", point.latitude, point.longitude)
        })
        .collect::<Vec<String>>(); 
    
    println!("{:?}", rlat_rlon);
    Ok(())
}

pub struct Error;

pub fn calc_density(lati: f64, loni: f64, radius: f64, grid_size: f64) -> Result<(), Error> {
    let grid_size: f64 = round_float(grid_size / 111.2, 3);
    let rad_dg: f64 = radius / 111.2; // radius as a latitudinal distance
    let rad_steps: f64 = (rad_dg / grid_size).round();
    let n = ((2.0 * rad_steps) + 1.0) as usize;

    // Get lat vector and tolerance
    let mut lat_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        lati - rad_steps * grid_size,
        lati + rad_steps * grid_size,
        n,
    );
    let mut lat_vec_t =
        lat_vec.map(|lat_t| ((rad_steps * grid_size).cos() / (lat_t - lati).cos()).acos());
    ndarray::Zip::from(&mut lat_vec_t)
        .and(&lat_vec)
        .for_each(|lat_t, &lat| {
            *lat_t = *lat_t / ((lat * 2.0 * PI / 360.0).cos());
        });
    lat_vec_t = lat_vec_t.map(|lat_t| round_float(lat_t / grid_size, 0) * grid_size);

    //vector that contains all longitude grids in neighborhood
    let lon_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        loni - rad_steps * grid_size,
        loni + rad_steps * grid_size,
        n,
    );

    //matrix that contains lon position of every grid in neighborhood
    let mut lon_mat = ndarray::Array2::<f64>::zeros((lon_vec.len(), lon_vec.len()));
    for (i, mut row) in lon_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(lon_vec[i]);
    }
    let tlon_mat = (&lon_mat - loni).mapv(f64::abs);
    let tlon_mat = tlon_mat.t();

    //apply latitude tolerance, zero-out all points not within neighborhood
    let temp = lat_vec_t - tlon_mat;
    let temp = temp.map(|x| {
        if x < &(grid_size - (10.0 as f64).powi(-6)) {
            return 0.0;
        } else if x > &0.0 {
            return 1.0;
        } else {
            return *x;
        }
    });
    let temp2 = temp * &lon_mat.t();
    //matrix containing latitude of all grids in neighborhood
    let mut lat_mat = ndarray::Array2::<f64>::zeros((lat_vec.len(), lat_vec.len()));
    for (i, mut row) in lat_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(lat_vec[lat_vec.len() - i - 1]);
    }
    let lat_vec = lat_mat
        .to_shape((1, lat_mat.nrows() * lat_mat.ncols()))
        .expect("Could not flatten 2D array.")
        .into_owned();
    let lon_vec = temp2
        .to_shape((1, temp2.nrows() * temp2.ncols()))
        .expect("Could not flatten 2D array.")
        .into_owned();
    //eliminate all rows not in neighborhood (0 value for lon)
        // row_sub = apply(return.mat, 1, function(x) all(x[1]*x[2] !=0 ))
        // return.mat <- return.mat[row_sub,]
    let mut counter = 0; 
    (lon_vec * lat_vec).map(|x| {
        if x != &0.0 {
            counter = counter + 1;
        } else {}
    });
    println!("{:?}", counter);
    // lat.vec <- c(lat.mat)
    // lon.vec <- c(temp2)
    // println!("{:?}", lat_mat);
    // count.vec <- rep(count,length(lat.vec))
    // sumDate.vec <- rep(sumDate,length(lat.vec))
    // return.mat <- cbind(lat.vec,lon.vec,count.vec,sumDate.vec)
    // print(return.mat)
    // #eliminate all rows not in neighborhood (0 value for lon)
    // row_sub = apply(return.mat, 1, function(x) all(x[1]*x[2] !=0 ))
    // return.mat <- return.mat[row_sub,]

    Ok(())
}
