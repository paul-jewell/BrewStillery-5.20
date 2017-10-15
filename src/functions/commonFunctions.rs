#![allow(non_snake_case)]
use std::f32::consts::E;

pub fn brixToGravity(brix: f32) -> f32 {
    (brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.0
}

pub fn gravityToPlato(gravity: f32) -> f32 {
    135.997 * gravity.powi(3) - 630.272 * gravity.powi(2) + 1111.14 * gravity - 616.868
}

pub fn gramsToOunces(grams: f32) -> f32 {
    0.03527396_f32 * grams
}

pub fn litresToGallons(litres: f32) -> f32 {
    0.2641729_f32 * litres
}

pub fn validInput(stringToCheck: &String) -> f32 {
    let isANumber = stringToCheck.parse::<f32>().is_ok();
    if isANumber == true {
        stringToCheck.parse().unwrap()
    } else {
        0.0
    }
}

pub fn realIBU(brix: f32, wortVolume: f32, boilTime: f32, alphaAcid: f32, hopAmount: f32) -> f32 {
    (1.65 * 0.000125_f32.powf(brixToGravity(brix) - 1.0)) * ((1.0 - E.powf(-0.04 * boilTime)) / 4.15) * (( (alphaAcid / 100.0) * hopAmount * 7490.0 ) / wortVolume)
}