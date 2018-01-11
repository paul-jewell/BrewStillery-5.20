use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

struct guestimateIBUData <'a> {
    preBoilBrix: f64,
    wortVolume: f64,
    boilTime: f64,
    firstHopAlpha: f64,
    firstHopAmount: f64,
    secondHopAlpha: f64,
    secondHopAmount: f64,
    thirdHopAlpha: f64,
    thirdHopAmount: f64,
    fourthHopAlpha: f64,
    fourthHopAmount: f64,
    fifthHopAlpha: f64,
    fifthHopAmount: f64,
    sixthHopAlpha: f64,
    sixthHopAmount: f64,
    seventhHopAlpha: f64,
    seventhHopAmount: f64,
    imperialOrMetric: &'a str,
    IBUBuilder: &'a gtk::Builder,
}

pub fn guestimateIBUPrep(IBUBuilderClone: &gtk::Builder) {
    let totalIBUPreBoilBrixInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUPreBoilBrixInput").unwrap();
    let totalIBUPreBoilBrixInputBuffer = totalIBUPreBoilBrixInput.get_text().expect("No input");

    let totalIBUWortVolumeInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUWortVolumeInput").unwrap();
    let totalIBUWortVolumeInputBuffer = totalIBUWortVolumeInput.get_text().expect("No input");

    let totalIBUBoilTimeInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUBoilTimeInput").unwrap();
    let totalIBUBoilTimeInputBuffer = totalIBUBoilTimeInput.get_text().expect("No input");

    let totalIBUFirstHopAlphaInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUFirstHopAlphaInput").unwrap();
    let totalIBUFirstHopAlphaInputBuffer = totalIBUFirstHopAlphaInput.get_text().expect("No input");

    let totalIBUFirstHopAmountInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUFirstHopAmountInput").unwrap();
    let totalIBUFirstHopAmountInputBuffer = totalIBUFirstHopAmountInput.get_text().expect("No input");

    let totalIBUSecondHopAlphaInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUSecondHopAlphaInput").unwrap();
    let totalIBUSecondHopAlphaInputBuffer = totalIBUSecondHopAlphaInput.get_text().expect("No input");

    let totalIBUSecondHopAmountInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUSecondHopAmountInput").unwrap();
    let totalIBUSecondHopAmountInputBuffer = totalIBUSecondHopAmountInput.get_text().expect("No input");

    let totalIBUThirdHopAlphaInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUThirdHopAlphaInput").unwrap();
    let totalIBUThirdHopAlphaInputBuffer = totalIBUThirdHopAlphaInput.get_text().expect("No input");

    let totalIBUThirdHopAmountInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUThirdHopAmountInput").unwrap();
    let totalIBUThirdHopAmountInputBuffer = totalIBUThirdHopAmountInput.get_text().expect("No input");

    let totalIBUFourthHopAlphaInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUFourthHopAlphaInput").unwrap();
    let totalIBUFourthHopAlphaInputBuffer = totalIBUFourthHopAlphaInput.get_text().expect("No input");

    let totalIBUFourthHopAmountInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUFourthHopAmountInput").unwrap();
    let totalIBUFourthHopAmountInputBuffer = totalIBUFourthHopAmountInput.get_text().expect("No input");

    let totalIBUFifthHopAlphaInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUFifthHopAlphaInput").unwrap();
    let totalIBUFifthHopAlphaInputBuffer = totalIBUFifthHopAlphaInput.get_text().expect("No input");

    let totalIBUFifthHopAmountInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUFifthHopAmountInput").unwrap();
    let totalIBUFifthHopAmountInputBuffer = totalIBUFifthHopAmountInput.get_text().expect("No input");

    let totalIBUSixthHopAlphaInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUSixthHopAlphaInput").unwrap();
    let totalIBUSixthHopAlphaInputBuffer = totalIBUSixthHopAlphaInput.get_text().expect("No input");

    let totalIBUSixthHopAmountInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUSixthHopAmountInput").unwrap();
    let totalIBUSixthHopAmountInputBuffer = totalIBUSixthHopAmountInput.get_text().expect("No input");

    let totalIBUSeventhHopAlphaInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUSeventhHopAlphaInput").unwrap();
    let totalIBUSeventhHopAlphaInputBuffer = totalIBUSeventhHopAlphaInput.get_text().expect("No input");

    let totalIBUSeventhHopAmountInput: gtk::Entry = IBUBuilderClone.get_object("totalIBUSeventhHopAmountInput").unwrap();
    let totalIBUSeventhHopAmountInputBuffer = totalIBUSeventhHopAmountInput.get_text().expect("No input");

    let allInputs = guestimateIBUData {
        preBoilBrix: validInput(&totalIBUPreBoilBrixInputBuffer),
        wortVolume: validInput(&totalIBUWortVolumeInputBuffer),
        boilTime: validInput(&totalIBUBoilTimeInputBuffer),
        firstHopAlpha: validInput(&totalIBUFirstHopAlphaInputBuffer),
        firstHopAmount: validInput(&totalIBUFirstHopAmountInputBuffer),
        secondHopAlpha: validInput(&totalIBUSecondHopAlphaInputBuffer),
        secondHopAmount: validInput(&totalIBUSecondHopAmountInputBuffer),
        thirdHopAlpha: validInput(&totalIBUThirdHopAlphaInputBuffer),
        thirdHopAmount: validInput(&totalIBUThirdHopAmountInputBuffer),
        fourthHopAlpha: validInput(&totalIBUFourthHopAlphaInputBuffer),
        fourthHopAmount: validInput(&totalIBUFourthHopAmountInputBuffer),
        fifthHopAlpha: validInput(&totalIBUFifthHopAlphaInputBuffer),
        fifthHopAmount: validInput(&totalIBUFifthHopAmountInputBuffer),
        sixthHopAlpha: validInput(&totalIBUSixthHopAlphaInputBuffer),
        sixthHopAmount: validInput(&totalIBUSixthHopAmountInputBuffer),
        seventhHopAlpha: validInput(&totalIBUSeventhHopAlphaInputBuffer),
        seventhHopAmount: validInput(&totalIBUSeventhHopAmountInputBuffer),
        imperialOrMetric: "imperial",
        IBUBuilder: IBUBuilderClone,
    };

    let totalIBUOutput: gtk::Entry = allInputs.IBUBuilder.get_object("totalIBUOutput").unwrap();

    if allInputs.preBoilBrix < 2.57 {
        totalIBUOutput.set_text("Enter a Pre-Boil Brix greater than 2.57");
    } else if allInputs.preBoilBrix > 49.48 {
        totalIBUOutput.set_text("Enter a Pre-Boil Brix less than 49.48");
    } else if allInputs.preBoilBrix == 0.0 || allInputs.wortVolume == 0.0 || allInputs.boilTime == 0.0 || allInputs.firstHopAlpha == 0.0 || allInputs.firstHopAmount == 0.0 {
        totalIBUOutput.set_text("Enter at least the first 5 inputs");
    } else if allInputs.preBoilBrix <= 0.0 || allInputs.wortVolume <= 0.0 || allInputs.boilTime <= 0.0 || allInputs.firstHopAlpha <= 0.0 || allInputs.firstHopAmount <= 0.0 {
        totalIBUOutput.set_text("Enter a positive number");
    } else {
        onIBUActivate(allInputs);
    }
}

