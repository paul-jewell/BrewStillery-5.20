use light::eigenVectors::averageData::averageData;

pub fn reconstructedTransmissionData(glassDiameter: f64, beerSRM: f64) -> [f64; 81] {
    let mut reconstructedTransmissionList: [f64; 81] = [0.0; 81];

    let averageData = averageData();

    // index is wavelengths of light, stepped by 5, from 380 to 780
    reconstructedTransmissionList[0] = (averageData[0].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[1] = (averageData[1].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[2] = (averageData[2].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[3] = (averageData[3].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[4] = (averageData[4].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[5] = (averageData[5].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[6] = (averageData[6].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[7] = (averageData[7].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[8] = (averageData[8].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[9] = (averageData[9].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[10] = (averageData[10].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[11] = (averageData[11].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[12] = (averageData[12].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[13] = (averageData[13].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[14] = (averageData[14].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[15] = (averageData[15].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[16] = (averageData[16].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[17] = (averageData[17].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[18] = (averageData[18].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[19] = (averageData[19].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[20] = (averageData[20].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[21] = (averageData[21].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[22] = (averageData[22].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[23] = (averageData[23].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[24] = (averageData[24].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[25] = (averageData[25].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[26] = (averageData[26].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[27] = (averageData[27].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[28] = (averageData[28].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[29] = (averageData[29].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[30] = (averageData[30].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[31] = (averageData[31].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[32] = (averageData[32].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[33] = (averageData[33].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[34] = (averageData[34].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[35] = (averageData[35].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[36] = (averageData[36].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[37] = (averageData[37].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[38] = (averageData[38].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[39] = (averageData[39].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[40] = (averageData[40].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[41] = (averageData[41].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[42] = (averageData[42].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[43] = (averageData[43].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[44] = (averageData[44].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[45] = (averageData[45].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[46] = (averageData[46].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[47] = (averageData[47].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[48] = (averageData[48].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[49] = (averageData[49].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[50] = (averageData[50].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[51] = (averageData[51].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[52] = (averageData[52].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[53] = (averageData[53].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[54] = (averageData[54].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[55] = (averageData[55].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[56] = (averageData[56].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[57] = (averageData[57].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[58] = (averageData[58].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[59] = (averageData[59].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[60] = (averageData[60].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[61] = (averageData[61].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[62] = (averageData[62].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[63] = (averageData[63].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[64] = (averageData[64].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[65] = (averageData[65].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[66] = (averageData[66].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[67] = (averageData[67].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[68] = (averageData[68].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[69] = (averageData[69].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[70] = (averageData[70].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[71] = (averageData[71].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[72] = (averageData[72].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[73] = (averageData[73].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[74] = (averageData[74].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[75] = (averageData[75].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[76] = (averageData[76].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[77] = (averageData[77].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[78] = (averageData[78].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[79] = (averageData[79].powf(glassDiameter)).powf(beerSRM / 12.7);
    reconstructedTransmissionList[80] = (averageData[80].powf(glassDiameter)).powf(beerSRM / 12.7);

    reconstructedTransmissionList
}