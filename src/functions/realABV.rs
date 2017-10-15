#![allow(non_snake_case)]

use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn realABVPrep(ref realABVBuilderClone: &gtk::Builder) {
    let realABVStartingBrixInput: &gtk::Entry = &realABVBuilderClone.get_object("realABVStartingBrixInput").unwrap();
    let realABVStartingBrixInputBuffer = realABVStartingBrixInput.get_text().expect("No input");
    let startingBrix = validInput(&realABVStartingBrixInputBuffer);

    let realABVFinalBrixInput: &gtk::Entry = &realABVBuilderClone.get_object("realABVFinalBrixInput").unwrap();
    let realABVFinalBrixInputBuffer = realABVFinalBrixInput.get_text().expect("No input");
    let finalBrix = validInput(&realABVFinalBrixInputBuffer);

    let realABVRealAttenuationOutput = String::from("realABVRealAttenuationOutput");

    if startingBrix == 0.0 {
        let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
        output.set_text("Enter a number");
    } else if finalBrix == 0.0 {
        let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        if startingBrix <= 0.0 {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if finalBrix <= 0.0 {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if startingBrix < finalBrix {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
            output.set_text("Starting brix must be greater than final brix");
        } else {
            realABVMaths(startingBrix, finalBrix, &realABVBuilderClone);
        }
    }
}

pub fn realABVMaths(startingBrix: f32, finalBrix: f32, ref realABVBuilderClone: &gtk::Builder) {
    let ref realABVRealAttenuationOutput: &gtk::Entry = &realABVBuilderClone.get_object("realABVRealAttenuationOutput").unwrap();
    let ref realABVFinalABVOutput: &gtk::Entry = &realABVBuilderClone.get_object("realABVFinalABVOutput").unwrap();

    let originalGravity = (startingBrix / (258.6 - ((startingBrix / 258.2) * 227.1))) + 1.0;
    let finalGravity = (finalBrix / (258.6 - ((finalBrix / 258.2) * 227.1))) + 1.0;
    let originalExtract = (-668.962) + (1262.45 * originalGravity) - (776.43 * originalGravity.powi(2)) + (182.94 * originalGravity.powi(3));
    let apparentExtract = (-668.962) + (1262.45 * finalGravity) - (776.43 * finalGravity.powi(2)) + (182.94 * finalGravity.powi(3));
    let attenuationCoefficient = (0.22) + (0.001 * originalExtract);
    let realExtract = (attenuationCoefficient * originalExtract + apparentExtract) / (1.0 + attenuationCoefficient);
    let realAttenuation = ((originalExtract - realExtract) / originalExtract) * 100.0;
    let attenuation = format!("{:.2}%", realAttenuation);
    let estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract));
    let finalABV = estimatedABW * (finalGravity / 0.794);
    let abv = format!("{:.2}%", finalABV);
    realABVRealAttenuationOutput.set_text(&attenuation);
    realABVFinalABVOutput.set_text(&abv);
}