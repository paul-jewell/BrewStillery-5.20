use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn waterSpargePrep(waterSpargeBuilderClone: &gtk::Builder) {
    let spargePreFermentVolumeInput: gtk::Entry = waterSpargeBuilderClone.get_object("spargePreFermentVolumeInput").unwrap();
    let spargePreFermentVolumeInputBuffer = spargePreFermentVolumeInput.get_text().expect("No input");
    let preFermentVolume = validInput(&spargePreFermentVolumeInputBuffer);

    let spargeTotalGrainInput: gtk::Entry = waterSpargeBuilderClone.get_object("spargeTotalGrainInput").unwrap();
    let spargeTotalGrainInputBuffer = spargeTotalGrainInput.get_text().expect("No input");
    let totalGrain = validInput(&spargeTotalGrainInputBuffer);

    let spargeBoilTimeInput: gtk::Entry = waterSpargeBuilderClone.get_object("spargeBoilTimeInput").unwrap();
    let spargeBoilTimeInputBuffer = spargeBoilTimeInput.get_text().expect("No input");
    let boilTemp = validInput(&spargeBoilTimeInputBuffer);

    let spargeMashWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeMashWaterOutput").unwrap();
    let spargeSpargeWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeSpargeWaterOutput").unwrap();
    let spargeTotalWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeTotalWaterOutput").unwrap();

    if preFermentVolume < 0.0  || totalGrain < 0.0 || boilTemp < 0.0 {
        spargeMashWaterOutput.set_text("Enter a positive number");
        spargeSpargeWaterOutput.set_text("");
        spargeTotalWaterOutput.set_text("");
    } else if preFermentVolume == 0.0  || totalGrain == 0.0 || boilTemp == 0.0 {
        spargeMashWaterOutput.set_text("Enter all 3 inputs");
        spargeSpargeWaterOutput.set_text("");
        spargeTotalWaterOutput.set_text("");
    } else {
        onSpargeActivate(preFermentVolume, totalGrain, boilTemp, waterSpargeBuilderClone);
    }
}

fn onSpargeActivate(preFermentVolume: f64, totalGrain: f64, boilTemp: f64, waterSpargeBuilderClone: &gtk::Builder) {
    let waterSpargeSwitch: gtk::Switch = waterSpargeBuilderClone.get_object("waterSpargeSwitch").unwrap();

    let boilTime: f64 = boilTemp / 60.0;

    if waterSpargeSwitch.get_active() == true {
        let imperialOrMetric = "metric";
        let grainAbsorption: f64 = 1.25181176;
        // constant value of 1.25181176 litres/kilo
        // 0.15 gal = 0.5678118 L
        // 1 lb  = 0.453592 kg
        let mashThickness: f64 = 2.781108353;
        // 1.333 quarts = 1.2614885 L
        // 1 lb  = 0.453592 kg
        // 2.781108353 litres/kilo
        waterSpargeMaths(preFermentVolume, totalGrain, boilTime, grainAbsorption, mashThickness, imperialOrMetric, waterSpargeBuilderClone);
    } else if waterSpargeSwitch.get_active() == false {
        let imperialOrMetric = "imperial";
        let grainAbsorption: f64 = 0.15;
        // constant value of 0.15 gallons/lb
        let mashThickness: f64 = 1.333;
        // 1.333 quarts/lb
        waterSpargeMaths(preFermentVolume, totalGrain, boilTime, grainAbsorption, mashThickness, imperialOrMetric, waterSpargeBuilderClone);
    }
}

fn waterSpargeMaths(preFermentVolume: f64, totalGrain: f64, boilTime: f64, grainAbsorption: f64, mashThickness: f64, imperialOrMetric: &str, waterSpargeBuilderClone: &gtk::Builder) {
    let wortShrinkage: f64 = 0.04;
    // constant value of 4%
    let percentBoiloff: f64 = 0.1;
    // constant value of 10%
    let trubLoss = preFermentVolume * 0.05;
    // 5% is an acceptable norm
    let equipmentLoss = preFermentVolume * 0.08;
    // 8% is an acceptable norm

    let totalWater = (((preFermentVolume + trubLoss) / (1.0 - wortShrinkage)) / (1.0 - (boilTime * percentBoiloff))) + equipmentLoss + (totalGrain * grainAbsorption);
    let mashWater = (totalGrain * mashThickness)/4.0;
    let spargeWater = totalWater - mashWater;

    let spargeMashWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeMashWaterOutput").unwrap();
    let spargeSpargeWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeSpargeWaterOutput").unwrap();
    let spargeTotalWaterOutput: &gtk::Entry = &waterSpargeBuilderClone.get_object("spargeTotalWaterOutput").unwrap();

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