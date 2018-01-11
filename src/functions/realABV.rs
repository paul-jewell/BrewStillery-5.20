use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn realABVPrep(realABVBuilderClone: &gtk::Builder) {
    let realABVStartingBrixInput: gtk::Entry = realABVBuilderClone.get_object("realABVStartingBrixInput").unwrap();
    let realABVStartingBrixInputBuffer = realABVStartingBrixInput.get_text().expect("No input");
    let startingBrix = validInput(&realABVStartingBrixInputBuffer);

    let realABVFinalBrixInput: gtk::Entry = realABVBuilderClone.get_object("realABVFinalBrixInput").unwrap();
    let realABVFinalBrixInputBuffer = realABVFinalBrixInput.get_text().expect("No input");
    let finalBrix = validInput(&realABVFinalBrixInputBuffer);

    let realABVAttenuationOutput: gtk::Entry = realABVBuilderClone.get_object("realABVAttenuationOutput").unwrap();
    let realABVFinalABVOutput: gtk::Entry = realABVBuilderClone.get_object("realABVFinalABVOutput").unwrap();

    if startingBrix == 0.0 || finalBrix == 0.0 {
        realABVAttenuationOutput.set_text("Enter both inputs");
        realABVFinalABVOutput.set_text("");
    } else if startingBrix <= 0.0 || finalBrix <= 0.0 {
        realABVAttenuationOutput.set_text("Enter a positive number");
        realABVFinalABVOutput.set_text("");
    } else if startingBrix <= finalBrix {
        realABVAttenuationOutput.set_text("Starting Brix must be");
        realABVFinalABVOutput.set_text("greater than Final Brix");
    } else if finalBrix < 2.57 {
        realABVAttenuationOutput.set_text("Enter a Final Brix");
        realABVFinalABVOutput.set_text("greater than 2.57");
    } else if startingBrix > 49.48 {
        realABVAttenuationOutput.set_text("Enter a Starting Brix");
        realABVFinalABVOutput.set_text("less than 49.48");
    } else {
        realABVMaths(startingBrix, finalBrix, realABVBuilderClone);
    }
}

fn realABVMaths(startingBrix: f64, finalBrix: f64, realABVBuilderClone: &gtk::Builder) {
    let realABVAttenuationOutput: gtk::Entry = realABVBuilderClone.get_object("realABVAttenuationOutput").unwrap();
    let realABVFinalABVOutput: gtk::Entry = realABVBuilderClone.get_object("realABVFinalABVOutput").unwrap();

    let abvAndAttenuation = realABV(startingBrix, finalBrix);
    let abv = abvAndAttenuation.0;
    let attenuation = format!("{:.2}%", abvAndAttenuation.1);
    let abvFormatted = format!("{:.2}%", abvAndAttenuation.0);
    if abv > 26.0 || abv <= 0.0 {
        realABVAttenuationOutput.set_text("Enter legimate amounts");
        realABVFinalABVOutput.set_text("");
    } else {
        realABVFinalABVOutput.set_text(&abvFormatted);
        realABVAttenuationOutput.set_text(&attenuation);
    }
}