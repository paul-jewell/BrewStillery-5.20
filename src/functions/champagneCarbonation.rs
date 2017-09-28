#![allow(non_snake_case)]

use gtk::{self};
use gtk::prelude::*;
use regex::Regex;

pub fn champagneCarbonationPrep(ref champagneCarbonationBuilderClone: &gtk::Builder) {
    let champagneCarbonationInput: &gtk::Entry = &champagneCarbonationBuilderClone.get_object("champagneCarbonationInput").unwrap();
    let champagneCarbonationInputBuffer = champagneCarbonationInput.get_text().expect("No input");

    let champagneCarbonationOutput = String::from("champagneCarbonationOutput");
    let isNumerical = Regex::new(r"^\d+\.\d+|\d+$").unwrap();
    let isCharacter = Regex::new(r"^\D$").unwrap();
    let isMismatched = Regex::new(r"^\d+\D+|\d+\D+\d+$").unwrap();

    if champagneCarbonationInputBuffer == "" || isNumerical.is_match(&champagneCarbonationInputBuffer) == false || isCharacter.is_match(&champagneCarbonationInputBuffer) == true || isMismatched.is_match(&champagneCarbonationInputBuffer) == true {
        let output: gtk::Entry = champagneCarbonationBuilderClone.get_object(&champagneCarbonationOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        let champagneCarbonationInputBufferFloat: f32 = champagneCarbonationInputBuffer.parse().unwrap();
        if champagneCarbonationInputBufferFloat <= 0.0 {
            let output: gtk::Entry = champagneCarbonationBuilderClone.get_object(&champagneCarbonationOutput).unwrap();
            output.set_text("Enter a positive number");
        } else {
            onChampagneActivate(champagneCarbonationInputBuffer, &champagneCarbonationBuilderClone);
        }
    }
}

pub fn onChampagneActivate(champagneCarbonationInputBuffer: String, ref champagneCarbonationBuilderClone: &gtk::Builder) {
    let ref champagneCarbonationSwitch: &gtk::Switch = &champagneCarbonationBuilderClone.get_object("champagneCarbonationSwitch").unwrap();

    let champagneVolume: f32 = champagneCarbonationInputBuffer.parse().unwrap();

    if champagneCarbonationSwitch.get_active() == true {
        let imperialOrMetric = String::from("metric");
        champagneCarbonationMaths(champagneVolume, imperialOrMetric, &champagneCarbonationBuilderClone);
    } else if champagneCarbonationSwitch.get_active() == false {
        let imperialOrMetric = String::from("imperial");
        champagneCarbonationMaths(champagneVolume, imperialOrMetric, &champagneCarbonationBuilderClone);
    }
}

fn champagneCarbonationMaths(champagneVolume: f32, imperialOrMetric: String, ref champagneCarbonationBuilderClone: &gtk::Builder) {
    let ref champagneCarbonationOutput: &gtk::Entry = &champagneCarbonationBuilderClone.get_object("champagneCarbonationOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let totalSugar = champagneVolume * 0.2;
        if totalSugar == 1.0 {
            let sugar = format!("{:.0} pound", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        } else {
            let sugar = format!("{:.2} pounds", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        }
    } else if imperialOrMetric == "metric" {
        let totalSugar = (champagneVolume * 23.986897025) / 1000.0;
        if totalSugar == 1.0 {
            let sugar = format!("{:.0} kilo", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        } else {
            let sugar = format!("{:.2} kilos", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        }
    }
}