use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
pub fn generate_tinkerbell(iterations: u32) -> Vec<f64> {
    let mut points = Vec::with_capacity(iterations as usize * 2);
    let (mut x, mut y) = (-0.72, -0.64);
    let (a, b, c, d) = (0.9, -0.6013, 2.0, 0.50);

    for _ in 0..iterations {
        let x_new = x * x - y * y + a * x + b * y;
        let y_new = 2.0 * x * y + c * x + d * y;
        x = x_new;
        y = y_new;
        points.push(x);
        points.push(y);
    }

    points
}