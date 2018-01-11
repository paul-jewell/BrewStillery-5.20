use gtk;
use gtk::prelude::*;
use gdk::RGBA;
use functions::commonFunctions::*;
use light::lightFunctions::*;
use gui::colourOverlay;

struct grainABVData <'a> {
    wortAmount: f64,
    firstGrainAmount: f64,
    firstGrainInfo: (f64, f64),
    secondGrainAmount: f64,
    secondGrainInfo: (f64, f64),
    thirdGrainAmount: f64,
    thirdGrainInfo: (f64, f64),
    fourthGrainAmount: f64,
    fourthGrainInfo: (f64, f64),
    fifthGrainAmount: f64,
    fifthGrainInfo: (f64, f64),
    sixthGrainAmount: f64,
    sixthGrainInfo: (f64, f64),
    seventhGrainAmount: f64,
    seventhGrainInfo: (f64, f64),
    glassSize: f64,
    imperialOrMetric: &'a str,
    grainABVBuilder: &'a gtk::Builder,
}

pub fn grainABVPrep(grainABVBuilderClone: &gtk::Builder, allOverlays: &colourOverlay) {
    let grainABVWortInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVWortInput").unwrap();
    let grainABVWortInputBuffer = grainABVWortInput.get_text().expect("No input");

    let grainABVFirstAmountInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVFirstAmountInput").unwrap();
    let grainABVFirstAmountInputBuffer = grainABVFirstAmountInput.get_text().expect("No input");

    let grainABVFirstTypeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVFirstTypeInput").unwrap();
    let grainABVFirstTypeInputBuffer = grainABVFirstTypeInput.get_active_id().unwrap();

    let grainABVSecondAmountInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVSecondAmountInput").unwrap();
    let grainABVSecondAmountInputBuffer = grainABVSecondAmountInput.get_text().expect("No input");

    let grainABVSecondTypeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVSecondTypeInput").unwrap();
    let grainABVSecondTypeInputBuffer = grainABVSecondTypeInput.get_active_id().unwrap();

    let grainABVThirdAmountInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVThirdAmountInput").unwrap();
    let grainABVThirdAmountInputBuffer = grainABVThirdAmountInput.get_text().expect("No input");

    let grainABVThirdTypeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVThirdTypeInput").unwrap();
    let grainABVThirdTypeInputBuffer = grainABVThirdTypeInput.get_active_id().unwrap();

    let grainABVFourthAmountInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVFourthAmountInput").unwrap();
    let grainABVFourthAmountInputBuffer = grainABVFourthAmountInput.get_text().expect("No input");

    let grainABVFourthTypeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVFourthTypeInput").unwrap();
    let grainABVFourthTypeInputBuffer = grainABVFourthTypeInput.get_active_id().unwrap();

    let grainABVFifthAmountInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVFifthAmountInput").unwrap();
    let grainABVFifthAmountInputBuffer = grainABVFifthAmountInput.get_text().expect("No input");

    let grainABVFifthTypeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVFifthTypeInput").unwrap();
    let grainABVFifthTypeInputBuffer = grainABVFifthTypeInput.get_active_id().unwrap();

    let grainABVSixthAmountInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVSixthAmountInput").unwrap();
    let grainABVSixthAmountInputBuffer = grainABVSixthAmountInput.get_text().expect("No input");

    let grainABVSixthTypeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVSixthTypeInput").unwrap();
    let grainABVSixthTypeInputBuffer = grainABVSixthTypeInput.get_active_id().unwrap();

    let grainABVSeventhAmountInput: gtk::Entry = grainABVBuilderClone.get_object("grainABVSeventhAmountInput").unwrap();
    let grainABVSeventhAmountInputBuffer = grainABVSeventhAmountInput.get_text().expect("No input");

    let grainABVSeventhTypeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVSeventhTypeInput").unwrap();
    let grainABVSeventhTypeInputBuffer = grainABVSeventhTypeInput.get_active_id().unwrap();

    let grainABVGlassSizeInput: gtk::ComboBoxText = grainABVBuilderClone.get_object("grainABVGlassSizeInput").unwrap();
    let grainABVGlassSizeInputBuffer = grainABVGlassSizeInput.get_active_id().unwrap();

    let allInputs = grainABVData {
        wortAmount: validInput(&grainABVWortInputBuffer),
        firstGrainAmount: validInput(&grainABVFirstAmountInputBuffer),
        firstGrainInfo: grainInfo(&grainABVFirstTypeInputBuffer),
        secondGrainAmount: validInput(&grainABVSecondAmountInputBuffer),
        secondGrainInfo: grainInfo(&grainABVSecondTypeInputBuffer),
        thirdGrainAmount: validInput(&grainABVThirdAmountInputBuffer),
        thirdGrainInfo: grainInfo(&grainABVThirdTypeInputBuffer),
        fourthGrainAmount: validInput(&grainABVFourthAmountInputBuffer),
        fourthGrainInfo: grainInfo(&grainABVFourthTypeInputBuffer),
        fifthGrainAmount: validInput(&grainABVFifthAmountInputBuffer),
        fifthGrainInfo: grainInfo(&grainABVFifthTypeInputBuffer),
        sixthGrainAmount: validInput(&grainABVSixthAmountInputBuffer),
        sixthGrainInfo: grainInfo(&grainABVSixthTypeInputBuffer),
        seventhGrainAmount: validInput(&grainABVSeventhAmountInputBuffer),
        seventhGrainInfo: grainInfo(&grainABVSeventhTypeInputBuffer),
        glassSize: glassSize(&grainABVGlassSizeInputBuffer),
        imperialOrMetric: "imperial",
        grainABVBuilder: grainABVBuilderClone,
    };

    let grainABVBrixOutput: gtk::Entry = allInputs.grainABVBuilder.get_object("grainABVBrixOutput").unwrap();
    let grainABVABVOutput: gtk::Entry = allInputs.grainABVBuilder.get_object("grainABVABVOutput").unwrap();


    if grainABVGlassSizeInputBuffer == "Dimple Pint" {
        allOverlays.dimplePint.set_opacity(1.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeInputBuffer == "Nonick Pint"{
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(1.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeInputBuffer == "Tulip Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(1.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeInputBuffer == "Pilsner" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(1.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeInputBuffer ==  "Maß" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(1.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeInputBuffer == "Dimple Half Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(1.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeInputBuffer == "Nonick Half Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(1.0);
        allOverlays.tulipHalfPint.set_opacity(0.0);
    } else if grainABVGlassSizeInputBuffer == "Tulip Half Pint" {
        allOverlays.dimplePint.set_opacity(0.0);
        allOverlays.nonickPint.set_opacity(0.0);
        allOverlays.tulipPint.set_opacity(0.0);
        allOverlays.pilsner.set_opacity(0.0);
        allOverlays.mafs.set_opacity(0.0);
        allOverlays.dimpleHalfPint.set_opacity(0.0);
        allOverlays.nonickHalfPint.set_opacity(0.0);
        allOverlays.tulipHalfPint.set_opacity(1.0);
    }

    let rgbaZero = RGBA {
        red: 255.0,
        green: 255.0,
        blue: 255.0,
        alpha: 1.0,
    };

    if allInputs.wortAmount == 0.0 {
        grainABVBrixOutput.set_text("Enter a Wort Volume");
        grainABVABVOutput.set_text("");
        allOverlays.colourOutput.set_rgba(&rgbaZero);
    } else if allInputs.firstGrainAmount == 0.0 {
        grainABVBrixOutput.set_text("Enter at least");
        grainABVABVOutput.set_text("1 grain amount");
        allOverlays.colourOutput.set_rgba(&rgbaZero);
    } else if allInputs.wortAmount <= 0.0 || allInputs.firstGrainAmount <= 0.0 {
        grainABVBrixOutput.set_text("Enter a positive number");
        grainABVABVOutput.set_text("");
        allOverlays.colourOutput.set_rgba(&rgbaZero);
    } else {
        onGrainABVActivate(allInputs, &allOverlays);
    }
}

fn onGrainABVActivate(mut allInputs: grainABVData, allOverlays: &colourOverlay) {
    let grainABVSwitch: gtk::Switch = allInputs.grainABVBuilder.get_object("grainABVSwitch").unwrap();

    if grainABVSwitch.get_active() == true {
        allInputs.imperialOrMetric = "metric";
        grainABVMaths(allInputs, allOverlays);
    } else if grainABVSwitch.get_active() == false {
        grainABVMaths(allInputs, allOverlays);
    }
}

fn grainABVMaths(allInputs: grainABVData, allOverlays: &colourOverlay) {
    let grainABVBrixOutput: gtk::Entry = allInputs.grainABVBuilder.get_object("grainABVBrixOutput").unwrap();
    let grainABVABVOutput: gtk::Entry = allInputs.grainABVBuilder.get_object("grainABVABVOutput").unwrap();

    let singleMCU1 = singleMCU(allInputs.wortAmount, allInputs.firstGrainAmount, allInputs.firstGrainInfo.1);
    let singleMCU2 = singleMCU(allInputs.wortAmount, allInputs.secondGrainAmount, allInputs.secondGrainInfo.1);
    let singleMCU3 = singleMCU(allInputs.wortAmount, allInputs.thirdGrainAmount, allInputs.thirdGrainInfo.1);
    let singleMCU4 = singleMCU(allInputs.wortAmount, allInputs.fourthGrainAmount, allInputs.fourthGrainInfo.1);
    let singleMCU5 = singleMCU(allInputs.wortAmount, allInputs.fifthGrainAmount, allInputs.fifthGrainInfo.1);
    let singleMCU6 = singleMCU(allInputs.wortAmount, allInputs.sixthGrainAmount, allInputs.sixthGrainInfo.1);
    let singleMCU7 = singleMCU(allInputs.wortAmount, allInputs.seventhGrainAmount, allInputs.seventhGrainInfo.1);
    let totalMCU = singleMCU1 + singleMCU2 + singleMCU3 + singleMCU4 + singleMCU5 + singleMCU6 + singleMCU7;
    let beerSRM = beerSRM(totalMCU);
    let labOutput = grainSRMToLAB(allInputs.glassSize, beerSRM);
    let xyzOutput = grainLABToXYZ(labOutput.0, labOutput.1, labOutput.2);
    let rgbaOutput = grainXYZToRGBA(xyzOutput.0, xyzOutput.1, xyzOutput.2);
    let rgbaZero = RGBA {
        red: 255.0,
        green: 255.0,
        blue: 255.0,
        alpha: 1.0,
    };

    if allInputs.imperialOrMetric == "imperial" {
        let totalGrain1 = grainToABV(allInputs.wortAmount, allInputs.firstGrainAmount, allInputs.firstGrainInfo.0);
        let totalGrain2 = grainToABV(allInputs.wortAmount, allInputs.secondGrainAmount, allInputs.secondGrainInfo.0);
        let totalGrain3 = grainToABV(allInputs.wortAmount, allInputs.thirdGrainAmount, allInputs.thirdGrainInfo.0);
        let totalGrain4 = grainToABV(allInputs.wortAmount, allInputs.fourthGrainAmount, allInputs.fourthGrainInfo.0);
        let totalGrain5 = grainToABV(allInputs.wortAmount, allInputs.fifthGrainAmount, allInputs.fifthGrainInfo.0);
        let totalGrain6 = grainToABV(allInputs.wortAmount, allInputs.sixthGrainAmount, allInputs.sixthGrainInfo.0);
        let totalGrain7 = grainToABV(allInputs.wortAmount, allInputs.seventhGrainAmount, allInputs.seventhGrainInfo.0);
        let startingBrix = gravityToBrix(1.0 + &totalGrain1 % 1.0 + &totalGrain2 % 1.0 + &totalGrain3 % 1.0 + &totalGrain4 % 1.0 + &totalGrain5 % 1.0 + &totalGrain6 % 1.0 + &totalGrain7 % 1.0);
        let estimatedBrix = format!("{:.2}°Bx", startingBrix);
        let abv = realABV(startingBrix, finalBrixIdeal).0;
        let abvFormatted = format!("{:.2}%", abv);
        if abv > 26.0 || abv <= 0.0 {
            grainABVBrixOutput.set_text("Enter legimate amounts");
            grainABVABVOutput.set_text("");
            allOverlays.colourOutput.set_rgba(&rgbaZero);
        } else {
            grainABVBrixOutput.set_text(&estimatedBrix);
            grainABVABVOutput.set_text(&abvFormatted);
            allOverlays.colourOutput.set_rgba(&rgbaOutput);
        }
    } else if allInputs.imperialOrMetric == "metric" {
        let totalGrain1 = grainToABV(litresToGallons(allInputs.wortAmount), kilosToPounds(allInputs.firstGrainAmount), allInputs.firstGrainInfo.0);
        let totalGrain2 = grainToABV(litresToGallons(allInputs.wortAmount), kilosToPounds(allInputs.secondGrainAmount), allInputs.secondGrainInfo.0);
        let totalGrain3 = grainToABV(litresToGallons(allInputs.wortAmount), kilosToPounds(allInputs.thirdGrainAmount), allInputs.thirdGrainInfo.0);
        let totalGrain4 = grainToABV(litresToGallons(allInputs.wortAmount), kilosToPounds(allInputs.fourthGrainAmount), allInputs.fourthGrainInfo.0);
        let totalGrain5 = grainToABV(litresToGallons(allInputs.wortAmount), kilosToPounds(allInputs.fifthGrainAmount), allInputs.fifthGrainInfo.0);
        let totalGrain6 = grainToABV(litresToGallons(allInputs.wortAmount), kilosToPounds(allInputs.sixthGrainAmount), allInputs.sixthGrainInfo.0);
        let totalGrain7 = grainToABV(litresToGallons(allInputs.wortAmount), kilosToPounds(allInputs.seventhGrainAmount), allInputs.seventhGrainInfo.0);
        let startingBrix = gravityToBrix(1.0 + &totalGrain1 % 1.0 + &totalGrain2 % 1.0 + &totalGrain3 % 1.0 + &totalGrain4 % 1.0 + &totalGrain5 % 1.0 + &totalGrain6 % 1.0 + &totalGrain7 % 1.0);
        let estimatedBrix = format!("{:.2}°Bx", startingBrix);
        let abv = realABV(startingBrix, finalBrixIdeal).0;
        let abvFormatted = format!("{:.2}%", abv);
        if abv > 26.0 || abv <= 0.0 {
            grainABVBrixOutput.set_text("Enter legimate amounts");
            grainABVABVOutput.set_text("");
            allOverlays.colourOutput.set_rgba(&rgbaZero);
        } else {
            grainABVBrixOutput.set_text(&estimatedBrix);
            grainABVABVOutput.set_text(&abvFormatted);
            allOverlays.colourOutput.set_rgba(&rgbaOutput);
        }
    }
}