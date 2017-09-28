#![allow(non_snake_case)]

use gtk::{self};
use gtk::prelude::*;
use regex::Regex;

pub fn realABVPrep(ref realABVBuilderClone: &gtk::Builder) {
    let realABVStartingBrixInput: &gtk::Entry = &realABVBuilderClone.get_object("realABVStartingBrixInput").unwrap();
    let realABVStartingBrixInputBuffer = realABVStartingBrixInput.get_text().expect("No input");

    let realABVFinalBrixInput: &gtk::Entry = &realABVBuilderClone.get_object("realABVFinalBrixInput").unwrap();
    let realABVFinalBrixInputBuffer = realABVFinalBrixInput.get_text().expect("No input");

    let realABVRealAttenuationOutput = String::from("realABVRealAttenuationOutput");
    let isNumerical = Regex::new(r"^\d+\.\d+|\d+$").unwrap();
    let isCharacter = Regex::new(r"^\D$").unwrap();
    let isMismatched = Regex::new(r"^\d+\D+|\d+\D+\d+$").unwrap();

    if realABVStartingBrixInputBuffer == "" || isNumerical.is_match(&realABVStartingBrixInputBuffer) == false || isCharacter.is_match(&realABVStartingBrixInputBuffer) == true || isMismatched.is_match(&realABVStartingBrixInputBuffer) == true {
        let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
        output.set_text("Enter a number");
    } else if realABVFinalBrixInputBuffer == "" || isNumerical.is_match(&realABVFinalBrixInputBuffer) == false || isCharacter.is_match(&realABVFinalBrixInputBuffer) == true || isMismatched.is_match(&realABVFinalBrixInputBuffer) == true {
        let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        let realABVStartingBrixInputBufferFloat: f32 = realABVStartingBrixInputBuffer.parse().unwrap();
        let realABVFinalBrixInputBufferFloat: f32 = realABVFinalBrixInputBuffer.parse().unwrap();
        if realABVStartingBrixInputBufferFloat <= 0.0 {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if realABVFinalBrixInputBufferFloat <= 0.0 {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if realABVStartingBrixInputBufferFloat < realABVFinalBrixInputBufferFloat {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVRealAttenuationOutput).unwrap();
            output.set_text("Starting brix must be greater than final brix");
        } else {
            realABVMaths(realABVStartingBrixInputBuffer, realABVFinalBrixInputBuffer, &realABVBuilderClone);
        }
    }
}

pub fn realABVMaths(realABVStartingBrixInputBuffer: String, realABVFinalBrixInputBuffer: String, ref realABVBuilderClone: &gtk::Builder) {
    let ref realABVRealAttenuationOutput: &gtk::Entry = &realABVBuilderClone.get_object("realABVRealAttenuationOutput").unwrap();
    let ref realABVFinalABVOutput: &gtk::Entry = &realABVBuilderClone.get_object("realABVFinalABVOutput").unwrap();

    let startingBrix: f32 = realABVStartingBrixInputBuffer.parse().unwrap();
    let finalBrix: f32 = realABVFinalBrixInputBuffer.parse().unwrap();
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