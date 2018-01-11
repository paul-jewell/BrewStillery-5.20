use gdk::RGBA;
use light::colour::computedIlluminantData::computedIlluminantData;
use light::colour::x2Data::x2Data;
use light::colour::y2Data::y2Data;
use light::colour::z2Data::z2Data;
use light::main::reconstructedTransmissionData::reconstructedTransmissionData;

pub fn twoArraySum(firstArray: [f64; 81], secondArray: [f64; 81]) -> f64 {
    let mut sum = 0.0;

    // this does the weird spreadsheet thing of (array1[0] * array2[0]) + (array1[0] * array2[0]) ...
    for index in 0..81 {
        sum = sum + firstArray[index] * secondArray[index];
    }

    sum
}

pub fn threeArraySum(firstArray: [f64; 81], secondArray: [f64; 81], thirdArray: [f64; 81]) -> f64 {
    let mut sum = 0.0;

    // this does the weird spreadsheet thing of (array1[0] * array2[0]) + (array1[0] * array2[0]) ...
    for index in 0..81 {
        sum = sum + firstArray[index] * secondArray[index] * thirdArray[index];
    }

    sum
}

pub fn singleMCU(volumeInGallons: f64, weightInPounds: f64, grainLVB: f64) -> f64 {
    let singleMCU = (grainLVB * weightInPounds) / volumeInGallons;
    singleMCU
}

pub fn beerSRM(totalMCU: f64) -> f64 {
    let beerSRM = 1.4922 * totalMCU.powf(0.6859);
    beerSRM
}

pub fn grainSRMToLAB(glassDiameter: f64, beerSRM: f64) -> (f64, f64, f64) {
    let randomAssEquation = 100.0 / twoArraySum(computedIlluminantData(), y2Data());
    let Y2 = randomAssEquation * threeArraySum(reconstructedTransmissionData(glassDiameter, beerSRM), y2Data(), computedIlluminantData());
    let X2 = randomAssEquation * threeArraySum(reconstructedTransmissionData(glassDiameter, beerSRM), x2Data(), computedIlluminantData());
    let Z2 = randomAssEquation * threeArraySum(reconstructedTransmissionData(glassDiameter, beerSRM), z2Data(), computedIlluminantData());
    let whitePointXr2 = twoArraySum(computedIlluminantData(), x2Data()) * randomAssEquation;
    let whitePointYr2 = twoArraySum(computedIlluminantData(), y2Data()) * randomAssEquation;
    let whitePointZr2  = twoArraySum(computedIlluminantData(), z2Data()) * randomAssEquation;

    let fY = if (Y2 / whitePointYr2) > 0.008856 {
        (Y2 / whitePointYr2).powf(1.0/3.0)
    } else {
        7.787 * (Y2 / whitePointYr2) + 16.0/116.0
    };

    let fZ = if (Z2 / whitePointZr2) > 0.008856 {
        (Z2 / whitePointZr2).powf(1.0/3.0)
    } else {
        7.787 * (Z2 / 86.0) + 16.0 / 116.0
    };

    let fX = if (X2 / whitePointXr2) > 0.008856 {
        (X2 / whitePointXr2).powf(1.0/3.0)
    } else {
        7.787 * (X2 / 86.0) +16.0 / 116.0
    };

    let reconstructedL = 116.0 * fY - 16.0;
    let reconstructedA = 500.0 * (fX - fY);
    let reconstructedB = 200.0 * (fY - fZ);

    (reconstructedL, reconstructedA, reconstructedB)
}

pub fn grainLABToXYZ(L: f64, a: f64, b: f64) -> (f64, f64, f64) {
    let mut yComputed = ( L + 16.0 ) / 116.0;
    let mut xComputed = a / 500.0 + yComputed;
    let mut zComputed = yComputed - b / 200.0;

    if yComputed.powi(3)  > 0.008856 {
        yComputed = yComputed.powi(3)
    } else {
        yComputed = ( yComputed - 16.0 / 116.0 ) / 7.787
    } if xComputed.powi(3)  > 0.008856 {
        xComputed = xComputed.powi(3)
    } else {
        xComputed = ( xComputed - 16.0 / 116.0 ) / 7.787
    } if zComputed.powi(3)  > 0.008856 {
        zComputed = zComputed.powi(3)
    } else {
        zComputed = ( zComputed - 16.0 / 116.0 ) / 7.787
    }

    // D65/2°
    // X_2 = 95.047
    // Y_2 = 100.000
    // Z_2 = 108.883
    // D65/10°
    // X_10 = 94.811
    // Y_10 = 100.000
    // Z_10 = 107.304
    // Daylight, sRGB, Adobe-RGB

    let xDaylight = 95.047;
    let yDaylight = 100.000;
    let zDaylight = 108.883;

    let xFinal = xComputed * xDaylight;
    let yFinal = yComputed * yDaylight;
    let zFinal = zComputed * zDaylight;

    (xFinal, yFinal, zFinal)
}

pub fn grainXYZToRGBA(xInput: f64, yInput: f64, zInput: f64) -> RGBA {
    //X, Y and Z input refer to a D65/2° standard illuminant.
    //sR, sG and sB (standard RGB) output range = 0 ÷ 255

    let var_X = xInput / 100.0;
    let var_Y = yInput / 100.0;
    let var_Z = zInput / 100.0;

    let mut var_R = var_X *  3.2406 + var_Y * -1.5372 + var_Z * -0.4986;
    let mut var_G = var_X * -0.9689 + var_Y *  1.8758 + var_Z *  0.0415;
    let mut var_B = var_X *  0.0557 + var_Y * -0.2040 + var_Z *  1.0570;

    if var_R > 0.0031308 {
        var_R = 1.055 * var_R.powf(1.0 / 2.4) - 0.055
    } else {
        var_R = 12.92 * var_R
    }

    if var_G > 0.0031308 {
        var_G = 1.055 * var_G.powf(1.0 / 2.4) - 0.055
    } else {
        var_G = 12.92 * var_G
    }

    if var_B > 0.0031308 {
        var_B = 1.055 * var_B.powf(1.0 / 2.4) - 0.055
    } else {
        var_B = 12.92 * var_B
    }

    let colourValues = RGBA {
        red: (var_R * 255.0) / 256.0,
        green: (var_G * 255.0) / 256.0,
        blue: (var_B * 255.0) / 256.0,
        alpha: 1.0,
    };

    colourValues
}