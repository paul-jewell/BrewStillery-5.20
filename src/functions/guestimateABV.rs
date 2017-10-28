use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn guestimatePrep(ref guestimatorBuilderClone: &gtk::Builder) {
    let guestimatorInput: &gtk::Entry = &guestimatorBuilderClone.get_object("guestimatorInput").unwrap();
    let guestimatorInput = guestimatorInput.get_text().expect("No input");
    let startingBrix = validInput(&guestimatorInput);

    let guestimatorOutput = String::from("guestimatorOutput");

    if startingBrix == 0.0 {
        let output: gtk::Entry = guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        if startingBrix < 2.57 {
            let output: gtk::Entry = guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
            output.set_text("Enter a number greater than 2.57");
        } else if startingBrix > 49.48 {
            let output: gtk::Entry = guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
            output.set_text("Enter a number less than 49.48");
        } else {
            guestiMaths(startingBrix, guestimatorOutput, &guestimatorBuilderClone);
        }
    }
}

fn guestiMaths(startingBrix: f32, guestimatorOutput: String, ref guestimatorBuilderClone: &gtk::Builder) {
    let abv = format!("{:.2}%", guestimateABV(brixToGravity(startingBrix)));
    let ref output: &gtk::Entry = &guestimatorBuilderClone.get_object(&guestimatorOutput).unwrap();
    output.set_text(&abv.to_string());
}