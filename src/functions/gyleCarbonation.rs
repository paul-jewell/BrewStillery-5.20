use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn gyleCarbonationPrep(gyleBuilderClone: &gtk::Builder) {
    let gyleBrixInput: gtk::Entry = gyleBuilderClone.get_object("gyleBrixInput").unwrap();
    let gyleBrixInputBuffer = gyleBrixInput.get_text().expect("No input");
    let startingBrix = validInput(&gyleBrixInputBuffer);

    let gyleCO2Input: gtk::Entry = gyleBuilderClone.get_object("gyleCO2Input").unwrap();
    let gyleCO2InputBuffer = gyleCO2Input.get_text().expect("No input");
    let desiredCO2Level = validInput(&gyleCO2InputBuffer);

    let gyleWortVolumeInput: gtk::Entry = gyleBuilderClone.get_object("gyleWortVolumeInput").unwrap();
    let gyleWortVolumeInputBuffer = gyleWortVolumeInput.get_text().expect("No input");
    let finalVolume = validInput(&gyleWortVolumeInputBuffer);

    let gyleOutput: &gtk::Entry = &gyleBuilderClone.get_object("gyleOutput").unwrap();

    if startingBrix < 2.57 {
        gyleOutput.set_text("Enter a Starting Brix greater than 2.57");
    } else if startingBrix > 49.48 {
        gyleOutput.set_text("Enter a Starting Brix less than 49.48");
    } else if startingBrix == 0.0 || desiredCO2Level == 0.0 || finalVolume == 0.0 {
        gyleOutput.set_text("Enter all 3 inputs");
    } else if startingBrix <= 0.0 || desiredCO2Level <= 0.0  || finalVolume <= 0.0 {
        gyleOutput.set_text("Enter a positive number");
    } else {
        onGyleActivate(startingBrix, desiredCO2Level, finalVolume, gyleBuilderClone);
    }
}

fn onGyleActivate(startingBrix: f64, desiredCO2Level: f64, finalVolume: f64, gyleBuilderClone: &gtk::Builder) {
    let gyleCarbonationSwitch: gtk::Switch = gyleBuilderClone.get_object("gyleCarbonationSwitch").unwrap();

    if gyleCarbonationSwitch.get_active() == true {
        let imperialOrMetric = "metric";
        gyleMaths(startingBrix, desiredCO2Level, finalVolume, imperialOrMetric, gyleBuilderClone);
    } else if gyleCarbonationSwitch.get_active() == false {
        let imperialOrMetric = "imperial";
        gyleMaths(startingBrix, desiredCO2Level, finalVolume, imperialOrMetric, gyleBuilderClone);
    }
}

fn gyleMaths(startingBrix: f64, desiredCO2Level: f64, finalVolume: f64, imperialOrMetric: &str, gyleBuilderClone: &gtk::Builder) {
    let startingGravity = brixToGravity(startingBrix);
    let startingPlato = gravityToPlato(startingGravity);
    let finalPlato = gravityToPlato(finalGravityIdeal);

    let gyleOutput: &gtk::Entry = &gyleBuilderClone.get_object("gyleOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let gyleVolume = litresToGallons((0.24 * gallonsToLitres(finalVolume) * desiredCO2Level) / (startingPlato - finalPlato));
        let total = format!("{:.2} gallons", gyleVolume);
        gyleOutput.set_text(&total);
    } else if imperialOrMetric == "metric" {
        let gyleVolume = (0.24 * finalVolume * desiredCO2Level) / (startingPlato - finalPlato);
        let total = format!("{:.2} litres", gyleVolume);
        gyleOutput.set_text(&total);
    }
}