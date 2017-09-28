#![allow(non_snake_case)]

pub fn brixToGravity(brix: f32) -> f32 {
    (brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.0
}


pub fn gravityToPlato(gravity: f32) -> f32 {
    135.997 * gravity.powi(3) - 630.272 * gravity.powi(2) + 1111.14 * gravity - 616.868
}