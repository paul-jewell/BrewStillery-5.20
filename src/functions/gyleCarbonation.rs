use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn gyleCarbonationPrep(ref gyleBuilderClone: &gtk::Builder) {
    let gyleBrixInput: &gtk::Entry = &gyleBuilderClone.get_object("gyleBrixInput").unwrap();
    let gyleBrixInputBuffer = gyleBrixInput.get_text().expect("No input");
    let startingBrix = validInput(&gyleBrixInputBuffer);

    let gyleCO2Input: &gtk::Entry = &gyleBuilderClone.get_object("gyleCO2Input").unwrap();
    let gyleCO2InputBuffer = gyleCO2Input.get_text().expect("No input");
    let desiredCO2Level = validInput(&gyleCO2InputBuffer);

    let gyleWortVolumeInput: &gtk::Entry = &gyleBuilderClone.get_object("gyleWortVolumeInput").unwrap();
    let gyleWortVolumeInputBuffer = gyleWortVolumeInput.get_text().expect("No input");
    let finalVolume = validInput(&gyleWortVolumeInputBuffer);

    let gyleOutput = String::from("gyleOutput");

    if startingBrix == 0.0 {
        let output: gtk::Entry = gyleBuilderClone.get_object(&gyleOutput).unwrap();
        output.set_text("Enter a number");
    } else if desiredCO2Level == 0.0 {
        let output: gtk::Entry = gyleBuilderClone.get_object(&gyleOutput).unwrap();
        output.set_text("Enter a number");
    } else if finalVolume == 0.0 {
        let output: gtk::Entry = gyleBuilderClone.get_object(&gyleOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        if startingBrix <= 0.0 {
            let output: gtk::Entry = gyleBuilderClone.get_object(&gyleOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if desiredCO2Level <= 0.0 {
            let output: gtk::Entry = gyleBuilderClone.get_object(&gyleOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if finalVolume <= 0.0 {
            let output: gtk::Entry = gyleBuilderClone.get_object(&gyleOutput).unwrap();
            output.set_text("Enter a positive number");
        } else {
            onGyleActivate(startingBrix, desiredCO2Level, finalVolume, &gyleBuilderClone);
        }
    }
}

fn onGyleActivate(startingBrix: f32, desiredCO2Level: f32, finalVolume: f32, ref gyleBuilderClone: &gtk::Builder) {
    let ref gyleCarbonationSwitch: &gtk::Switch = &gyleBuilderClone.get_object("gyleCarbonationSwitch").unwrap();

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