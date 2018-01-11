use std::f64::consts::E;

pub const finalGravityIdeal: f64 = 1.01627;
// when finalGravity is unknown, this constant is ideal. it is the average of all BJCP styles

pub const finalBrixIdeal: f64 = 4.1480675;
// when finalBrix is unknown, this constant is ideal. it is the average of all BJCP styles

pub fn brixToGravity(brix: f64) -> f64 {
    (brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.0
}

pub fn gravityToBrix(gravity: f64) -> f64 {
    ((258.6 * gravity) - 258.6) / ((0.87955073 * gravity) + 0.12044926)
}

pub fn gravityToPlato(gravity: f64) -> f64 {
    135.997 * gravity.powi(3) - 630.272 * gravity.powi(2) + 1111.14 * gravity - 616.868
}

pub fn gramsToOunces(grams: f64) -> f64 {
    0.03527396_f64 * grams
}

pub fn kilosToPounds(kilos: f64) -> f64 {
    2.204623_f64 * kilos
}

pub fn litresToGallons(litres: f64) -> f64 {
    0.2641729_f64 * litres
}

pub fn gallonsToLitres(gallons: f64) -> f64 {
    3.7854_f64 * gallons
}

pub fn validInput(stringToCheck: &str) -> f64 {
    let stringInput = stringToCheck.parse::<f64>();
    match stringInput {
        Ok(number) => number,
        Err(_) => 0.0,
    }
}

pub fn realIBU(brix: f64, wortVolume: f64, boilTime: f64, alphaAcid: f64, hopAmount: f64) -> f64 {
    (1.65 * 0.000125_f64.powf(brixToGravity(brix) - 1.0)) * ((1.0 - E.powf(-0.04 * boilTime)) / 4.15) * (( (alphaAcid / 100.0) * hopAmount * 7490.0 ) / wortVolume)
}

pub fn realABV(startingBrix: f64, finalBrix: f64) -> (f64, f64) {
    let wortCorrectionFactor: f64 = 1.040;
    let initialRefractiveIndex = startingBrix / wortCorrectionFactor;
    let finalRefractiveIndex = finalBrix / wortCorrectionFactor;

    let newCubic = (1.0 - 0.0044992999999999995 * initialRefractiveIndex) + (0.0117741 * finalRefractiveIndex) + (0.000275806 * initialRefractiveIndex.powi(2)) - (0.00127169 * finalRefractiveIndex.powi(2)) - (7.27999e-06 * initialRefractiveIndex.powi(3)) + (6.32929e-05 * finalRefractiveIndex.powi(3));

    let apparentAttenuation = 1.0 - (0.18080000000000002 * (668.72 * newCubic - 463.37 - 205.347 * newCubic.powi(2)) + 0.8192 * (668.72 * newCubic - 463.37 - 205.347 * newCubic.powi(2))) / (initialRefractiveIndex);

    let attenuation = apparentAttenuation * 0.8192;

    let abv = ((0.01 / 0.8192) * (initialRefractiveIndex - (0.18080000000000002 * initialRefractiveIndex + 0.8192 * (668.72 * newCubic - 463.37 - 205.347 * newCubic.powi(2)))) / (2.0665-0.010665 * initialRefractiveIndex)) * 100.0;

    (abv, attenuation)
}

pub fn grainToABV(volumeInGallons: f64, weightInPounds: f64, grainGravity: f64) -> f64 {
    let extractPotential = grainGravity % 1.0 * 1000.0;
    let extractionEfficiency = 0.57;
    // sane default here of 57%
    let mut originalGravity = 1.0;
    // set to the gravity of water

    let specificGravityPoints = (weightInPounds * extractPotential * extractionEfficiency) / volumeInGallons;

    if specificGravityPoints != 0.0 {
        originalGravity = specificGravityPoints / 1000.0 + 1.0;
        originalGravity
    } else {
        originalGravity
    }
}

pub fn grainInfo(grainType: &str) -> (f64, f64) {
    match grainType {
        // First value is Gravity, second value is Lovibond
        "2-Row" => (1.037, 1.8),
        "6-Row" => (1.035, 1.8),
        "Black" => (1.025, 500.0),
        "Caramel 60" => (1.034, 60.0),
        "Caramel 80" => (1.034, 80.0),
        "Caramel 120" => (1.033, 120.0),
        "Chocolate" => (1.034, 350.0),
        "Corn" => (1.037, 1.3),
        "Dextrine" => (1.033, 1.5),
        "Oat" => (1.034, 1.0),
        "Rice" => (1.032, 1.0),
        "Rye" => (1.036, 2.0),
        "Wheat" => (1.036, 1.6),
        _ => (1.0, 0.0),
        // Gravity set to water, and color to nothing as a catch
    }
}

pub fn glassSize(glassType: &str) -> f64 {
    match glassType {
        // Value is in centimeters
        "Dimple Pint" => 9.8,
        "Nonick Pint" => 9.0,
        "Tulip Pint" => 8.5,
        "Pilsner" => 8.0,
        "Maß" => 10.5,
        "Dimple Half Pint" => 8.0,
        "Nonick Half Pint" => 7.0,
        "Tulip Half Pint" => 7.2,
        _ => 9.8,
    }
}