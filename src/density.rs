//Grid size: distance in kilometers between the
//grid lines that will support discretization
//of data and density reference

//Radius: distance in kilometers that represents
//the local neighborhood where an event
//adds density

use ndarray;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    latitude: Decimal,
    longitude: Decimal,
    name: Option<String>,
}

#[derive(Debug)]
pub struct Density {
    coord_string: String,
    density: usize,
}

#[derive(Debug)]
pub struct Score {
    desirable: usize,
    undesirable: usize,
}

fn round(x: Decimal, n: u32) -> Decimal {
    return x.round_dp_with_strategy(n, RoundingStrategy::MidpointAwayFromZero);
}

pub fn score(
    points: Vec<Point>,
    desirables: Vec<Point>,
    undesirables: Vec<Point>,
    radius: Decimal,
    grid_size: Decimal,
) -> Vec<Score> {
    //convert grid size and radius
    let grid_size: Decimal = round(grid_size / dec!(111.2), 3);
    let rad_dg: Decimal = radius / dec!(111.2); // radius as a latitudinal distance
    let rad_steps: Decimal = round(rad_dg / grid_size, 0);
    let n = ((2.0 * rad_steps.to_f64().unwrap()) + 1.0) as usize;

    //Get density
    let desirable_density = point_density(&desirables, grid_size, n, rad_dg);
    let undesirable_density = point_density(&undesirables, grid_size, n, rad_dg);
    //Score each point
    let scores: Vec<Score> = points
        .iter()
        .map(|point| {
            let lati = round(
                round(point.latitude * (dec!(1.0) / grid_size), 0) * grid_size,
                3,
            );
            let loni = round(
                round(point.longitude * (dec!(1.0) / grid_size), 0) * grid_size,
                3,
            );
            let coord_string = format!("{}_{}", lati, loni);
            let desirable_score: usize = match desirable_density.get(&coord_string) {
                Some(&number) => number,
                _ => 0,
            };
            let undesirable_score: usize = match undesirable_density.get(&coord_string) {
                Some(&number) => number,
                _ => 0,
            };
            Score {
                desirable: desirable_score,
                undesirable: undesirable_score,
            }
        })
        .collect::<Vec<Score>>();
    scores
}

pub fn point_density(
    points: &Vec<Point>,
    grid_size: Decimal,
    n: usize,
    rad_dg: Decimal,
) -> HashMap<String, usize> {
    let rad_steps: Decimal = round(rad_dg / grid_size, 0);

    //round all latitude data to nearest grid
    let lat = points
        .into_iter()
        .map(|point| {
            return round(
                round(point.latitude * (dec!(1.0) / grid_size), 0) * grid_size,
                3,
            );
        })
        .collect::<Vec<Decimal>>();
    let lon = points
        .into_iter()
        .map(|point| {
            return round(
                round(point.longitude * (dec!(1.0) / grid_size), 0) * grid_size,
                3,
            );
        })
        .collect::<Vec<Decimal>>();

    let mut grid: HashMap<String, usize> = HashMap::new();
    for point in points {
        for g in calc_density(
            point.latitude,
            point.longitude,
            rad_steps,
            grid_size,
            n,
            rad_dg,
        ) {
            *grid.entry(g).or_default() += 1;
        }
    }
    return grid;
}

pub fn calc_density(
    lati: Decimal,
    loni: Decimal,
    rad_steps: Decimal,
    grid_size: Decimal,
    n: usize,
    rad_dg: Decimal,
) -> Vec<String> {
    // Get lat vector and matrix
    let lat_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        (lati - rad_steps * grid_size).to_f64().unwrap(),
        (lati + rad_steps * grid_size).to_f64().unwrap(),
        n,
    );
    let mut lat_mat = ndarray::Array2::<Decimal>::zeros((lat_vec.len(), lat_vec.len()));
    for (i, mut row) in lat_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(round(Decimal::from_f64(lat_vec[i]).unwrap(), 3));
    }

    //Get lon vector and matrix
    let lon_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        (loni - rad_steps * grid_size).to_f64().unwrap(),
        (loni + rad_steps * grid_size).to_f64().unwrap(),
        n,
    );
    let mut lon_mat = ndarray::Array2::<Decimal>::zeros((lon_vec.len(), lon_vec.len()));
    for (i, mut row) in lon_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(round(Decimal::from_f64(lon_vec[i]).unwrap(), 3));
    }
    //Transpose lon
    let lon_mat = lon_mat.t();

    //Loop through both arrays and return point
    let mut output: Vec<String> = Vec::new();
    for i in lat_mat.indexed_iter() {
        //Check if in circle.
        let lat = i.1;
        let lon = lon_mat[[i.0 .0, i.0 .1]];

        if (lat - lati).powi(2) + (lon - loni).powi(2) <= rad_dg.powi(2) {
            output.push(format!("{}_{}", *lat, lon));
        }
    }
    output
}
