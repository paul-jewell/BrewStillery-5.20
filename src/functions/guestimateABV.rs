#![allow(non_snake_case)]

use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn guestimatePrep(ref guestimatorBuilderClone: &gtk::Builder) {
    let guestimatorInput: &gtk::Entry = &guestimatorBuilderClone.get_object("guestimatorInput").unwrap();
    let guestimatorInput = guestimatorInput.get_text().expect("No input");
    let startingBrix = validInput(&guestimatorInput);
    
    let guestimatorOutput = String::from("guestimatorOutput");

    if startingBrix == 0.0 {
        let output: gtk::Entry = guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        if startingBrix < 3.83 {
            let output: gtk::Entry = guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
            output.set_text("Enter a number greater than 3.83");
        } else if startingBrix > 49.48 {
            let output: gtk::Entry = guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
            output.set_text("Enter a number less than 49.48");
        } else {
            guestiMaths(startingBrix, guestimatorOutput, &guestimatorBuilderClone);
        }
    }
}

pub fn guestiMaths(startingBrix: f32, guestimatorOutput: String, ref guestimatorBuilderClone: &gtk::Builder) {
    let originalGravity = brixToGravity(startingBrix);
    let finalGravity: f32 = 1.015;
    // since finalGravity is unknown, this constant is ideal
    let originalExtract = (-668.962) + (1262.45 * originalGravity ) - (776.43 * originalGravity.powi(2)) + (182.94 * originalGravity.powi(3));
    let apparentExtract = (-668.962) + (1262.45 * finalGravity ) - (776.43 * finalGravity.powi(2)) + (182.94 * finalGravity.powi(3));
    let attenuationCoefficient = (0.22) + (0.001 * originalExtract);
    let realExtract = ((attenuationCoefficient * originalExtract) + apparentExtract) / (1.0 + attenuationCoefficient);

    // unneeded information, but here if we want it
    //
    // let realAttenuation = ((originalExtract - realExtract) / originalExtract) * 100.0;
    // let realAttenuationRounded = format!("{:.2}", realAttenuation);
    // println!("Your Real Attenuation is: {}", realAttenuationRounded);

    let estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract));
    let estimatedABV = estimatedABW * (finalGravity / 0.794);
    let abv = format!("{:.2}%", estimatedABV);
    let ref output: &gtk::Entry = &guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
    output.set_text(&abv.to_string());
}