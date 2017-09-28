#![allow(non_snake_case)]

use gtk::{self};
use gtk::prelude::*;
use regex::Regex;
use functions::commonFunctions::*;

pub fn gyleCarbonationPrep(ref gyleBuilderClone: &gtk::Builder) {
    let gyleBrixInput: &gtk::Entry = &gyleBuilderClone.get_object("gyleBrixInput").unwrap();
    let gyleBrixInputBuffer = gyleBrixInput.get_text().expect("No input");

    let gyleCO2Input: &gtk::Entry = &gyleBuilderClone.get_object("gyleCO2Input").unwrap();
    let gyleCO2InputBuffer = gyleCO2Input.get_text().expect("No input");

    let gyleFinalVolumeInput: &gtk::Entry = &gyleBuilderClone.get_object("gyleFinalVolumeInput").unwrap();
    let gyleFinalVolumeInputBuffer = gyleFinalVolumeInput.get_text().expect("No input");



    let spargeMashWaterOutput = String::from("spargeMashWaterOutput");
    let isNumerical = Regex::new(r"^\d+\.\d+|\d+$").unwrap();
    let isCharacter = Regex::new(r"^\D$").unwrap();
    let isMismatched = Regex::new(r"^\d+\D+|\d+\D+\d+$").unwrap();

    if gyleBrixInputBuffer == "" || isNumerical.is_match(&gyleBrixInputBuffer) == false || isCharacter.is_match(&gyleBrixInputBuffer) == true || isMismatched.is_match(&gyleBrixInputBuffer) == true {
        let output: gtk::Entry = gyleBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
        output.set_text("Enter a number");
    } else if gyleCO2InputBuffer == "" || isNumerical.is_match(&gyleCO2InputBuffer) == false || isCharacter.is_match(&gyleCO2InputBuffer) == true || isMismatched.is_match(&gyleCO2InputBuffer) == true {
        let output: gtk::Entry = gyleBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
        output.set_text("Enter a number");
    } else if gyleFinalVolumeInputBuffer == "" || isNumerical.is_match(&gyleFinalVolumeInputBuffer) == false || isCharacter.is_match(&gyleFinalVolumeInputBuffer) == true || isMismatched.is_match(&gyleFinalVolumeInputBuffer) == true {
        let output: gtk::Entry = gyleBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        let gyleBrixInputBufferFloat: f32 = gyleBrixInputBuffer.parse().unwrap();
        let gyleCO2InputBufferFloat: f32 = gyleCO2InputBuffer.parse().unwrap();
        let gyleFinalVolumeInputBufferFloat: f32 = gyleFinalVolumeInputBuffer.parse().unwrap();
        if gyleBrixInputBufferFloat <= 0.0 {
            let output: gtk::Entry = gyleBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if gyleCO2InputBufferFloat <= 0.0 {
            let output: gtk::Entry = gyleBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if gyleFinalVolumeInputBufferFloat <= 0.0 {
            let output: gtk::Entry = gyleBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
            output.set_text("Enter a positive number");
        } else {
            onGyleActivate(gyleBrixInputBuffer, gyleCO2InputBuffer, gyleFinalVolumeInputBuffer, &gyleBuilderClone);
        }
    }
}

pub fn onGyleActivate(startingBrixBuffer: String, desiredCO2LevelBuffer: String, finalVolumeBuffer: String, ref gyleBuilderClone: &gtk::Builder) {
    let ref gyleCarbonationSwitch: &gtk::Switch = &gyleBuilderClone.get_object("gyleCarbonationSwitch").unwrap();

    let startingBrix: f32 = startingBrixBuffer.parse().unwrap();
    let desiredCO2Level: f32 = desiredCO2LevelBuffer.parse().unwrap();
    let finalVolume: f32 = finalVolumeBuffer.parse().unwrap();

    if gyleCarbonationSwitch.get_active() == true {
        let imperialOrMetric = String::from("metric");
        gyleMaths(startingBrix, desiredCO2Level, finalVolume, imperialOrMetric, &gyleBuilderClone);
    } else if gyleCarbonationSwitch.get_active() == false {
        let imperialOrMetric = String::from("imperial");
        gyleMaths(startingBrix, desiredCO2Level, finalVolume, imperialOrMetric, &gyleBuilderClone);
    }
}

fn gyleMaths(startingBrix: f32, desiredCO2Level: f32, finalVolume: f32, imperialOrMetric: String, ref gyleBuilderClone: &gtk::Builder) {
    let startingGravity = brixToGravity(startingBrix);
    let finalGravity: f32 = 1.015;
    let startingPlato = gravityToPlato(startingGravity);
    let finalPlato = gravityToPlato(finalGravity);


    let ref gyleOutput: &gtk::Entry = &gyleBuilderClone.get_object("gyleOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let gyleVolume = ((0.24 * finalVolume * 3.7854 * desiredCO2Level) / (startingPlato - finalPlato)) * 0.2641729;
        let total = format!("{:.2} gallons", &gyleVolume);
        gyleOutput.set_text(&total);
    } else if imperialOrMetric == "metric" {
        let gyleVolume = (0.24 * finalVolume * desiredCO2Level) / (startingPlato - finalPlato);
        let total = format!("{:.2} litres", &gyleVolume);
        gyleOutput.set_text(&total);
    }
}