use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn guestimatePrep(guestimatorBuilderClone: &gtk::Builder) {
    let guestimatorInput: gtk::Entry = guestimatorBuilderClone.get_object("guestimatorInput").unwrap();
    let guestimatorInput = guestimatorInput.get_text().expect("No input");
    let startingBrix = validInput(&guestimatorInput);
    let guestimatorOutput: gtk::Entry = guestimatorBuilderClone.get_object("guestimatorOutput").unwrap();

    if startingBrix == 0.0 {
        guestimatorOutput.set_text("Enter a number");
    } else if startingBrix < 2.57 {
        guestimatorOutput.set_text("Enter a Brix greater than 2.57");
    } else if startingBrix > 49.48 {
        guestimatorOutput.set_text("Enter a Brix less than 49.48");
    } else {
        guestiMaths(startingBrix, guestimatorBuilderClone);
    }
}

fn guestiMaths(startingBrix: f64, guestimatorBuilderClone: &gtk::Builder) {
    let guestimatorOutput: gtk::Entry = guestimatorBuilderClone.get_object("guestimatorOutput").unwrap();

    let abv = format!("{:.2}%", realABV(startingBrix, finalBrixIdeal).0);
    guestimatorOutput.set_text(&abv);
}