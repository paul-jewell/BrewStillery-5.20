use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn guestimateIBUPrep(ref IBUBuilderClone: &gtk::Builder) {
    let totalIBUPreBoilBrixInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUPreBoilBrixInput").unwrap();
    let totalIBUPreBoilBrixInputBuffer = totalIBUPreBoilBrixInput.get_text().expect("No input");
    let totalIBUPreBoilBrixInputBufferFloat = validInput(&totalIBUPreBoilBrixInputBuffer);

    let totalIBUWortVolumeInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUWortVolumeInput").unwrap();
    let totalIBUWortVolumeInputBuffer = totalIBUWortVolumeInput.get_text().expect("No input");
    let totalIBUWortVolumeInputBufferFloat = validInput(&totalIBUWortVolumeInputBuffer);

    let totalIBUBoilTimeInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUBoilTimeInput").unwrap();
    let totalIBUBoilTimeInputBuffer = totalIBUBoilTimeInput.get_text().expect("No input");
    let totalIBUBoilTimeInputBufferFloat = validInput(&totalIBUBoilTimeInputBuffer);

    let totalIBUFirstHopAlphaInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUFirstHopAlphaInput").unwrap();
    let totalIBUFirstHopAlphaInputBuffer = totalIBUFirstHopAlphaInput.get_text().expect("No input");
    let totalIBUFirstHopAlphaInputBufferFloat = validInput(&totalIBUFirstHopAlphaInputBuffer);

    let totalIBUFirstHopAmountInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUFirstHopAmountInput").unwrap();
    let totalIBUFirstHopAmountInputBuffer = totalIBUFirstHopAmountInput.get_text().expect("No input");
    let totalIBUFirstHopAmountInputBufferFloat = validInput(&totalIBUFirstHopAmountInputBuffer);

    let totalIBUSecondHopAlphaInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUSecondHopAlphaInput").unwrap();
    let totalIBUSecondHopAlphaInputBuffer = totalIBUSecondHopAlphaInput.get_text().expect("No input");
    let totalIBUSecondHopAlphaInputBufferFloat = validInput(&totalIBUSecondHopAlphaInputBuffer);

    let totalIBUSecondHopAmountInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUSecondHopAmountInput").unwrap();
    let totalIBUSecondHopAmountInputBuffer = totalIBUSecondHopAmountInput.get_text().expect("No input");
    let totalIBUSecondHopAmountInputBufferFloat = validInput(&totalIBUSecondHopAmountInputBuffer);

    let totalIBUThirdHopAlphaInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUThirdHopAlphaInput").unwrap();
    let totalIBUThirdHopAlphaInputBuffer = totalIBUThirdHopAlphaInput.get_text().expect("No input");
    let totalIBUThirdHopAlphaInputBufferFloat = validInput(&totalIBUThirdHopAlphaInputBuffer);

    let totalIBUThirdHopAmountInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUThirdHopAmountInput").unwrap();
    let totalIBUThirdHopAmountInputBuffer = totalIBUThirdHopAmountInput.get_text().expect("No input");
    let totalIBUThirdHopAmountInputBufferFloat = validInput(&totalIBUThirdHopAmountInputBuffer);

    let totalIBUFourthHopAlphaInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUFourthHopAlphaInput").unwrap();
    let totalIBUFourthHopAlphaInputBuffer = totalIBUFourthHopAlphaInput.get_text().expect("No input");
    let totalIBUFourthHopAlphaInputBufferFloat = validInput(&totalIBUFourthHopAlphaInputBuffer);

    let totalIBUFourthHopAmountInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUFourthHopAmountInput").unwrap();
    let totalIBUFourthHopAmountInputBuffer = totalIBUFourthHopAmountInput.get_text().expect("No input");
    let totalIBUFourthHopAmountInputBufferFloat = validInput(&totalIBUFourthHopAmountInputBuffer);

    let totalIBUFifthHopAlphaInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUFifthHopAlphaInput").unwrap();
    let totalIBUFifthHopAlphaInputBuffer = totalIBUFifthHopAlphaInput.get_text().expect("No input");
    let totalIBUFifthHopAlphaInputBufferFloat = validInput(&totalIBUFifthHopAlphaInputBuffer);

    let totalIBUFifthHopAmountInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUFifthHopAmountInput").unwrap();
    let totalIBUFifthHopAmountInputBuffer = totalIBUFifthHopAmountInput.get_text().expect("No input");
    let totalIBUFifthHopAmountInputBufferFloat = validInput(&totalIBUFifthHopAmountInputBuffer);

    let totalIBUSixthHopAlphaInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUSixthHopAlphaInput").unwrap();
    let totalIBUSixthHopAlphaInputBuffer = totalIBUSixthHopAlphaInput.get_text().expect("No input");
    let totalIBUSixthHopAlphaInputBufferFloat = validInput(&totalIBUSixthHopAlphaInputBuffer);

    let totalIBUSixthHopAmountInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUSixthHopAmountInput").unwrap();
    let totalIBUSixthHopAmountInputBuffer = totalIBUSixthHopAmountInput.get_text().expect("No input");
    let totalIBUSixthHopAmountInputBufferFloat = validInput(&totalIBUSixthHopAmountInputBuffer);

    let totalIBUSeventhHopAlphaInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUSeventhHopAlphaInput").unwrap();
    let totalIBUSeventhHopAlphaInputBuffer = totalIBUSeventhHopAlphaInput.get_text().expect("No input");
    let totalIBUSeventhHopAlphaInputBufferFloat = validInput(&totalIBUSeventhHopAlphaInputBuffer);

    let totalIBUSeventhHopAmountInput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUSeventhHopAmountInput").unwrap();
    let totalIBUSeventhHopAmountInputBuffer = totalIBUSeventhHopAmountInput.get_text().expect("No input");
    let totalIBUSeventhHopAmountInputBufferFloat = validInput(&totalIBUSeventhHopAmountInputBuffer);

    let totalIBUOutput = String::from("totalIBUOutput");

    if totalIBUPreBoilBrixInputBufferFloat == 0.0 {
        let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
        output.set_text("Enter at least the first five inputs");
    } else if totalIBUWortVolumeInputBufferFloat == 0.0 {
        let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
        output.set_text("Enter at least the first five inputs");
    } else if totalIBUBoilTimeInputBufferFloat == 0.0 {
        let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
        output.set_text("Enter at least the first five inputs");
    } else if totalIBUFirstHopAlphaInputBufferFloat == 0.0 {
        let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
        output.set_text("Enter at least the first five inputs");
    } else if totalIBUFirstHopAmountInputBufferFloat == 0.0 {
        let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
        output.set_text("Enter at least the first five inputs");
    } else {
        if totalIBUPreBoilBrixInputBufferFloat <= 0.0 {
            let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if totalIBUWortVolumeInputBufferFloat <= 0.0 {
            let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if totalIBUBoilTimeInputBufferFloat <= 0.0 {
            let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if totalIBUFirstHopAlphaInputBufferFloat <= 0.0 {
            let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if totalIBUFirstHopAmountInputBufferFloat <= 0.0 {
            let output: gtk::Entry = IBUBuilderClone.get_object(&totalIBUOutput).unwrap();
            output.set_text("Enter a positive number");
        } else {
            onIBUActivate(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUFirstHopAlphaInputBufferFloat, totalIBUFirstHopAmountInputBufferFloat, totalIBUSecondHopAlphaInputBufferFloat, totalIBUSecondHopAmountInputBufferFloat, totalIBUThirdHopAlphaInputBufferFloat, totalIBUThirdHopAmountInputBufferFloat, totalIBUFourthHopAlphaInputBufferFloat, totalIBUFourthHopAmountInputBufferFloat, totalIBUFifthHopAlphaInputBufferFloat, totalIBUFifthHopAmountInputBufferFloat, totalIBUSixthHopAlphaInputBufferFloat, totalIBUSixthHopAmountInputBufferFloat, totalIBUSeventhHopAlphaInputBufferFloat, totalIBUSeventhHopAmountInputBufferFloat, &IBUBuilderClone);
            }
        }
    }

fn onIBUActivate(totalIBUPreBoilBrixInputBufferFloat: f32, totalIBUWortVolumeInputBufferFloat: f32, totalIBUBoilTimeInputBufferFloat: f32, totalIBUFirstHopAlphaInputBufferFloat: f32, totalIBUFirstHopAmountInputBufferFloat: f32, totalIBUSecondHopAlphaInputBufferFloat: f32, totalIBUSecondHopAmountInputBufferFloat: f32, totalIBUThirdHopAlphaInputBufferFloat: f32, totalIBUThirdHopAmountInputBufferFloat: f32, totalIBUFourthHopAlphaInputBufferFloat: f32, totalIBUFourthHopAmountInputBufferFloat: f32, totalIBUFifthHopAlphaInputBufferFloat: f32, totalIBUFifthHopAmountInputBufferFloat: f32, totalIBUSixthHopAlphaInputBufferFloat: f32, totalIBUSixthHopAmountInputBufferFloat: f32, totalIBUSeventhHopAlphaInputBufferFloat: f32, totalIBUSeventhHopAmountInputBufferFloat: f32, ref IBUBuilderClone: &gtk::Builder) {
    let ref totalIBUSwitch: &gtk::Switch = &IBUBuilderClone.get_object("totalIBUSwitch").unwrap();

    if totalIBUSwitch.get_active() == true {
        let imperialOrMetric = String::from("metric");
        totalIBUMaths(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUFirstHopAlphaInputBufferFloat, totalIBUFirstHopAmountInputBufferFloat, totalIBUSecondHopAlphaInputBufferFloat, totalIBUSecondHopAmountInputBufferFloat, totalIBUThirdHopAlphaInputBufferFloat, totalIBUThirdHopAmountInputBufferFloat, totalIBUFourthHopAlphaInputBufferFloat, totalIBUFourthHopAmountInputBufferFloat, totalIBUFifthHopAlphaInputBufferFloat, totalIBUFifthHopAmountInputBufferFloat, totalIBUSixthHopAlphaInputBufferFloat, totalIBUSixthHopAmountInputBufferFloat, totalIBUSeventhHopAlphaInputBufferFloat, totalIBUSeventhHopAmountInputBufferFloat, imperialOrMetric, &IBUBuilderClone);
    } else if totalIBUSwitch.get_active() == false {
        let imperialOrMetric = String::from("imperial");
        totalIBUMaths(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUFirstHopAlphaInputBufferFloat, totalIBUFirstHopAmountInputBufferFloat, totalIBUSecondHopAlphaInputBufferFloat, totalIBUSecondHopAmountInputBufferFloat, totalIBUThirdHopAlphaInputBufferFloat, totalIBUThirdHopAmountInputBufferFloat, totalIBUFourthHopAlphaInputBufferFloat, totalIBUFourthHopAmountInputBufferFloat, totalIBUFifthHopAlphaInputBufferFloat, totalIBUFifthHopAmountInputBufferFloat, totalIBUSixthHopAlphaInputBufferFloat, totalIBUSixthHopAmountInputBufferFloat, totalIBUSeventhHopAlphaInputBufferFloat, totalIBUSeventhHopAmountInputBufferFloat, imperialOrMetric, &IBUBuilderClone);
    }
}

fn totalIBUMaths(totalIBUPreBoilBrixInputBufferFloat: f32, totalIBUWortVolumeInputBufferFloat: f32, totalIBUBoilTimeInputBufferFloat: f32, totalIBUFirstHopAlphaInputBufferFloat: f32, totalIBUFirstHopAmountInputBufferFloat: f32, totalIBUSecondHopAlphaInputBufferFloat: f32, totalIBUSecondHopAmountInputBufferFloat: f32, totalIBUThirdHopAlphaInputBufferFloat: f32, totalIBUThirdHopAmountInputBufferFloat: f32, totalIBUFourthHopAlphaInputBufferFloat: f32, totalIBUFourthHopAmountInputBufferFloat: f32, totalIBUFifthHopAlphaInputBufferFloat: f32, totalIBUFifthHopAmountInputBufferFloat: f32, totalIBUSixthHopAlphaInputBufferFloat: f32, totalIBUSixthHopAmountInputBufferFloat: f32, totalIBUSeventhHopAlphaInputBufferFloat: f32, totalIBUSeventhHopAmountInputBufferFloat: f32, imperialOrMetric: String, ref IBUBuilderClone: &gtk::Builder) {
    let ref totalIBUOutput: &gtk::Entry = &IBUBuilderClone.get_object("totalIBUOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let totalIBU1 = realIBU(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUFirstHopAlphaInputBufferFloat, totalIBUFirstHopAmountInputBufferFloat);
        let totalIBU2 = realIBU(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUSecondHopAlphaInputBufferFloat, totalIBUSecondHopAmountInputBufferFloat);
        let totalIBU3 = realIBU(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUThirdHopAlphaInputBufferFloat, totalIBUThirdHopAmountInputBufferFloat);
        let totalIBU4 = realIBU(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUFourthHopAlphaInputBufferFloat, totalIBUFourthHopAmountInputBufferFloat);
        let totalIBU5 = realIBU(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUFifthHopAlphaInputBufferFloat, totalIBUFifthHopAmountInputBufferFloat);
        let totalIBU6 = realIBU(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUSixthHopAlphaInputBufferFloat, totalIBUSixthHopAmountInputBufferFloat);
        let totalIBU7 = realIBU(totalIBUPreBoilBrixInputBufferFloat, totalIBUWortVolumeInputBufferFloat, totalIBUBoilTimeInputBufferFloat, totalIBUSeventhHopAlphaInputBufferFloat, totalIBUSeventhHopAmountInputBufferFloat);
        let total = format!("{:.2} IBUs", &totalIBU1 + &totalIBU2 + &totalIBU3 + &totalIBU4 + &totalIBU5 + &totalIBU6 + &totalIBU7);
        totalIBUOutput.set_text(&total);
    } else if imperialOrMetric == "metric" {
        let totalIBU1 = realIBU(totalIBUPreBoilBrixInputBufferFloat, litresToGallons(totalIBUWortVolumeInputBufferFloat), totalIBUBoilTimeInputBufferFloat, totalIBUFirstHopAlphaInputBufferFloat, gramsToOunces(totalIBUFirstHopAmountInputBufferFloat));
        let totalIBU2 = realIBU(totalIBUPreBoilBrixInputBufferFloat, litresToGallons(totalIBUWortVolumeInputBufferFloat), totalIBUBoilTimeInputBufferFloat, totalIBUSecondHopAlphaInputBufferFloat, gramsToOunces(totalIBUSecondHopAmountInputBufferFloat));
        let totalIBU3 = realIBU(totalIBUPreBoilBrixInputBufferFloat, litresToGallons(totalIBUWortVolumeInputBufferFloat), totalIBUBoilTimeInputBufferFloat, totalIBUThirdHopAlphaInputBufferFloat, gramsToOunces(totalIBUThirdHopAmountInputBufferFloat));
        let totalIBU4 = realIBU(totalIBUPreBoilBrixInputBufferFloat, litresToGallons(totalIBUWortVolumeInputBufferFloat), totalIBUBoilTimeInputBufferFloat, totalIBUFourthHopAlphaInputBufferFloat, gramsToOunces(totalIBUFourthHopAmountInputBufferFloat));
        let totalIBU5 = realIBU(totalIBUPreBoilBrixInputBufferFloat, litresToGallons(totalIBUWortVolumeInputBufferFloat), totalIBUBoilTimeInputBufferFloat, totalIBUFifthHopAlphaInputBufferFloat, gramsToOunces(totalIBUFifthHopAmountInputBufferFloat));
        let totalIBU6 = realIBU(totalIBUPreBoilBrixInputBufferFloat, litresToGallons(totalIBUWortVolumeInputBufferFloat), totalIBUBoilTimeInputBufferFloat, totalIBUSixthHopAlphaInputBufferFloat, gramsToOunces(totalIBUSixthHopAmountInputBufferFloat));
        let totalIBU7 = realIBU(totalIBUPreBoilBrixInputBufferFloat, litresToGallons(totalIBUWortVolumeInputBufferFloat), totalIBUBoilTimeInputBufferFloat, totalIBUSeventhHopAlphaInputBufferFloat, gramsToOunces(totalIBUSeventhHopAmountInputBufferFloat));
        let total = format!("{:.2} IBUs", &totalIBU1 + &totalIBU2 + &totalIBU3 + &totalIBU4 + &totalIBU5 + &totalIBU6 + &totalIBU7);
        totalIBUOutput.set_text(&total);
    }
}