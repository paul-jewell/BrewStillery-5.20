use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn grainABVPrep(ref grainABVBuilderClone: &gtk::Builder) {
    let grainABVWortInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVWortInput").unwrap();
    let grainABVWortInputBuffer = grainABVWortInput.get_text().expect("No input");
    let grainABVWortInputBufferFloat = validInput(&grainABVWortInputBuffer);

    let grainABVFirstAmountInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVFirstAmountInput").unwrap();
    let grainABVFirstAmountInputBuffer = grainABVFirstAmountInput.get_text().expect("No input");
    let grainABVFirstAmountInputBufferFloat = validInput(&grainABVFirstAmountInputBuffer);

    let grainABVFirstTypeInput: &gtk::ComboBoxText = &grainABVBuilderClone.get_object("grainABVFirstTypeInput").unwrap();
    let grainABVFirstTypeInputBuffer = grainABVFirstTypeInput.get_active_id().unwrap();
    let grainABVFirstTypeInputBufferFloat = grainInfo(&grainABVFirstTypeInputBuffer);

    let grainABVSecondAmountInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVSecondAmountInput").unwrap();
    let grainABVSecondAmountInputBuffer = grainABVSecondAmountInput.get_text().expect("No input");
    let grainABVSecondAmountInputBufferFloat = validInput(&grainABVSecondAmountInputBuffer);

    let grainABVSecondTypeInput: &gtk::ComboBoxText = &grainABVBuilderClone.get_object("grainABVSecondTypeInput").unwrap();
    let grainABVSecondTypeInputBuffer = grainABVSecondTypeInput.get_active_id().unwrap();
    let grainABVSecondTypeInputBufferFloat = grainInfo(&grainABVSecondTypeInputBuffer);

    let grainABVThirdAmountInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVThirdAmountInput").unwrap();
    let grainABVThirdAmountInputBuffer = grainABVThirdAmountInput.get_text().expect("No input");
    let grainABVThirdAmountInputBufferFloat = validInput(&grainABVThirdAmountInputBuffer);

    let grainABVThirdTypeInput: &gtk::ComboBoxText = &grainABVBuilderClone.get_object("grainABVThirdTypeInput").unwrap();
    let grainABVThirdTypeInputBuffer = grainABVThirdTypeInput.get_active_id().unwrap();
    let grainABVThirdTypeInputBufferFloat = grainInfo(&grainABVThirdTypeInputBuffer);

    let grainABVFourthAmountInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVFourthAmountInput").unwrap();
    let grainABVFourthAmountInputBuffer = grainABVFourthAmountInput.get_text().expect("No input");
    let grainABVFourthAmountInputBufferFloat = validInput(&grainABVFourthAmountInputBuffer);

    let grainABVFourthTypeInput: &gtk::ComboBoxText = &grainABVBuilderClone.get_object("grainABVFourthTypeInput").unwrap();
    let grainABVFourthTypeInputBuffer = grainABVFourthTypeInput.get_active_id().unwrap();
    let grainABVFourthTypeInputBufferFloat = grainInfo(&grainABVFourthTypeInputBuffer);

    let grainABVFifthAmountInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVFifthAmountInput").unwrap();
    let grainABVFifthAmountInputBuffer = grainABVFifthAmountInput.get_text().expect("No input");
    let grainABVFifthAmountInputBufferFloat = validInput(&grainABVFifthAmountInputBuffer);

    let grainABVFifthTypeInput: &gtk::ComboBoxText = &grainABVBuilderClone.get_object("grainABVFifthTypeInput").unwrap();
    let grainABVFifthTypeInputBuffer = grainABVFifthTypeInput.get_active_id().unwrap();
    let grainABVFifthTypeInputBufferFloat = grainInfo(&grainABVFifthTypeInputBuffer);

    let grainABVSixthAmountInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVSixthAmountInput").unwrap();
    let grainABVSixthAmountInputBuffer = grainABVSixthAmountInput.get_text().expect("No input");
    let grainABVSixthAmountInputBufferFloat = validInput(&grainABVSixthAmountInputBuffer);

    let grainABVSixthTypeInput: &gtk::ComboBoxText = &grainABVBuilderClone.get_object("grainABVSixthTypeInput").unwrap();
    let grainABVSixthTypeInputBuffer = grainABVSixthTypeInput.get_active_id().unwrap();
    let grainABVSixthTypeInputBufferFloat = grainInfo(&grainABVSixthTypeInputBuffer);

    let grainABVSeventhAmountInput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVSeventhAmountInput").unwrap();
    let grainABVSeventhAmountInputBuffer = grainABVSeventhAmountInput.get_text().expect("No input");
    let grainABVSeventhAmountInputBufferFloat = validInput(&grainABVSeventhAmountInputBuffer);

    let grainABVSeventhTypeInput: &gtk::ComboBoxText = &grainABVBuilderClone.get_object("grainABVSeventhTypeInput").unwrap();
    let grainABVSeventhTypeInputBuffer = grainABVSeventhTypeInput.get_active_id().unwrap();
    let grainABVSeventhTypeInputBufferFloat = grainInfo(&grainABVSeventhTypeInputBuffer);

    let grainABVBrixOutput = String::from("grainABVBrixOutput");

    if grainABVWortInputBufferFloat == 0.0 {
        let output: gtk::Entry = grainABVBuilderClone.get_object(&grainABVBrixOutput).unwrap();
        output.set_text("Enter at least the first two inputs");
    } else if grainABVFirstAmountInputBufferFloat == 0.0 {
        let output: gtk::Entry = grainABVBuilderClone.get_object(&grainABVBrixOutput).unwrap();
        output.set_text("Enter at least one grain amount");
    } else {
        if grainABVWortInputBufferFloat <= 0.0 {
            let output: gtk::Entry = grainABVBuilderClone.get_object(&grainABVBrixOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if grainABVFirstAmountInputBufferFloat <= 0.0 {
            let output: gtk::Entry = grainABVBuilderClone.get_object(&grainABVBrixOutput).unwrap();
            output.set_text("Enter a positive number");
        } else {
            onGrainABVActivate(grainABVWortInputBufferFloat, grainABVFirstAmountInputBufferFloat, grainABVFirstTypeInputBufferFloat, grainABVSecondAmountInputBufferFloat, grainABVSecondTypeInputBufferFloat, grainABVThirdAmountInputBufferFloat, grainABVThirdTypeInputBufferFloat, grainABVFourthAmountInputBufferFloat, grainABVFourthTypeInputBufferFloat, grainABVFifthAmountInputBufferFloat, grainABVFifthTypeInputBufferFloat, grainABVSixthAmountInputBufferFloat, grainABVSixthTypeInputBufferFloat, grainABVSeventhAmountInputBufferFloat, grainABVSeventhTypeInputBufferFloat, &grainABVBuilderClone);
            }
        }
    }

fn onGrainABVActivate(grainABVWortInputBufferFloat: f32, grainABVFirstAmountInputBufferFloat: f32, grainABVFirstTypeInputBufferFloat: f32, grainABVSecondAmountInputBufferFloat: f32, grainABVSecondTypeInputBufferFloat: f32, grainABVThirdAmountInputBufferFloat: f32, grainABVThirdTypeInputBufferFloat: f32, grainABVFourthAmountInputBufferFloat: f32, grainABVFourthTypeInputBufferFloat: f32, grainABVFifthAmountInputBufferFloat: f32, grainABVFifthTypeInputBufferFloat: f32, grainABVSixthAmountInputBufferFloat: f32, grainABVSixthTypeInputBufferFloat: f32, grainABVSeventhAmountInputBufferFloat: f32, grainABVSeventhTypeInputBufferFloat: f32, ref grainABVBuilderClone: &gtk::Builder) {
    let ref grainABVSwitch: &gtk::Switch = &grainABVBuilderClone.get_object("grainABVSwitch").unwrap();

    if grainABVSwitch.get_active() == true {
        let imperialOrMetric = String::from("metric");
        grainABVMaths(grainABVWortInputBufferFloat, grainABVFirstAmountInputBufferFloat, grainABVFirstTypeInputBufferFloat, grainABVSecondAmountInputBufferFloat, grainABVSecondTypeInputBufferFloat, grainABVThirdAmountInputBufferFloat, grainABVThirdTypeInputBufferFloat, grainABVFourthAmountInputBufferFloat, grainABVFourthTypeInputBufferFloat, grainABVFifthAmountInputBufferFloat, grainABVFifthTypeInputBufferFloat, grainABVSixthAmountInputBufferFloat, grainABVSixthTypeInputBufferFloat, grainABVSeventhAmountInputBufferFloat, grainABVSeventhTypeInputBufferFloat, imperialOrMetric, &grainABVBuilderClone);
    } else if grainABVSwitch.get_active() == false {
        let imperialOrMetric = String::from("imperial");
        grainABVMaths(grainABVWortInputBufferFloat, grainABVFirstAmountInputBufferFloat, grainABVFirstTypeInputBufferFloat, grainABVSecondAmountInputBufferFloat, grainABVSecondTypeInputBufferFloat, grainABVThirdAmountInputBufferFloat, grainABVThirdTypeInputBufferFloat, grainABVFourthAmountInputBufferFloat, grainABVFourthTypeInputBufferFloat, grainABVFifthAmountInputBufferFloat, grainABVFifthTypeInputBufferFloat, grainABVSixthAmountInputBufferFloat, grainABVSixthTypeInputBufferFloat, grainABVSeventhAmountInputBufferFloat, grainABVSeventhTypeInputBufferFloat, imperialOrMetric, &grainABVBuilderClone);
    }
}

fn grainABVMaths(grainABVWortInputBufferFloat: f32, grainABVFirstAmountInputBufferFloat: f32, grainABVFirstTypeInputBuffer: f32, grainABVSecondAmountInputBufferFloat: f32, grainABVSecondTypeInputBuffer: f32, grainABVThirdAmountInputBufferFloat: f32, grainABVThirdTypeInputBuffer: f32, grainABVFourthAmountInputBufferFloat: f32, grainABVFourthTypeInputBuffer: f32, grainABVFifthAmountInputBufferFloat: f32, grainABVFifthTypeInputBuffer: f32, grainABVSixthAmountInputBufferFloat: f32, grainABVSixthTypeInputBuffer: f32, grainABVSeventhAmountInputBufferFloat: f32, grainABVSeventhTypeInputBuffer: f32, imperialOrMetric: String, ref grainABVBuilderClone: &gtk::Builder) {
    let ref grainABVBrixOutput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVBrixOutput").unwrap();
    let ref grainABVABVOutput: &gtk::Entry = &grainABVBuilderClone.get_object("grainABVABVOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let totalGrain1 = grainToABV(grainABVWortInputBufferFloat, grainABVFirstAmountInputBufferFloat, grainABVFirstTypeInputBuffer);
        let totalGrain2 = grainToABV(grainABVWortInputBufferFloat, grainABVSecondAmountInputBufferFloat, grainABVSecondTypeInputBuffer);
        let totalGrain3 = grainToABV(grainABVWortInputBufferFloat, grainABVThirdAmountInputBufferFloat, grainABVThirdTypeInputBuffer);
        let totalGrain4 = grainToABV(grainABVWortInputBufferFloat, grainABVFourthAmountInputBufferFloat, grainABVFourthTypeInputBuffer);
        let totalGrain5 = grainToABV(grainABVWortInputBufferFloat, grainABVFifthAmountInputBufferFloat, grainABVFifthTypeInputBuffer);
        let totalGrain6 = grainToABV(grainABVWortInputBufferFloat, grainABVSixthAmountInputBufferFloat, grainABVSixthTypeInputBuffer);
        let totalGrain7 = grainToABV(grainABVWortInputBufferFloat, grainABVSeventhAmountInputBufferFloat, grainABVSeventhTypeInputBuffer);
        let estimatedBrix = format!("{:.2}°Bx", gravityToBrix(1.0 + &totalGrain1 % 1.0 + &totalGrain2 % 1.0 + &totalGrain3 % 1.0 + &totalGrain4 % 1.0 + &totalGrain5 % 1.0 + &totalGrain6 % 1.0 + &totalGrain7 % 1.0));
        let abv = format!("{:.2}%", guestimateABV(1.0 + &totalGrain1 % 1.0 + &totalGrain2 % 1.0 + &totalGrain3 % 1.0 + &totalGrain4 % 1.0 + &totalGrain5 % 1.0 + &totalGrain6 % 1.0 + &totalGrain7 % 1.0));
        grainABVBrixOutput.set_text(&estimatedBrix);
        grainABVABVOutput.set_text(&abv);
    } else if imperialOrMetric == "metric" {
        let totalGrain1 = grainToABV(litresToGallons(grainABVWortInputBufferFloat), kilosToPounds(grainABVFirstAmountInputBufferFloat), grainABVFirstTypeInputBuffer);
        let totalGrain2 = grainToABV(litresToGallons(grainABVWortInputBufferFloat), kilosToPounds(grainABVSecondAmountInputBufferFloat), grainABVSecondTypeInputBuffer);
        let totalGrain3 = grainToABV(litresToGallons(grainABVWortInputBufferFloat), kilosToPounds(grainABVThirdAmountInputBufferFloat), grainABVThirdTypeInputBuffer);
        let totalGrain4 = grainToABV(litresToGallons(grainABVWortInputBufferFloat), kilosToPounds(grainABVFourthAmountInputBufferFloat), grainABVFourthTypeInputBuffer);
        let totalGrain5 = grainToABV(litresToGallons(grainABVWortInputBufferFloat), kilosToPounds(grainABVFifthAmountInputBufferFloat), grainABVFifthTypeInputBuffer);
        let totalGrain6 = grainToABV(litresToGallons(grainABVWortInputBufferFloat), kilosToPounds(grainABVSixthAmountInputBufferFloat), grainABVSixthTypeInputBuffer);
        let totalGrain7 = grainToABV(litresToGallons(grainABVWortInputBufferFloat), kilosToPounds(grainABVSeventhAmountInputBufferFloat), grainABVSeventhTypeInputBuffer);
        let estimatedBrix = format!("{:.2}°Bx", gravityToBrix(1.0 + &totalGrain1 % 1.0 + &totalGrain2 % 1.0 + &totalGrain3 % 1.0 + &totalGrain4 % 1.0 + &totalGrain5 % 1.0 + &totalGrain6 % 1.0 + &totalGrain7 % 1.0));
        let abv = format!("{:.2}%", guestimateABV(1.0 + &totalGrain1 % 1.0 + &totalGrain2 % 1.0 + &totalGrain3 % 1.0 + &totalGrain4 % 1.0 + &totalGrain5 % 1.0 + &totalGrain6 % 1.0 + &totalGrain7 % 1.0));
        grainABVBrixOutput.set_text(&estimatedBrix);
        grainABVABVOutput.set_text(&abv);
    }
}