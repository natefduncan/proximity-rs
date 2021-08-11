//Modeled after https://cran.r-project.org/web/packages/pointdensityP/index.html

use ndarray;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Point {
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub name: Option<String>,
    pub category: Option<String>, 
    pub score: Option<usize>,
}

#[derive(Debug)]
pub struct Density {
    pub coord_string: String,
    pub density: usize,
}

pub fn round(x: Decimal, n: u32) -> Decimal {
    return x.round_dp_with_strategy(n, RoundingStrategy::MidpointAwayFromZero);
}

pub fn score(
    points: Vec<Point>,
    x : &str, 
    y : &str,
    radius: Decimal,
    grid_size: Decimal,
    output_y : bool, 
    x_n : Option<usize>
) -> Vec<Point> {
    //convert grid size and radius
    let grid_size: Decimal = round(grid_size / dec!(111.2), 3);
    let rad_dg: Decimal = radius / dec!(111.2); // radius as a latitudinal distance
    let rad_steps: Decimal = round(rad_dg / grid_size, 0);
    let n = ((2.0 * rad_steps.to_f64().unwrap()) + 1.0) as usize;

    //Split into x and y points
    let mut x_points : Vec<Point> = Vec::new();
    let mut y_points : Vec<Point> = Vec::new();
    for point in &points {
        if point.category == Some(x.to_owned()) {
            x_points.push(point.clone());
        } else if point.category == Some(y.to_owned()) {
            y_points.push(point.clone()); 
        }
    }

    //Get density
    let y_density = point_density(&y_points, grid_size, n, rad_dg);

    //Score each point
    let mut scores: Vec<Point> = x_points
        .iter_mut()
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
            let score: usize = match y_density.get(&coord_string) {
                Some(&number) => number,
                _ => 0,
            };

            let output = Point {
                latitude: point.latitude,
                longitude: point.longitude,
                name: point.name.to_owned(),
                category: point.category.to_owned(),
                score: Some(score),
            };
            output
        })
        .collect::<Vec<Point>>();

    //Sort by score: All x will be on top
    scores.sort_by(|a, b| {
        b.score.cmp(&a.score)
    }); 

    //Return N of X
    match x_n {
        Some(v) => {
            scores = scores.into_iter().take(v).collect::<Vec<Point>>();
        }, 
        None => ()
    }

    //Append Y points
    if output_y {
        scores.append(&mut y_points);  
    }
    scores
}

pub fn point_density(
    points: &Vec<Point>,
    grid_size: Decimal,
    n: usize,
    rad_dg: Decimal,
) -> HashMap<String, usize> {
    let rad_steps: Decimal = round(rad_dg / grid_size, 0);
    let mut grid: HashMap<String, usize> = HashMap::new();
    for point in points {
        let lati = round(
            round(point.latitude * (dec!(1.0) / grid_size), 0) * grid_size,
            3,
        );
        let loni = round(
            round(point.longitude * (dec!(1.0) / grid_size), 0) * grid_size,
            3,
        );
        for g in calc_density(lati, loni, rad_steps, grid_size, n, rad_dg) {
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
