use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn champagneCarbonationPrep(champagneCarbonationBuilderClone: &gtk::Builder) {
    let champagneCarbonationInput: gtk::Entry = champagneCarbonationBuilderClone.get_object("champagneCarbonationInput").unwrap();
    let champagneCarbonationInputBuffer = champagneCarbonationInput.get_text().expect("No input");
    let champagneVolume = validInput(&champagneCarbonationInputBuffer);

    let champagneCarbonationOutput: gtk::Entry = champagneCarbonationBuilderClone.get_object("champagneCarbonationOutput").unwrap();

    if champagneVolume == 0.0 {
        champagneCarbonationOutput.set_text("Enter a number");
    } else if champagneVolume <= 0.0 {
        champagneCarbonationOutput.set_text("Enter a positive number");
    } else {
        onChampagneActivate(champagneVolume, champagneCarbonationBuilderClone);
    }
}

fn onChampagneActivate(champagneVolume: f64, champagneCarbonationBuilderClone: &gtk::Builder) {
    let champagneCarbonationSwitch: gtk::Switch = champagneCarbonationBuilderClone.get_object("champagneCarbonationSwitch").unwrap();

    if champagneCarbonationSwitch.get_active() == true {
        let imperialOrMetric = "metric";
        champagneCarbonationMaths(champagneVolume, imperialOrMetric, champagneCarbonationBuilderClone);
    } else if champagneCarbonationSwitch.get_active() == false {
        let imperialOrMetric = "imperial";
        champagneCarbonationMaths(champagneVolume, imperialOrMetric, champagneCarbonationBuilderClone);
    }
}

fn champagneCarbonationMaths(champagneVolume: f64, imperialOrMetric: &str, champagneCarbonationBuilderClone: &gtk::Builder) {
    let champagneCarbonationOutput: gtk::Entry = champagneCarbonationBuilderClone.get_object("champagneCarbonationOutput").unwrap();

    if imperialOrMetric == "imperial" {
        let totalSugar = champagneVolume * 0.2;
        if totalSugar == 1.0 {
            let sugar = format!("{:.0} pound", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        } else {
            let sugar = format!("{:.2} pounds", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        }
    } else if imperialOrMetric == "metric" {
        let totalSugar = (champagneVolume * 23.986897025) / 1000.0;
        if totalSugar == 1.0 {
            let sugar = format!("{:.0} kilo", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        } else {
            let sugar = format!("{:.2} kilos", totalSugar);
            champagneCarbonationOutput.set_text(&sugar);
        }
    }
}