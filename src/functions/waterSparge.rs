#![allow(non_snake_case)]

use gtk::{self};
use gtk::prelude::*;
use regex::Regex;

pub fn waterSpargePrep(ref waterSpargeBuilderClone: &gtk::Builder) {
    let spargePreFermentVolumeInput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargePreFermentVolumeInput").unwrap();
    let spargePreFermentVolumeInputBuffer = spargePreFermentVolumeInput.get_text().expect("No input");

    let spargeTotalGrainInput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeTotalGrainInput").unwrap();
    let spargeTotalGrainInputBuffer = spargeTotalGrainInput.get_text().expect("No input");

    let spargeBoilTimeInput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeBoilTimeInput").unwrap();
    let spargeBoilTimeInputBuffer = spargeBoilTimeInput.get_text().expect("No input");

    let spargeMashWaterOutput = String::from("spargeMashWaterOutput");
    let isNumerical = Regex::new(r"^\d+\.\d+|\d+$").unwrap();
    let isCharacter = Regex::new(r"^\D$").unwrap();
    let isMismatched = Regex::new(r"^\d+\D+|\d+\D+\d+$").unwrap();

    if spargePreFermentVolumeInputBuffer == "" || isNumerical.is_match(&spargePreFermentVolumeInputBuffer) == false || isCharacter.is_match(&spargePreFermentVolumeInputBuffer) == true || isMismatched.is_match(&spargePreFermentVolumeInputBuffer) == true {
        let output: gtk::Entry = waterSpargeBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
        output.set_text("Enter a number");
    } else if spargeTotalGrainInputBuffer == "" || isNumerical.is_match(&spargeTotalGrainInputBuffer) == false || isCharacter.is_match(&spargeTotalGrainInputBuffer) == true || isMismatched.is_match(&spargeTotalGrainInputBuffer) == true {
        let output: gtk::Entry = waterSpargeBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
        output.set_text("Enter a number");
    } else if spargeBoilTimeInputBuffer == "" || isNumerical.is_match(&spargeBoilTimeInputBuffer) == false || isCharacter.is_match(&spargeBoilTimeInputBuffer) == true || isMismatched.is_match(&spargeBoilTimeInputBuffer) == true {
        let output: gtk::Entry = waterSpargeBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        let spargePreFermentVolumeInputBufferFloat: f32 = spargePreFermentVolumeInputBuffer.parse().unwrap();
        let spargeTotalGrainInputBufferFloat: f32 = spargeTotalGrainInputBuffer.parse().unwrap();
        let spargeBoilTimeInputBufferFloat: f32 = spargeBoilTimeInputBuffer.parse().unwrap();
        if spargePreFermentVolumeInputBufferFloat <= 0.0 {
            let output: gtk::Entry = waterSpargeBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if spargeTotalGrainInputBufferFloat <= 0.0 {
            let output: gtk::Entry = waterSpargeBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if spargeBoilTimeInputBufferFloat <= 0.0 {
            let output: gtk::Entry = waterSpargeBuilderClone.get_object(&spargeMashWaterOutput).unwrap();
            output.set_text("Enter a positive number");
        } else {
            onSpargeActivate(spargePreFermentVolumeInputBuffer, spargeTotalGrainInputBuffer, spargeBoilTimeInputBuffer, &waterSpargeBuilderClone);
        }
    }
}

pub fn onSpargeActivate(spargePreFermentVolumeInputBuffer: String, spargeTotalGrainInputBuffer: String, spargeBoilTimeInputBuffer: String, ref waterSpargeBuilderClone: &gtk::Builder) {
    let ref waterSpargeSwitch: &gtk::Switch = &waterSpargeBuilderClone.get_object("waterSpargeSwitch").unwrap();

    let preFermentVolume: f32 = spargePreFermentVolumeInputBuffer.parse().unwrap();
    let totalGrain: f32 = spargeTotalGrainInputBuffer.parse().unwrap();
    let boilTemp: f32 = spargeBoilTimeInputBuffer.parse().unwrap();
    let boilTime: f32 = boilTemp / 60.0;

    if waterSpargeSwitch.get_active() == true {
        let imperialOrMetric = String::from("metric");
        let grainAbsorption: f32 = 1.25181176;
        // constant value of 1.25181176 litres/kilo
        // 0.15 gal = 0.5678118 L
        // 1 lb  = 0.453592 kg
        let mashThickness: f32 = 2.781108353;
        // 1.333 quarts = 1.2614885 L
        // 1 lb  = 0.453592 kg
        // 2.781108353 litres/kilo
        waterSpargeMaths(preFermentVolume, totalGrain, boilTime, grainAbsorption, mashThickness, imperialOrMetric, &waterSpargeBuilderClone);
    } else if waterSpargeSwitch.get_active() == false {
        let imperialOrMetric = String::from("imperial");
        let grainAbsorption: f32 = 0.15;
        // constant value of 0.15 gallons/lb
        let mashThickness: f32 = 1.333;
        // 1.333 quarts/lb
        waterSpargeMaths(preFermentVolume, totalGrain, boilTime, grainAbsorption, mashThickness, imperialOrMetric, &waterSpargeBuilderClone);
    }
}

fn waterSpargeMaths(preFermentVolume: f32, totalGrain: f32, boilTime: f32, grainAbsorption: f32, mashThickness: f32, imperialOrMetric: String, ref waterSpargeBuilderClone: &gtk::Builder) {
    let wortShrinkage: f32 = 0.04;
    // constant value of 4%
    let percentBoiloff: f32 = 0.1;
    // constant value of 10%
    let trubLoss = preFermentVolume * 0.05;
    // 5% is an acceptable norm
    let equipmentLoss = preFermentVolume * 0.08;
    // 8% is an acceptable norm

    let totalWater = (((preFermentVolume + trubLoss) / (1.0 - wortShrinkage)) / (1.0 - (boilTime * percentBoiloff))) + equipmentLoss + (totalGrain * grainAbsorption);
    let mashWater = (totalGrain * mashThickness)/4.0;
    let spargeWater = totalWater - mashWater;

    let ref spargeMashWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeMashWaterOutput").unwrap();
    let ref spargeSpargeWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeSpargeWaterOutput").unwrap();
    let ref spargeTotalWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeTotalWaterOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let mash = format!("{:.2} gallons", mashWater);
        let sparge = format!("{:.2} gallons", spargeWater);
        let total = format!("{:.2} gallons", totalWater);
        spargeMashWaterOutput.set_text(&mash);
        spargeSpargeWaterOutput.set_text(&sparge);
        spargeTotalWaterOutput.set_text(&total);
    } else if imperialOrMetric == "metric" {
        let mash = format!("{:.2} litres", mashWater);
        let sparge = format!("{:.2} litres", spargeWater);
        let total = format!("{:.2} litres", totalWater);
        spargeMashWaterOutput.set_text(&mash);
        spargeSpargeWaterOutput.set_text(&sparge);
        spargeTotalWaterOutput.set_text(&total);
    }
}