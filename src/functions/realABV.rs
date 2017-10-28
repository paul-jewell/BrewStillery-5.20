use gtk;
use gtk::prelude::*;
use functions::commonFunctions::*;

pub fn realABVPrep(ref realABVBuilderClone: &gtk::Builder) {
    let realABVStartingBrixInput: &gtk::Entry = &realABVBuilderClone.get_object("realABVStartingBrixInput").unwrap();
    let realABVStartingBrixInputBuffer = realABVStartingBrixInput.get_text().expect("No input");
    let startingBrix = validInput(&realABVStartingBrixInputBuffer);

    let realABVFinalBrixInput: &gtk::Entry = &realABVBuilderClone.get_object("realABVFinalBrixInput").unwrap();
    let realABVFinalBrixInputBuffer = realABVFinalBrixInput.get_text().expect("No input");
    let finalBrix = validInput(&realABVFinalBrixInputBuffer);

    let realABVFinalABVOutput = String::from("realABVFinalABVOutput");

    if startingBrix == 0.0 {
        let output: gtk::Entry = realABVBuilderClone.get_object(&realABVFinalABVOutput).unwrap();
        output.set_text("Enter a number");
    } else if finalBrix == 0.0 {
        let output: gtk::Entry = realABVBuilderClone.get_object(&realABVFinalABVOutput).unwrap();
        output.set_text("Enter a number");
    } else {
        if startingBrix <= 0.0 {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVFinalABVOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if finalBrix <= 0.0 {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVFinalABVOutput).unwrap();
            output.set_text("Enter a positive number");
        } else if startingBrix < finalBrix {
            let output: gtk::Entry = realABVBuilderClone.get_object(&realABVFinalABVOutput).unwrap();
            output.set_text("Starting brix must be greater than final brix");
        } else {
            realABVMaths(startingBrix, finalBrix, &realABVBuilderClone);
        }
    }
}

fn realABVMaths(startingBrix: f32, finalBrix: f32, ref realABVBuilderClone: &gtk::Builder) {
    let ref realABVFinalABVOutput: &gtk::Entry = &realABVBuilderClone.get_object("realABVFinalABVOutput").unwrap();

    let finalGravity = 1.001843 - (0.002318474 * startingBrix) - (0.000007775 * startingBrix.powi(2)) - (0.000000034 * startingBrix.powi(3)) + (0.00574 * finalBrix) + (0.00003344 * finalBrix.powi(2)) + (0.000000086 * finalBrix.powi(3));
    let refractiveIndex = 1.33302 + (0.001427193 * finalBrix) + (0.000005791157 * finalBrix.powi(2));
    let abw = 1017.5596 - (277.4 * finalGravity) + refractiveIndex * ((937.8135 * refractiveIndex) - 1805.1228);
    let abv = format!("{:.2}%", abw * (finalGravity / 0.794));
    realABVFinalABVOutput.set_text(&abv);
}