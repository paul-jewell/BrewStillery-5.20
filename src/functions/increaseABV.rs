use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn increaseABVPrep(increaseABVBuilderClone: &gtk::Builder) {
    let increaseABVBrixInput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVBrixInput").unwrap();
    let increaseABVBrixInputBuffer = increaseABVBrixInput.get_text().expect("No input");
    let startingBrix = validInput(&increaseABVBrixInputBuffer);

    let increaseABVABVInput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVABVInput").unwrap();
    let increaseABVABVInputBuffer = increaseABVABVInput.get_text().expect("No input");
    let desiredABV = validInput(&increaseABVABVInputBuffer);

    let increaseABVVolumeInput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVVolumeInput").unwrap();
    let increaseABVVolumeInputBuffer = increaseABVVolumeInput.get_text().expect("No input");
    let desiredWortVolume = validInput(&increaseABVVolumeInputBuffer);

    let increaseABVNewBrixOutput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVNewBrixOutput").unwrap();
    let increaseABVSugarAddOutput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVSugarAddOutput").unwrap();
    let increaseABVHoneyAddOutput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVHoneyAddOutput").unwrap();

    if startingBrix < 2.57 {
        increaseABVNewBrixOutput.set_text("Enter a brix greater than 2.57");
        increaseABVSugarAddOutput.set_text("");
        increaseABVHoneyAddOutput.set_text("");
    } else if startingBrix > 49.48 {
        increaseABVNewBrixOutput.set_text("Enter a brix less than 49.48");
        increaseABVSugarAddOutput.set_text("");
        increaseABVHoneyAddOutput.set_text("");
    } else if startingBrix == 0.0 || desiredABV == 0.0 || desiredWortVolume == 0.0 {
        increaseABVNewBrixOutput.set_text("Enter all 3 inputs");
        increaseABVSugarAddOutput.set_text("");
        increaseABVHoneyAddOutput.set_text("");
    } else if desiredABV <= 0.0 || desiredWortVolume <= 0.0 {
        increaseABVNewBrixOutput.set_text("Enter a positive number");
        increaseABVSugarAddOutput.set_text("");
        increaseABVHoneyAddOutput.set_text("");
    } else {
        onIncreaseActivate(startingBrix, desiredABV, desiredWortVolume, increaseABVBuilderClone);
    }
}

fn onIncreaseActivate(startingBrix: f64, desiredABV: f64, desiredWortVolume: f64, increaseABVBuilderClone: &gtk::Builder) {
    let increaseABVSwitch: &gtk::Switch = &increaseABVBuilderClone.get_object("increaseABVSwitch").unwrap();

    if increaseABVSwitch.get_active() == true {
        let imperialOrMetric = "metric";
        differenceBrixMaths(startingBrix, desiredABV, desiredWortVolume, imperialOrMetric, increaseABVBuilderClone);
    } else if increaseABVSwitch.get_active() == false {
        let imperialOrMetric = "imperial";
        differenceBrixMaths(startingBrix, desiredABV, desiredWortVolume, imperialOrMetric, increaseABVBuilderClone);
    }
}

fn differenceBrixMaths(startingBrix: f64, desiredABV: f64, desiredWortVolume: f64, imperialOrMetric: &str, increaseABVBuilderClone: &gtk::Builder) {
    let mut newStartingBrix = startingBrix;
    let mut newEstimatedABV = realABV(newStartingBrix, finalBrixIdeal).0;

    while newEstimatedABV <= desiredABV {
        newStartingBrix = newStartingBrix + 0.001;
        newEstimatedABV = realABV(newStartingBrix, finalBrixIdeal).0;
    }

    let differenceBrix = newStartingBrix - startingBrix;
    let newSB = format!("{:.2}°Bx", newStartingBrix);

    let increaseABVNewBrixOutput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVNewBrixOutput").unwrap();
    let increaseABVSugarAddOutput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVSugarAddOutput").unwrap();
    let increaseABVHoneyAddOutput: gtk::Entry = increaseABVBuilderClone.get_object("increaseABVHoneyAddOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let sugarToAddImperial = (desiredWortVolume * 1.5 * differenceBrix) / 16.0;
        let honeyToAddImperial = sugarToAddImperial * 1.25;
        let sugar = format!("{} lbs {:.2} oz", sugarToAddImperial as u32, sugarToAddImperial % 1.0 * 16.0);
        let honey = format!("{} lbs {:.2} oz", honeyToAddImperial as u32, honeyToAddImperial % 1.0 * 16.0);
        increaseABVNewBrixOutput.set_text(&newSB);
        increaseABVSugarAddOutput.set_text(&sugar);
        increaseABVHoneyAddOutput.set_text(&honey);
    } else if imperialOrMetric == "metric" {
        let sugarToAddMetric = ((desiredWortVolume / 3.78541) * 1.5 * differenceBrix) * 0.0283495;
        let honeyToAddMetric = sugarToAddMetric * 1.250001102;
        let sugar = format!("{:.2} kilos", sugarToAddMetric);
        let honey = format!("{:.2} kilos", honeyToAddMetric);
        increaseABVNewBrixOutput.set_text(&newSB);
        increaseABVSugarAddOutput.set_text(&sugar);
        increaseABVHoneyAddOutput.set_text(&honey);
    }
}