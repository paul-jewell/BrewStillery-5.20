use gtk;
use gtk::prelude::*;
use gdk::DisplayExt;
use gdk::RGBA;

use functions::guestimateABV::*;
use functions::increaseABV::*;
use functions::realABV::*;
use functions::waterSparge::*;
use functions::gyleCarbonation::*;
use functions::guestimateIBU::*;
use functions::champagneCarbonation::*;
use functions::grainToABV::*;


macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


#[derive(Clone)]
pub struct colourOverlay {
    pub overlay: gtk::Overlay,
    pub colourOutput: gtk::ColorButton,
    pub dimplePint: gtk::Image,
    pub nonickPint: gtk::Image,
    pub tulipPint: gtk::Image,
    pub pilsner: gtk::Image,
    pub mafs: gtk::Image,
    pub dimpleHalfPint: gtk::Image,
    pub nonickHalfPint: gtk::Image,
    pub tulipHalfPint: gtk::Image,
}


pub fn startGTK(application: &gtk::Application) {

    let builder = gtk::Builder::new_from_string(include_str!("BrewStillery.glade"));

    let mainWindow: gtk::ApplicationWindow = builder.get_object("mainWindow").unwrap();

    mainWindow.set_application(application);
    mainWindow.connect_delete_event(clone!(mainWindow => move |_, _| {
        mainWindow.destroy();
        Inhibit(false)
    }));


    // css section
    
    let css = gtk::CssProvider::new();
    let cssFile = include_bytes!("BrewStillery.css");
    css.load_from_data(cssFile).unwrap();
    let screen = mainWindow.get_display().unwrap().get_screen(0);
    gtk::StyleContext::add_provider_for_screen(&screen, &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);


    // guestimateABV Section

    let guestimatorInput: gtk::Entry = builder.get_object("guestimatorInput").unwrap();
    guestimatorInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimatePrep(&builder);
        Inhibit(false)
    }));


    // increaseABV Section

    let increaseABVBrixInput: gtk::Entry = builder.get_object("increaseABVBrixInput").unwrap();
    increaseABVBrixInput.connect_key_release_event(clone!(builder => move |_,_key| {
        increaseABVPrep(&builder);
        Inhibit(false)
    }));

    let increaseABVABVInput: gtk::Entry = builder.get_object("increaseABVABVInput").unwrap();
    increaseABVABVInput.connect_key_release_event(clone!(builder => move |_,_key| {
        increaseABVPrep(&builder);
        Inhibit(false)
    }));

    let increaseABVVolumeInput: gtk::Entry = builder.get_object("increaseABVVolumeInput").unwrap();
    increaseABVVolumeInput.connect_key_release_event(clone!(builder => move |_,_key| {
        increaseABVPrep(&builder);
        Inhibit(false)
    }));

    let increaseABVSwitch: gtk::Switch = builder.get_object("increaseABVSwitch").unwrap();
    increaseABVSwitch.connect_state_set(clone!(builder => move |_,_key| {
        increaseABVPrep(&builder);
        Inhibit(false)
    }));


    // realABV Section

    let realABVStartingBrixInput: gtk::Entry = builder.get_object("realABVStartingBrixInput").unwrap();
    realABVStartingBrixInput.connect_key_release_event(clone!(builder => move |_,_key| {
        realABVPrep(&builder);
        Inhibit(false)
    }));

    let realABVFinalBrixInput: gtk::Entry = builder.get_object("realABVFinalBrixInput").unwrap();
    realABVFinalBrixInput.connect_key_release_event(clone!(builder => move |_,_key| {
        realABVPrep(&builder);
        Inhibit(false)
    }));


    // grainABV Section

    let allOverlays = colourOverlay {
        overlay: builder.get_object("grainABVColourOutputOverlay").unwrap(),
        colourOutput: gtk::ColorButton::new(),
        dimplePint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Dimple.png"),
        nonickPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Nonick.png"),
        tulipPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Tulip.png"),
        pilsner: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Pilsner.png"),
        mafs: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/Mafs.png"),
        dimpleHalfPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/DimpleHalf.png"),
        nonickHalfPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/NonickHalf.png"),
        tulipHalfPint: gtk::Image::new_from_file("/usr/share/BrewStillery/glassware/TulipHalf.png"),
    };

    let rgbaZero = RGBA {
        red: 255.0,
        green: 255.0,
        blue: 255.0,
        alpha: 1.0,
    };

    allOverlays.colourOutput.set_tooltip_text("Expected Colour");
    allOverlays.colourOutput.set_sensitive(false);
    allOverlays.colourOutput.set_property_show_editor(false);
    allOverlays.colourOutput.set_rgba(&rgbaZero);

    allOverlays.overlay.add_overlay(&allOverlays.colourOutput);
    allOverlays.overlay.add_overlay(&allOverlays.dimplePint);
    allOverlays.overlay.add_overlay(&allOverlays.nonickPint);
    allOverlays.overlay.add_overlay(&allOverlays.tulipPint);
    allOverlays.overlay.add_overlay(&allOverlays.pilsner);
    allOverlays.overlay.add_overlay(&allOverlays.mafs);
    allOverlays.overlay.add_overlay(&allOverlays.dimpleHalfPint);
    allOverlays.overlay.add_overlay(&allOverlays.nonickHalfPint);
    allOverlays.overlay.add_overlay(&allOverlays.tulipHalfPint);

    allOverlays.overlay.set_child_index(&allOverlays.dimplePint, 8);
    allOverlays.overlay.set_child_index(&allOverlays.nonickPint, 7);
    allOverlays.overlay.set_child_index(&allOverlays.tulipPint, 6);
    allOverlays.overlay.set_child_index(&allOverlays.pilsner, 5);
    allOverlays.overlay.set_child_index(&allOverlays.mafs, 4);
    allOverlays.overlay.set_child_index(&allOverlays.dimpleHalfPint, 3);
    allOverlays.overlay.set_child_index(&allOverlays.nonickHalfPint, 2);
    allOverlays.overlay.set_child_index(&allOverlays.tulipHalfPint, 1);
    allOverlays.overlay.set_child_index(&allOverlays.colourOutput, 0);

    allOverlays.nonickPint.set_opacity(0.0);
    allOverlays.tulipPint.set_opacity(0.0);
    allOverlays.pilsner.set_opacity(0.0);
    allOverlays.mafs.set_opacity(0.0);
    allOverlays.dimpleHalfPint.set_opacity(0.0);
    allOverlays.nonickHalfPint.set_opacity(0.0);
    allOverlays.tulipHalfPint.set_opacity(0.0);


    let grainABVWortInput: gtk::Entry = builder.get_object("grainABVWortInput").unwrap();
    grainABVWortInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVFirstAmountInput: gtk::Entry = builder.get_object("grainABVFirstAmountInput").unwrap();
    grainABVFirstAmountInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVFirstTypeInput: gtk::ComboBoxText = builder.get_object("grainABVFirstTypeInput").unwrap();
    grainABVFirstTypeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVSecondAmountInput: gtk::Entry = builder.get_object("grainABVSecondAmountInput").unwrap();
    grainABVSecondAmountInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVSecondTypeInput: gtk::ComboBoxText = builder.get_object("grainABVSecondTypeInput").unwrap();
    grainABVSecondTypeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVThirdAmountInput: gtk::Entry = builder.get_object("grainABVThirdAmountInput").unwrap();
    grainABVThirdAmountInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVThirdTypeInput: gtk::ComboBoxText = builder.get_object("grainABVThirdTypeInput").unwrap();
    grainABVThirdTypeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVFourthAmountInput: gtk::Entry = builder.get_object("grainABVFourthAmountInput").unwrap();
    grainABVFourthAmountInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVFourthTypeInput: gtk::ComboBoxText = builder.get_object("grainABVFourthTypeInput").unwrap();
    grainABVFourthTypeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVFifthAmountInput: gtk::Entry = builder.get_object("grainABVFifthAmountInput").unwrap();
    grainABVFifthAmountInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVFifthTypeInput: gtk::ComboBoxText = builder.get_object("grainABVFifthTypeInput").unwrap();
    grainABVFifthTypeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVSixthAmountInput: gtk::Entry = builder.get_object("grainABVSixthAmountInput").unwrap();
    grainABVSixthAmountInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVSixthTypeInput: gtk::ComboBoxText = builder.get_object("grainABVSixthTypeInput").unwrap();
    grainABVSixthTypeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVSeventhAmountInput: gtk::Entry = builder.get_object("grainABVSeventhAmountInput").unwrap();
    grainABVSeventhAmountInput.connect_key_release_event(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));

    let grainABVSeventhTypeInput: gtk::ComboBoxText = builder.get_object("grainABVSeventhTypeInput").unwrap();
    grainABVSeventhTypeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVGlassSizeInput: gtk::ComboBoxText = builder.get_object("grainABVGlassSizeInput").unwrap();
    grainABVGlassSizeInput.connect_changed(clone!(builder, allOverlays => move |_| {
        grainABVPrep(&builder, &allOverlays);
    }));

    let grainABVSwitch: gtk::Switch = builder.get_object("grainABVSwitch").unwrap();
    grainABVSwitch.connect_state_set(clone!(builder, allOverlays => move |_,_key| {
        grainABVPrep(&builder, &allOverlays);
        Inhibit(false)
    }));


    // waterSparge Section

    let spargePreFermentVolumeInput: gtk::Entry = builder.get_object("spargePreFermentVolumeInput").unwrap();
    spargePreFermentVolumeInput.connect_key_release_event(clone!(builder => move |_,_key| {
        waterSpargePrep(&builder);
        Inhibit(false)
    }));

    let spargeTotalGrainInput: gtk::Entry = builder.get_object("spargeTotalGrainInput").unwrap();
    spargeTotalGrainInput.connect_key_release_event(clone!(builder => move |_,_key| {
        waterSpargePrep(&builder);
        Inhibit(false)
    }));

    let spargeBoilTimeInput: gtk::Entry = builder.get_object("spargeBoilTimeInput").unwrap();
    spargeBoilTimeInput.connect_key_release_event(clone!(builder => move |_,_key| {
        waterSpargePrep(&builder);
        Inhibit(false)
    }));

    let waterSpargeSwitch: gtk::Switch = builder.get_object("waterSpargeSwitch").unwrap();
    waterSpargeSwitch.connect_state_set(clone!(builder => move |_,_key| {
        waterSpargePrep(&builder);
        Inhibit(false)
    }));


    // guestimateIBU Section

    let totalIBUPreBoilBrixInput: gtk::Entry = builder.get_object("totalIBUPreBoilBrixInput").unwrap();
    totalIBUPreBoilBrixInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUWortVolumeInput: gtk::Entry = builder.get_object("totalIBUWortVolumeInput").unwrap();
    totalIBUWortVolumeInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUBoilTimeInput: gtk::Entry = builder.get_object("totalIBUBoilTimeInput").unwrap();
    totalIBUBoilTimeInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFirstHopAlphaInput: gtk::Entry = builder.get_object("totalIBUFirstHopAlphaInput").unwrap();
    totalIBUFirstHopAlphaInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFirstHopAmountInput: gtk::Entry = builder.get_object("totalIBUFirstHopAmountInput").unwrap();
    totalIBUFirstHopAmountInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSecondHopAlphaInput: gtk::Entry = builder.get_object("totalIBUSecondHopAlphaInput").unwrap();
    totalIBUSecondHopAlphaInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSecondHopAmountInput: gtk::Entry = builder.get_object("totalIBUSecondHopAmountInput").unwrap();
    totalIBUSecondHopAmountInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUThirdHopAlphaInput: gtk::Entry = builder.get_object("totalIBUThirdHopAlphaInput").unwrap();
    totalIBUThirdHopAlphaInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUThirdHopAmountInput: gtk::Entry = builder.get_object("totalIBUThirdHopAmountInput").unwrap();
    totalIBUThirdHopAmountInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFourthHopAlphaInput: gtk::Entry = builder.get_object("totalIBUFourthHopAlphaInput").unwrap();
    totalIBUFourthHopAlphaInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFourthHopAmountInput: gtk::Entry = builder.get_object("totalIBUFourthHopAmountInput").unwrap();
    totalIBUFourthHopAmountInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFifthHopAlphaInput: gtk::Entry = builder.get_object("totalIBUFifthHopAlphaInput").unwrap();
    totalIBUFifthHopAlphaInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUFifthHopAmountInput: gtk::Entry = builder.get_object("totalIBUFifthHopAmountInput").unwrap();
    totalIBUFifthHopAmountInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSixthHopAlphaInput: gtk::Entry = builder.get_object("totalIBUSixthHopAlphaInput").unwrap();
    totalIBUSixthHopAlphaInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSixthHopAmountInput: gtk::Entry = builder.get_object("totalIBUSixthHopAmountInput").unwrap();
    totalIBUSixthHopAmountInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSeventhHopAlphaInput: gtk::Entry = builder.get_object("totalIBUSeventhHopAlphaInput").unwrap();
    totalIBUSeventhHopAlphaInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSeventhHopAmountInput: gtk::Entry = builder.get_object("totalIBUSeventhHopAmountInput").unwrap();
    totalIBUSeventhHopAmountInput.connect_key_release_event(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));

    let totalIBUSwitch: gtk::Switch = builder.get_object("totalIBUSwitch").unwrap();
    totalIBUSwitch.connect_state_set(clone!(builder => move |_,_key| {
        guestimateIBUPrep(&builder);
        Inhibit(false)
    }));


    // gyleCarbonation Section

    let gyleBrixInput: gtk::Entry = builder.get_object("gyleBrixInput").unwrap();
    gyleBrixInput.connect_key_release_event(clone!(builder => move |_,_key| {
        gyleCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let gyleCO2Input: gtk::Entry = builder.get_object("gyleCO2Input").unwrap();
    gyleCO2Input.connect_key_release_event(clone!(builder => move |_,_key| {
        gyleCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let gyleWortVolumeInput: gtk::Entry = builder.get_object("gyleWortVolumeInput").unwrap();
    gyleWortVolumeInput.connect_key_release_event(clone!(builder => move |_,_key| {
        gyleCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let gyleCarbonationSwitch: gtk::Switch = builder.get_object("gyleCarbonationSwitch").unwrap();
    gyleCarbonationSwitch.connect_state_set(clone!(builder => move |_,_key| {
        gyleCarbonationPrep(&builder);
        Inhibit(false)
    }));


    // champagneCarbonation Section

    let champagneCarbonationInput: gtk::Entry = builder.get_object("champagneCarbonationInput").unwrap();
    champagneCarbonationInput.connect_key_release_event(clone!(builder => move |_,_key| {
        champagneCarbonationPrep(&builder);
        Inhibit(false)
    }));

    let champagneCarbonationSwitch: gtk::Switch = builder.get_object("champagneCarbonationSwitch").unwrap();
    champagneCarbonationSwitch.connect_state_set(clone!(builder => move |_,_key| {
        champagneCarbonationPrep(&builder);
        Inhibit(false)
    }));

    mainWindow.show_all();
}