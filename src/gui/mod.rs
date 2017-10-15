#![allow(non_snake_case)]

use gtk;
use gtk::prelude::*;

use functions::guestimateABV::*;
use functions::increaseABV::*;
use functions::realABV::*;
use functions::waterSparge::*;
use functions::gyleCarbonation::*;
use functions::guestimateIBU::*;
use functions::champagneCarbonation::*;

pub fn startGTK() {
    if gtk::init().is_err() {
        println!("Failed to initialise GTK", );
        return;
    }

    let builder = gtk::Builder::new_from_string(include_str!("BrewStillery.glade"));

    let mainWindow: gtk::Window = builder.get_object("mainWindow").unwrap();

    mainWindow.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    // guestimateABV Section

    let guestimatorButton: gtk::Button = builder.get_object("guestimatorButton").unwrap();
    let guestimatorButtonBuilderClone = builder.clone();
    guestimatorButton.connect_clicked(move |_| {
        guestimatePrep(&guestimatorButtonBuilderClone);
    });

    let guestimatorInput: gtk::Entry = builder.get_object("guestimatorInput").unwrap();
    let guestimatorInputBuilderClone = builder.clone();
    guestimatorInput.connect_activate(move |_| {
        guestimatePrep(&guestimatorInputBuilderClone);
    });


    // increaseABV Section

    let increaseABVButton: gtk::Button = builder.get_object("increaseABVButton").unwrap();
    let increaseButtonBuilderClone = builder.clone();
    increaseABVButton.connect_clicked(move |_| {
        increaseABVPrep(&increaseButtonBuilderClone);
    });

    let increaseABVBrixInput: gtk::Entry = builder.get_object("increaseABVBrixInput").unwrap();
    let increaseABVBrixInputBuilderClone = builder.clone();
    increaseABVBrixInput.connect_activate(move |_| {
        increaseABVPrep(&increaseABVBrixInputBuilderClone);
    });

    let increaseABVABVInput: gtk::Entry = builder.get_object("increaseABVABVInput").unwrap();
    let increaseABVABVInputBuilderClone = builder.clone();
    increaseABVABVInput.connect_activate(move |_| {
        increaseABVPrep(&increaseABVABVInputBuilderClone);
    });

    let increaseABVVolumeInput: gtk::Entry = builder.get_object("increaseABVVolumeInput").unwrap();
    let increaseABVVolumeInputBuilderClone = builder.clone();
    increaseABVVolumeInput.connect_activate(move |_| {
        increaseABVPrep(&increaseABVVolumeInputBuilderClone);
    });


    // realABV Section

    let realABVButton: gtk::Button = builder.get_object("realABVButton").unwrap();
    let realButtonBuilderClone = builder.clone();
    realABVButton.connect_clicked(move |_| {
        realABVPrep(&realButtonBuilderClone);
    });

    let realABVStartingBrixInput: gtk::Entry = builder.get_object("realABVStartingBrixInput").unwrap();
    let realABVStartingBrixInputBuilderClone = builder.clone();
    realABVStartingBrixInput.connect_activate(move |_| {
        realABVPrep(&realABVStartingBrixInputBuilderClone);
    });

    let realABVFinalBrixInput: gtk::Entry = builder.get_object("realABVFinalBrixInput").unwrap();
    let realABVFinalBrixInputBuilderClone = builder.clone();
    realABVFinalBrixInput.connect_activate(move |_| {
        realABVPrep(&realABVFinalBrixInputBuilderClone);
    });


    // waterSparge Section

    let spargeTotalWaterButton: gtk::Button = builder.get_object("spargeTotalWaterButton").unwrap();
    let spargeTotalWaterButtonClone = builder.clone();
    spargeTotalWaterButton.connect_clicked(move |_| {
        waterSpargePrep(&spargeTotalWaterButtonClone);
    });

    let spargePreFermentVolumeInput: gtk::Entry = builder.get_object("spargePreFermentVolumeInput").unwrap();
    let spargePreFermentVolumeInputBuilderClone = builder.clone();
    spargePreFermentVolumeInput.connect_activate(move |_| {
        waterSpargePrep(&spargePreFermentVolumeInputBuilderClone);
    });

    let spargeTotalGrainInput: gtk::Entry = builder.get_object("spargeTotalGrainInput").unwrap();
    let spargeTotalGrainInputBuilderClone = builder.clone();
    spargeTotalGrainInput.connect_activate(move |_| {
        waterSpargePrep(&spargeTotalGrainInputBuilderClone);
    });

    let spargeBoilTimeInput: gtk::Entry = builder.get_object("spargeBoilTimeInput").unwrap();
    let spargeBoilTimeInputBuilderClone = builder.clone();
    spargeBoilTimeInput.connect_activate(move |_| {
        waterSpargePrep(&spargeBoilTimeInputBuilderClone);
    });


    // gyleCarbonation Section

    let gyleButton: gtk::Button = builder.get_object("gyleButton").unwrap();
    let gyleButtonClone = builder.clone();
    gyleButton.connect_clicked(move |_| {
        gyleCarbonationPrep(&gyleButtonClone);
    });

    let gyleBrixInput: gtk::Entry = builder.get_object("gyleBrixInput").unwrap();
    let gyleBrixInputBuilderClone = builder.clone();
    gyleBrixInput.connect_activate(move |_| {
        gyleCarbonationPrep(&gyleBrixInputBuilderClone);
    });

    let gyleCO2Input: gtk::Entry = builder.get_object("gyleCO2Input").unwrap();
    let gyleCO2InputBuilderClone = builder.clone();
    gyleCO2Input.connect_activate(move |_| {
        gyleCarbonationPrep(&gyleCO2InputBuilderClone);
    });

    let gyleWortVolumeInput: gtk::Entry = builder.get_object("gyleWortVolumeInput").unwrap();
    let gyleWortVolumeInputBuilderClone = builder.clone();
    gyleWortVolumeInput.connect_activate(move |_| {
        gyleCarbonationPrep(&gyleWortVolumeInputBuilderClone);
    });


    // guestimateIBU Section

    let totalIBUButton: gtk::Button = builder.get_object("totalIBUButton").unwrap();
    let totalIBUButtonClone = builder.clone();
    totalIBUButton.connect_clicked(move |_| {
        guestimateIBUPrep(&totalIBUButtonClone);
    });

    let totalIBUPreBoilBrixInput: gtk::Entry = builder.get_object("totalIBUPreBoilBrixInput").unwrap();
    let totalIBUPreBoilBrixInputBuilderClone = builder.clone();
    totalIBUPreBoilBrixInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUPreBoilBrixInputBuilderClone);
    });

    let totalIBUWortVolumeInput: gtk::Entry = builder.get_object("totalIBUWortVolumeInput").unwrap();
    let totalIBUWortVolumeInputBuilderClone = builder.clone();
    totalIBUWortVolumeInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUWortVolumeInputBuilderClone);
    });

    let totalIBUBoilTimeInput: gtk::Entry = builder.get_object("totalIBUBoilTimeInput").unwrap();
    let totalIBUBoilTimeInputBuilderClone = builder.clone();
    totalIBUBoilTimeInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUBoilTimeInputBuilderClone);
    });

    let totalIBUFirstHopAlphaInput: gtk::Entry = builder.get_object("totalIBUFirstHopAlphaInput").unwrap();
    let totalIBUFirstHopAlphaInputBuilderClone = builder.clone();
    totalIBUFirstHopAlphaInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUFirstHopAlphaInputBuilderClone);
    });

    let totalIBUFirstHopAmountInput: gtk::Entry = builder.get_object("totalIBUFirstHopAmountInput").unwrap();
    let totalIBUFirstHopAmountInputBuilderClone = builder.clone();
    totalIBUFirstHopAmountInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUFirstHopAmountInputBuilderClone);
    });

    let totalIBUSecondHopAlphaInput: gtk::Entry = builder.get_object("totalIBUSecondHopAlphaInput").unwrap();
    let totalIBUSecondHopAlphaInputBuilderClone = builder.clone();
    totalIBUSecondHopAlphaInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUSecondHopAlphaInputBuilderClone);
    });

    let totalIBUSecondHopAmountInput: gtk::Entry = builder.get_object("totalIBUSecondHopAmountInput").unwrap();
    let totalIBUSecondHopAmountInputBuilderClone = builder.clone();
    totalIBUSecondHopAmountInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUSecondHopAmountInputBuilderClone);
    });

    let totalIBUThirdHopAlphaInput: gtk::Entry = builder.get_object("totalIBUThirdHopAlphaInput").unwrap();
    let totalIBUThirdHopAlphaInputBuilderClone = builder.clone();
    totalIBUThirdHopAlphaInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUThirdHopAlphaInputBuilderClone);
    });

    let totalIBUThirdHopAmountInput: gtk::Entry = builder.get_object("totalIBUThirdHopAmountInput").unwrap();
    let totalIBUThirdHopAmountInputBuilderClone = builder.clone();
    totalIBUThirdHopAmountInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUThirdHopAmountInputBuilderClone);
    });

    let totalIBUFourthHopAlphaInput: gtk::Entry = builder.get_object("totalIBUFourthHopAlphaInput").unwrap();
    let totalIBUFourthHopAlphaInputBuilderClone = builder.clone();
    totalIBUFourthHopAlphaInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUFourthHopAlphaInputBuilderClone);
    });

    let totalIBUFourthHopAmountInput: gtk::Entry = builder.get_object("totalIBUFourthHopAmountInput").unwrap();
    let totalIBUFourthHopAmountInputBuilderClone = builder.clone();
    totalIBUFourthHopAmountInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUFourthHopAmountInputBuilderClone);
    });

    let totalIBUFifthHopAlphaInput: gtk::Entry = builder.get_object("totalIBUFifthHopAlphaInput").unwrap();
    let totalIBUFifthHopAlphaInputBuilderClone = builder.clone();
    totalIBUFifthHopAlphaInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUFifthHopAlphaInputBuilderClone);
    });

    let totalIBUFifthHopAmountInput: gtk::Entry = builder.get_object("totalIBUFifthHopAmountInput").unwrap();
    let totalIBUFifthHopAmountInputBuilderClone = builder.clone();
    totalIBUFifthHopAmountInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUFifthHopAmountInputBuilderClone);
    });

    let totalIBUSixthHopAlphaInput: gtk::Entry = builder.get_object("totalIBUSixthHopAlphaInput").unwrap();
    let totalIBUSixthHopAlphaInputBuilderClone = builder.clone();
    totalIBUSixthHopAlphaInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUSixthHopAlphaInputBuilderClone);
    });

    let totalIBUSixthHopAmountInput: gtk::Entry = builder.get_object("totalIBUSixthHopAmountInput").unwrap();
    let totalIBUSixthHopAmountInputBuilderClone = builder.clone();
    totalIBUSixthHopAmountInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUSixthHopAmountInputBuilderClone);
    });

    let totalIBUSeventhHopAlphaInput: gtk::Entry = builder.get_object("totalIBUSeventhHopAlphaInput").unwrap();
    let totalIBUSeventhHopAlphaInputBuilderClone = builder.clone();
    totalIBUSeventhHopAlphaInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUSeventhHopAlphaInputBuilderClone);
    });

    let totalIBUSeventhHopAmountInput: gtk::Entry = builder.get_object("totalIBUSeventhHopAmountInput").unwrap();
    let totalIBUSeventhHopAmountInputBuilderClone = builder.clone();
    totalIBUSeventhHopAmountInput.connect_activate(move |_| {
        guestimateIBUPrep(&totalIBUSeventhHopAmountInputBuilderClone);
    });


    // champagneCarbonation Section

    let champagneCarbonationButton: gtk::Button = builder.get_object("champagneCarbonationButton").unwrap();
    let champagneCarbonationButtonClone = builder.clone();
    champagneCarbonationButton.connect_clicked(move |_| {
        champagneCarbonationPrep(&champagneCarbonationButtonClone);
    });

    let champagneCarbonationInput: gtk::Entry = builder.get_object("champagneCarbonationInput").unwrap();
    let champagneCarbonationInputBuilderClone = builder.clone();
    champagneCarbonationInput.connect_activate(move |_| {
        champagneCarbonationPrep(&champagneCarbonationInputBuilderClone);
    });



    mainWindow.show_all();

    gtk::main();
}