fn onIBUActivate(mut allInputs: guestimateIBUData) {
    let totalIBUSwitch: gtk::Switch = allInputs.IBUBuilder.get_object("totalIBUSwitch").unwrap();

    if totalIBUSwitch.get_active() == true {
        allInputs.imperialOrMetric = "metric";
        totalIBUMaths(allInputs);
    } else if totalIBUSwitch.get_active() == false {
        totalIBUMaths(allInputs);
    }
}

fn totalIBUMaths(allInputs: guestimateIBUData) {
    let totalIBUOutput: gtk::Entry = allInputs.IBUBuilder.get_object("totalIBUOutput").unwrap();

    if allInputs.imperialOrMetric == "imperial" {
        let totalIBU1 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.boilTime, allInputs.firstHopAlpha, allInputs.firstHopAmount);
        let totalIBU2 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.boilTime, allInputs.secondHopAlpha, allInputs.secondHopAmount);
        let totalIBU3 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.boilTime, allInputs.thirdHopAlpha, allInputs.thirdHopAmount);
        let totalIBU4 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.boilTime, allInputs.fourthHopAlpha, allInputs.fourthHopAmount);
        let totalIBU5 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.boilTime, allInputs.fifthHopAlpha, allInputs.fifthHopAmount);
        let totalIBU6 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.boilTime, allInputs.sixthHopAlpha, allInputs.sixthHopAmount);
        let totalIBU7 = realIBU(allInputs.preBoilBrix, allInputs.wortVolume, allInputs.boilTime, allInputs.seventhHopAlpha, allInputs.seventhHopAmount);
        let total = format!("{:.2} IBUs", totalIBU1 + totalIBU2 + totalIBU3 + totalIBU4 + totalIBU5 + totalIBU6 + totalIBU7);
        totalIBUOutput.set_text(&total);
    } else if allInputs.imperialOrMetric == "metric" {
        let totalIBU1 = realIBU(allInputs.preBoilBrix, litresToGallons(allInputs.wortVolume), allInputs.boilTime, allInputs.firstHopAlpha, gramsToOunces(allInputs.firstHopAmount));
        let totalIBU2 = realIBU(allInputs.preBoilBrix, litresToGallons(allInputs.wortVolume), allInputs.boilTime, allInputs.secondHopAlpha, gramsToOunces(allInputs.secondHopAmount));
        let totalIBU3 = realIBU(allInputs.preBoilBrix, litresToGallons(allInputs.wortVolume), allInputs.boilTime, allInputs.thirdHopAlpha, gramsToOunces(allInputs.thirdHopAmount));
        let totalIBU4 = realIBU(allInputs.preBoilBrix, litresToGallons(allInputs.wortVolume), allInputs.boilTime, allInputs.fourthHopAlpha, gramsToOunces(allInputs.fourthHopAmount));
        let totalIBU5 = realIBU(allInputs.preBoilBrix, litresToGallons(allInputs.wortVolume), allInputs.boilTime, allInputs.fifthHopAlpha, gramsToOunces(allInputs.fifthHopAmount));
        let totalIBU6 = realIBU(allInputs.preBoilBrix, litresToGallons(allInputs.wortVolume), allInputs.boilTime, allInputs.sixthHopAlpha, gramsToOunces(allInputs.sixthHopAmount));
        let totalIBU7 = realIBU(allInputs.preBoilBrix, litresToGallons(allInputs.wortVolume), allInputs.boilTime, allInputs.seventhHopAlpha, gramsToOunces(allInputs.seventhHopAmount));
        let total = format!("{:.2} IBUs", totalIBU1 + totalIBU2 + totalIBU3 + totalIBU4 + totalIBU5 + totalIBU6 + totalIBU7);
        totalIBUOutput.set_text(&total);
    }
}