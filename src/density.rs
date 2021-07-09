//Grid size: distance in kilometers between the
//grid lines that will support discretization
//of data and density reference

//Radius: distance in kilometers that represents
//the local neighborhood where an event
//adds density

use ndarray;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    latitude: f64,
    longitude: f64,
    name: Option<String>,
}

fn round_float(float: f64, decimal_places: i32) -> f64 {
    let mut mult: i32 = 10;
    for _ in 1..decimal_places {
        mult = mult * 10
    }
    let mult_float = f64::from(mult);
    return (float * mult_float).round() / mult_float;
}

pub fn point_density(points: &Vec<Point>, radius: f64, grid_size: f64) -> Result<(), Error> {
    //convert grid size and radius
    let grid_size: f64 = round_float(grid_size / 111.2, 3);
    let rad_dg: f64 = radius / 111.2; // radius as a latitudinal distance
    let rad_steps: f64 = (rad_dg / grid_size).round();
    let n = ((2.0 * rad_steps) + 1.0) as usize;

    //round all latitude data to nearest grid
    let lat = points
        .into_iter()
        .map(|point| {
            return round_float(
                round_float(point.latitude * (1.0 / grid_size), 0) * grid_size,
                3,
            );
        })
        .collect::<Vec<f64>>();
    let lon = points
        .into_iter()
        .map(|point| {
            return round_float(
                round_float(point.longitude * (1.0 / grid_size), 0) * grid_size,
                3,
            );
        })
        .collect::<Vec<f64>>();

    let olat_olon: Vec<String> = lat
        .iter()
        .zip(lon.iter())
        .map(|coord| {
            let lati = coord.0;
            let loni = coord.1;
            return format!("{}_{}", lati.to_string(), loni.to_string());
        })
        .collect::<Vec<String>>();

    let rlat_rlon: Vec<String> = points
        .into_iter()
        .map(|point| return format!("{}_{}", point.latitude, point.longitude))
        .collect::<Vec<String>>();

    vector_grid_points
    println!("{:?}", rlat_rlon);
    Ok(())
}

pub struct Error;

pub fn calc_density(lati: f64, loni: f64, radius: f64, grid_size: f64) -> Vec<Point> {
    // Get lat vector and matrix
    let mut lat_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        lati - rad_steps * grid_size,
        lati + rad_steps * grid_size,
        n,
    );
    let mut lat_mat = ndarray::Array2::<f64>::zeros((lat_vec.len(), lat_vec.len()));
    for (i, mut row) in lat_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(lat_vec[i]);
    }

    //Get lon vector and matrix
    let lon_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        loni - rad_steps * grid_size,
        loni + rad_steps * grid_size,
        n,
    );
    let mut lon_mat = ndarray::Array2::<f64>::zeros((lon_vec.len(), lon_vec.len()));
    for (i, mut row) in lon_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(lon_vec[i]);
    }
    //Transpose lon
    let lon_mat = lon_mat.t();

    //Loop through matrix and check if in circle.
    let lat_vec = lat_mat
        .to_shape((1, lat_mat.nrows() * lat_mat.ncols()))
        .expect("Could not flatten 2D array.")
        .into_owned();
    let lon_vec = lon_mat
        .to_shape((1, lon_mat.nrows() * lon_mat.ncols()))
        .expect("Could not flatten 2D array.")
        .into_owned();

    //Loop through both arrays and return point
    let mut output: Vec<Point> = Vec::new();
    for i in lat_mat.indexed_iter() {
        //Check if in circle.
        let mut lat = i.1;
        let mut lon = lon_mat[[i.0 .0, i.0 .1]];

        if (lat - lati).powf(2.0) + (lon - loni).powf(2.0) <= rad_dg.powf(2.0) {
            // println!("Left : {}", (lati-lat).powf(2.0) + (loni-lon).powf(2.0));
            // println!("Right : {}", rad_dg.powf(2.0));
            output.push(Point {
                latitude: *lat,
                longitude: lon,
                name: Some("test".to_string()),
            })
        }
    }
    output
}
