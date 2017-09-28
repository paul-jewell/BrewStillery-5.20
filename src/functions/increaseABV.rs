#![allow(non_snake_case)]

use gtk::{self};
use gtk::prelude::*;
use regex::Regex;

pub fn increaseABVPrep(ref increaseABVBuilderClone: &gtk::Builder) {
    let increaseABVBrixInput: &gtk::Entry = &increaseABVBuilderClone.get_object("increaseABVBrixInput").unwrap();
    let increaseABVBrixInputBuffer = increaseABVBrixInput.get_text().expect("No input");

    let increaseABVABVInput: &gtk::Entry = &increaseABVBuilderClone.get_object("increaseABVABVInput").unwrap();
    let increaseABVABVInputBuffer = increaseABVABVInput.get_text().expect("No input");

    let increaseABVVolumeInput: &gtk::Entry = &increaseABVBuilderClone.get_object("increaseABVVolumeInput").unwrap();
    let increaseABVVolumeInputBuffer = increaseABVVolumeInput.get_text().expect("No input");

    let increaseABVNewBrixOutput = String::from("increaseABVNewBrixOutput");
    let isNumerical = Regex::new(r"^\d+\.\d+|\d+$").unwrap();
    let isCharacter = Regex::new(r"^\D$").unwrap();
    let isMismatched = Regex::new(r"^\d+\D+|\d+\D+\d+$").unwrap();

    if increaseABVBrixInputBuffer == "" || isNumerical.is_match(&increaseABVBrixInputBuffer) == false || isCharacter.is_match(&increaseABVBrixInputBuffer) == true || isMismatched.is_match(&increaseABVBrixInputBuffer) == true {
        let output: gtk::Entry = increaseABVBuilderClone.get_object(&increaseABVNewBrixOutput).unwrap();
        output.set_text("Enter a number");
    } else if increaseABVABVInputBuffer == "" || isNumerical.is_match(&increaseABVABVInputBuffer) == false || isCharacter.is_match(&increaseABVABVInputBuffer) == true || isMismatched.is_match(&increaseABVABVInputBuffer) == true {
        let output: gtk::Entry = increaseABVBuilderClone.get_object(&increaseABVNewBrixOutput).unwrap();
        output.set_text("Enter a number");
    } else if increaseABVVolumeInputBuffer == "" || isNumerical.is_match(&increaseABVVolumeInputBuffer) == false || isCharacter.is_match(&increaseABVVolumeInputBuffer) == true || isMismatched.is_match(&increaseABVVolumeInputBuffer) == true {
        let output: gtk::Entry = increaseABVBuilderClone.get_object(&increaseABVNewBrixOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        let increaseABVBrixInputBufferFloat: f32 = increaseABVBrixInputBuffer.parse().unwrap();
        let increaseABVABVInputBufferFloat: f32 = increaseABVABVInputBuffer.parse().unwrap();
        let increaseABVVolumeInputBufferFloat: f32 = increaseABVVolumeInputBuffer.parse().unwrap();
        if increaseABVBrixInputBufferFloat <= 0.0 {
            let output: gtk::Entry = increaseABVBuilderClone.get_object(&increaseABVNewBrixOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if increaseABVABVInputBufferFloat <= 0.0 {
            let output: gtk::Entry = increaseABVBuilderClone.get_object(&increaseABVNewBrixOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if increaseABVVolumeInputBufferFloat <= 0.0 {
            let output: gtk::Entry = increaseABVBuilderClone.get_object(&increaseABVNewBrixOutput).unwrap();
            output.set_text("Enter a positive number");
        } else {
            onIncreaseActivate(increaseABVBrixInputBuffer, increaseABVABVInputBuffer, increaseABVVolumeInputBuffer, &increaseABVBuilderClone);
        }
    }
}

pub fn onIncreaseActivate(increaseABVBrixInputBuffer: String, increaseABVABVInputBuffer: String, increaseABVVolumeInputBuffer: String, ref increaseABVBuilderClone: &gtk::Builder) {
    let ref increaseABVSwitch: &gtk::Switch = &increaseABVBuilderClone.get_object("increaseABVSwitch").unwrap();

    let startingBrix: f32 = increaseABVBrixInputBuffer.parse().unwrap();
    let desiredABV: f32 = increaseABVABVInputBuffer.parse().unwrap();
    let desiredWortVolume: f32 = increaseABVVolumeInputBuffer.parse().unwrap();

    if increaseABVSwitch.get_active() == true {
        let imperialOrMetric = String::from("metric");
        differenceBrixMaths(startingBrix, desiredABV, desiredWortVolume, imperialOrMetric, &increaseABVBuilderClone);
    } else if increaseABVSwitch.get_active() == false {
        let imperialOrMetric = String::from("imperial");
        differenceBrixMaths(startingBrix, desiredABV, desiredWortVolume, imperialOrMetric, &increaseABVBuilderClone);
    }
}

fn differenceBrixMaths(startingBrix: f32, desiredABV: f32, desiredWortVolume: f32, imperialOrMetric: String, ref increaseABVBuilderClone: &gtk::Builder) {
    let finalGravity: f32 = 1.015;
    // this is an ideal final gravity
    let originalExtract = -1.0 * ((513.11767463 * desiredABV + 59931.43605250) - (46882.32536333 * (0.00022734 * desiredABV.powi(2) + 0.02819081 * desiredABV + 1.63414684).sqrt())) / desiredABV;
    let originalGravity = 1.00001 + 0.0038661 * originalExtract + 1.3488 * 10.0_f32.powi(-5) * originalExtract.powi(2) + 4.3074 * 10.0_f32.powi(-8) * originalExtract.powi(3);
    let newStartingBrix = (258.6 * originalGravity - 258.6) / (0.87955073 * originalGravity + 0.12044926);
    let apparentExtract = (-668.962) + (1262.45 * finalGravity ) - (776.43 * finalGravity.powi(2)) + (182.94 * finalGravity.powi(3));
    let attenuationCoefficient = (0.22) + (0.001 * originalExtract);
    let realExtract = ((attenuationCoefficient * originalExtract) + apparentExtract) / (1.0 + attenuationCoefficient);
    let estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract));
    let newEstimatedABV = estimatedABW * (finalGravity / 0.794);
    let differenceBrix = newStartingBrix - startingBrix;
    let newSB = format!("{:.2}°Bx", newStartingBrix);
    let newABV = format!("{:.2}%", newEstimatedABV);

    let ref increaseABVNewBrixOutput: &gtk::Entry = &increaseABVBuilderClone.get_object("increaseABVNewBrixOutput").unwrap();
    let ref increaseABVEstimatedABVOutput: &gtk::Entry = &increaseABVBuilderClone.get_object("increaseABVEstimatedABVOutput").unwrap();
    let ref increaseABVSugarAddOutput: &gtk::Entry = &increaseABVBuilderClone.get_object("increaseABVSugarAddOutput").unwrap();
    let ref increaseABVHoneyAddOutput: &gtk::Entry = &increaseABVBuilderClone.get_object("increaseABVHoneyAddOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let sugarToAddImperial = (desiredWortVolume * 1.5 * differenceBrix) / 16.0;
        let honeyToAddImperial = sugarToAddImperial * 1.25;
        let sugar = format!("{} lbs {:.2} oz", sugarToAddImperial as u32, sugarToAddImperial % 1.0 * 16.0);
        let honey = format!("{} lbs {:.2} oz", honeyToAddImperial as u32, honeyToAddImperial % 1.0 * 16.0);
        increaseABVNewBrixOutput.set_text(&newSB);
        increaseABVEstimatedABVOutput.set_text(&newABV);
        increaseABVSugarAddOutput.set_text(&sugar);
        increaseABVHoneyAddOutput.set_text(&honey);
    } else if imperialOrMetric == "metric" {
        let sugarToAddMetric = ((desiredWortVolume / 3.78541) * 1.5 * differenceBrix) * 0.0283495;
        let honeyToAddMetric = sugarToAddMetric * 1.250001102;
        let sugar = format!("{:.2} kilos", sugarToAddMetric);
        let honey = format!("{:.2} kilos", honeyToAddMetric);
        increaseABVNewBrixOutput.set_text(&newSB);
        increaseABVEstimatedABVOutput.set_text(&newABV);
        increaseABVSugarAddOutput.set_text(&sugar);
        increaseABVHoneyAddOutput.set_text(&honey);
    }
}