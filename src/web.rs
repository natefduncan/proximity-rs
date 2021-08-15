use ndarray;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

fn num_after_decimal(x : f64) -> u32 {
    x.to_string()
    .split(".")
    .nth(0)
    .unwrap()
    .chars()
    .nth(0)
    .unwrap() as u32
}

fn round(x: f64, n: u32) -> f64 {
    let mut m = 1.; 
    for _ in 0..n {
        m = m * 10.; 
    }
    if num_after_decimal(x * m) >= 5 {
        match x.is_sign_negative() {
            true => (x * m).floor() / m, 
            false => (x * m).ceil() / m, 
        } 

    } else {
        match x.is_sign_negative() {
            true => (x * m).ceil() / m, 
            false => (x * m).floor() / m
        }
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test(a : u32, b: u32) {
    alert("Test!"); 
}

pub fn score(
    //(id, latitude, longitude)
    x : &JsValue,
    y : &JsValue,
    radius : f64,
    grid_size : f64, 
    x_n : Option<usize>,
) -> Vec<JsValue> {
    let mut x : Vec<(usize, f64, f64)> = x.into_serde().unwrap();
    let y : Vec<(usize, f64, f64)> = y.into_serde().unwrap(); 

    //convert grid size and radius
    let grid_size: f64 = round(grid_size / 111.2, 3);
    let rad_dg: f64 = radius / 111.2; // radius as a latitudinal distance
    let rad_steps: f64 = round(rad_dg / grid_size, 0);
    let n = ((2.0 * rad_steps) + 1.0) as usize;

    //Get density
    let y_density = point_density(&y, grid_size, n, rad_dg);

    //Score each point
    let mut scores: Vec<(usize, f64, f64, usize)> = x
        .iter_mut()
        .map(|&mut point| {
            let (id, latitude, longitude) = point; 
            let lati = round(
                round(latitude * (1.0 / grid_size), 0) * grid_size,
                3,
            );
            let loni = round(
                round(longitude * (1.0 / grid_size), 0) * grid_size,
                3,
            );
            let coord_string = format!("{}_{}", lati, loni);
            let score: usize = match y_density.get(&coord_string) {
                Some(&number) => number,
                _ => 0,
            };
            let output : (usize, f64, f64, usize) = (id, latitude, longitude, score); 
            output
        })
        .collect::<Vec<(usize, f64, f64, usize)>>();

    //Sort by score: All x will be on top
    scores.sort_by(|a, b| {
        b.3.cmp(&a.3)
    }); 

    //Return N of X
    match x_n {
        Some(v) => {
            scores = scores.into_iter().take(v).collect::<Vec<(usize, f64, f64, usize)>>();
        }, 
        None => ()
    }

    //Conver to JS Array
    scores.iter().map(|s| JsValue::from(s.0 as u32)).collect::<Vec<JsValue>>()
}

fn point_density(
    //(id, latitude, longitude)
    points : &Vec<(usize, f64, f64)>, 
    grid_size: f64,
    n: usize,
    rad_dg: f64,
) -> HashMap<String, usize> {
    let rad_steps: f64 = round(rad_dg / grid_size, 0);
    let mut grid: HashMap<String, usize> = HashMap::new();
    for point in points {
        let (_, latitude, longitude) = point; 
        let lati = round(
            round(latitude * (1.0 / grid_size), 0) * grid_size,
            3,
        );
        let loni = round(
            round(longitude * (1.0 / grid_size), 0) * grid_size,
            3,
        );
        for g in calc_density(lati, loni, rad_steps, grid_size, n, rad_dg) {
            *grid.entry(g).or_default() += 1;
        }
    }
    return grid;
}

fn calc_density(
    lati: f64,
    loni: f64,
    rad_steps: f64,
    grid_size: f64,
    n: usize,
    rad_dg: f64,
) -> Vec<String> {
    // Get lat vector and matrix
    let lat_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        lati - rad_steps * grid_size,
        lati + rad_steps * grid_size,
        n,
    );
    let mut lat_mat = ndarray::Array2::<f64>::zeros((lat_vec.len(), lat_vec.len()));
    for (i, mut row) in lat_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(round(lat_vec[i], 3));
    }

    //Get lon vector and matrix
    let lon_vec: ndarray::Array1<f64> = ndarray::Array::linspace(
        loni - rad_steps * grid_size,
        loni + rad_steps * grid_size,
        n,
    );
    let mut lon_mat = ndarray::Array2::<f64>::zeros((lon_vec.len(), lon_vec.len()));
    for (i, mut row) in lon_mat.axis_iter_mut(ndarray::Axis(0)).enumerate() {
        row.fill(round(lon_vec[i], 3));
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
