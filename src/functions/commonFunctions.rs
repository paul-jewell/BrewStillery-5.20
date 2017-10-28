use std::f32::consts::E;

pub fn brixToGravity(brix: f32) -> f32 {
    (brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.0
}

pub fn gravityToBrix(gravity: f32) -> f32 {
    ((258.6 * gravity) - 258.6) / ((0.87955073 * gravity) + 0.12044926)
}

pub fn gravityToPlato(gravity: f32) -> f32 {
    135.997 * gravity.powi(3) - 630.272 * gravity.powi(2) + 1111.14 * gravity - 616.868
}

pub fn gramsToOunces(grams: f32) -> f32 {
    0.03527396_f32 * grams
}

pub fn kilosToPounds(kilos: f32) -> f32 {
    2.204623_f32 * kilos
}

pub fn litresToGallons(litres: f32) -> f32 {
    0.2641729_f32 * litres
}

pub fn validInput(stringToCheck: &String) -> f32 {
    let stringInput = stringToCheck.parse::<f32>();
    match stringInput {
        Ok(number) => number,
        Err(_) => 0.0,
    }
}

pub fn realIBU(brix: f32, wortVolume: f32, boilTime: f32, alphaAcid: f32, hopAmount: f32) -> f32 {
    (1.65 * 0.000125_f32.powf(brixToGravity(brix) - 1.0)) * ((1.0 - E.powf(-0.04 * boilTime)) / 4.15) * (( (alphaAcid / 100.0) * hopAmount * 7490.0 ) / wortVolume)
}

pub fn guestimateABV(originalGravity: f32) -> f32 {
    let finalGravity: f32 = 1.010;
    // since finalGravity is unknown, this constant is ideal
    let originalExtract = (-668.962) + (1262.45 * originalGravity ) - (776.43 * originalGravity.powi(2)) + (182.94 * originalGravity.powi(3));
    let apparentExtract = (-668.962) + (1262.45 * finalGravity ) - (776.43 * finalGravity.powi(2)) + (182.94 * finalGravity.powi(3));
    let attenuationCoefficient = (0.22) + (0.001 * originalExtract);
    let realExtract = ((attenuationCoefficient * originalExtract) + apparentExtract) / (1.0 + attenuationCoefficient);

    // unneeded information, but here if we want it
    //
    // let realAttenuation = ((originalExtract - realExtract) / originalExtract) * 100.0;
    // let realAttenuationRounded = format!("{:.2}", realAttenuation);
    // println!("Your Real Attenuation is: {}", realAttenuationRounded);

    let estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract));
    let estimatedABV = estimatedABW * (finalGravity / 0.794);
    estimatedABV
}

pub fn grainToABV(volumeInGallons: f32, weightInPounds: f32, grainGravity: f32) -> f32 {
    let extractPotential = grainGravity % 1.0 * 1000.0;
    let extractionEfficiency = 0.57;
    // sane default here of 57%
    let specificGravityPoints = (weightInPounds * extractPotential * extractionEfficiency) / volumeInGallons;
    if specificGravityPoints == 0.0 {
        let originalGravity = 1.0;
        // set to the gravity of water
        originalGravity
    } else {
        let originalGravity = specificGravityPoints / 1000.0 + 1.0;
        originalGravity
    }
}

pub fn grainInfo(grainType: &String) -> f32 {
    match grainType.as_ref() {
        "2-Row PG" => 1.037,
        "2-Row LVB" => 1.8,
        "6-Row PG" => 1.035,
        "6-Row LVB" => 1.8,
        "Black PG" => 1.025,
        "Black LVB" => 500.0,
        "Caramel PG" => 1.034,
        "Caramel 60 LVB" => 60.0,
        "Caramel 80 LVB" => 80.0,
        "Caramel 120 LVB" => 120.0,
        "Chocolate PG" => 1.034,
        "Chocolate LVB" => 350.0,
        "Corn PG" => 1.037,
        "Corn LVB" => 1.3,
        "Dextrine PG" => 1.033,
        "Dextrine LVB" => 1.5,
        "Oat PG" => 1.034,
        "Oat LVB" => 1.0,
        "Rice PG" => 1.032,
        "Rice LVB" => 1.0,
        "Rye PG" => 1.036,
        "Rye LVB" => 2.0,
        "Wheat PG" => 1.036,
        "Wheat LVB" => 1.6,
        _ => 0.0,
    }
}