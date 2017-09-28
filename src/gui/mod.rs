#![allow(non_snake_case)]

use gtk::{self};
use gtk::prelude::*;

use functions::guestimateABV::*;
use functions::increaseABV::*;
use functions::realABV::*;
use functions::waterSparge::*;
use functions::gyleCarbonation::*;
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

    let gyleFinalVolumeInput: gtk::Entry = builder.get_object("gyleFinalVolumeInput").unwrap();
    let gyleFinalVolumeInputBuilderClone = builder.clone();
    gyleFinalVolumeInput.connect_activate(move |_| {
        gyleCarbonationPrep(&gyleFinalVolumeInputBuilderClone);
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