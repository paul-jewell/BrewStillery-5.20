def increaseABV
  def beerSettings
    @increaseABVBrixInput = $builder.get_object("increaseABVBrixInputBeer")
    @increaseABVABVInput = $builder.get_object("increaseABVABVInputBeer")
    @increaseABVVolumeInput = $builder.get_object("increaseABVVolumeInputBeer")
    @switch = $builder.get_object("imperialOrMetricSwitchSugarBeer")
    @increaseABVNewBrix = $builder.get_object("increaseABVNewBrixBeer")
    @increaseABVEstimatedABV = $builder.get_object("increaseABVEstimatedABVBeer")
    @sugarAddOutput = $builder.get_object("sugarAddOutputBeer")
    @honeyAddOutput = $builder.get_object("honeyAddOutputBeer")
    onSwitchActivatedSugar
  end
  buttonBeer = $builder.get_object("increaseABVButtonBeer")
  buttonBeer.signal_connect("clicked") {
    beerSettings
  }
  increaseABVBrixInputBeer = $builder.get_object("increaseABVBrixInputBeer")
  increaseABVBrixInputBeer.signal_connect("activate") {
    beerSettings
  }
  increaseABVABVInputBeer = $builder.get_object("increaseABVABVInputBeer")
  increaseABVABVInputBeer.signal_connect("activate") {
    beerSettings
  }
  increaseABVVolumeInputBeer = $builder.get_object("increaseABVVolumeInputBeer")
  increaseABVVolumeInputBeer.signal_connect("activate") {
    beerSettings
  }

  def wineSettings
    @increaseABVBrixInput = $builder.get_object("increaseABVBrixInputWine")
    @increaseABVABVInput = $builder.get_object("increaseABVABVInputWine")
    @increaseABVVolumeInput = $builder.get_object("increaseABVVolumeInputWine")
    @switch = $builder.get_object("imperialOrMetricSwitchSugarWine")
    @increaseABVNewBrix = $builder.get_object("increaseABVNewBrixWine")
    @increaseABVEstimatedABV = $builder.get_object("increaseABVEstimatedABVWine")
    @sugarAddOutput = $builder.get_object("sugarAddOutputWine")
    @honeyAddOutput = $builder.get_object("honeyAddOutputWine")
    onSwitchActivatedSugar
  end
  buttonWine = $builder.get_object("increaseABVButtonWine")
  buttonWine.signal_connect("clicked") {
    wineSettings
  }
  increaseABVBrixInputWine = $builder.get_object("increaseABVBrixInputWine")
  increaseABVBrixInputWine.signal_connect("activate") {
    wineSettings
  }
  increaseABVABVInputWine = $builder.get_object("increaseABVABVInputWine")
  increaseABVABVInputWine.signal_connect("activate") {
    wineSettings
  }
  increaseABVVolumeInputWine = $builder.get_object("increaseABVVolumeInputWine")
  increaseABVVolumeInputWine.signal_connect("activate") {
    wineSettings
  }

  def champagneSettings
    @increaseABVBrixInput = $builder.get_object("increaseABVBrixInputChampagne")
    @increaseABVABVInput = $builder.get_object("increaseABVABVInputChampagne")
    @increaseABVVolumeInput = $builder.get_object("increaseABVVolumeInputChampagne")
    @switch = $builder.get_object("imperialOrMetricSwitchSugarChampagne")
    @increaseABVNewBrix = $builder.get_object("increaseABVNewBrixChampagne")
    @increaseABVEstimatedABV = $builder.get_object("increaseABVEstimatedABVChampagne")
    @sugarAddOutput = $builder.get_object("sugarAddOutputChampagne")
    @honeyAddOutput = $builder.get_object("honeyAddOutputChampagne")
    onSwitchActivatedSugar
  end
  buttonChampagne = $builder.get_object("increaseABVButtonChampagne")
  buttonChampagne.signal_connect("clicked") {
    champagneSettings
  }
  increaseABVBrixInputChampagne = $builder.get_object("increaseABVBrixInputChampagne")
  increaseABVBrixInputChampagne.signal_connect("activate") {
    champagneSettings
  }
  increaseABVABVInputChampagne = $builder.get_object("increaseABVABVInputChampagne")
  increaseABVABVInputChampagne.signal_connect("activate") {
    champagneSettings
  }
  increaseABVVolumeInputChampagne = $builder.get_object("increaseABVVolumeInputChampagne")
  increaseABVVolumeInputChampagne.signal_connect("activate") {
    champagneSettings
  }

  def spiritsSettings
    @increaseABVBrixInput = $builder.get_object("increaseABVBrixInputSpirits")
    @increaseABVABVInput = $builder.get_object("increaseABVABVInputSpirits")
    @increaseABVVolumeInput = $builder.get_object("increaseABVVolumeInputSpirits")
    @switch = $builder.get_object("imperialOrMetricSwitchSugarSpirits")
    @increaseABVNewBrix = $builder.get_object("increaseABVNewBrixSpirits")
    @increaseABVEstimatedABV = $builder.get_object("increaseABVEstimatedABVSpirits")
    @sugarAddOutput = $builder.get_object("sugarAddOutputSpirits")
    @honeyAddOutput = $builder.get_object("honeyAddOutputSpirits")
    onSwitchActivatedSugar
  end
  buttonSpirits = $builder.get_object("increaseABVButtonSpirits")
  buttonSpirits.signal_connect("clicked") {
    spiritsSettings
  }
  increaseABVBrixInputSpirits = $builder.get_object("increaseABVBrixInputSpirits")
  increaseABVBrixInputSpirits.signal_connect("activate") {
    spiritsSettings
  }
  increaseABVABVInputSpirits = $builder.get_object("increaseABVABVInputSpirits")
  increaseABVABVInputSpirits.signal_connect("activate") {
    spiritsSettings
  }
  increaseABVVolumeInputSpirits = $builder.get_object("increaseABVVolumeInputSpirits")
  increaseABVVolumeInputSpirits.signal_connect("activate") {
    spiritsSettings
  }
end

def onSwitchActivatedSugar
  # to determine if imperial or metric so to use the correct measurements and calculations
  if @switch.active? == true
    differenceBrixMaths

    sugarToAddMetric = ((@desiredWortVolume / 3.78541) * 1.5 * @newEstimatedABV) * 0.0283495
    honeyToAddMetric = sugarToAddMetric * 1.250001102
    newSB = @newStartingBrix.round(2).to_s + '°Bx'
    newABV = @newEstimatedABV.round(2).to_s + '%'
    sugar = sugarToAddMetric.round(2).to_s + ' kilos'
    honey = honeyToAddMetric.round(2).to_s + ' kilos'

  elsif @switch.active? == false
    differenceBrixMaths

    sugarToAddImperial = (@desiredWortVolume * 1.5 * @newEstimatedABV) / 16
    honeyToAddImperial = sugarToAddImperial * 1.25
    newSB = @newStartingBrix.round(2).to_s + '°Bx'
    newABV = @newEstimatedABV.round(2).to_s + '%'
    sugar = sugarToAddImperial.to_i.to_s + ' lbs ' + (sugarToAddImperial % 1 * 16).to_i.to_s + ' oz'
    honey = honeyToAddImperial.to_i.to_s + ' lbs ' + (honeyToAddImperial % 1 * 16).to_i.to_s + ' oz'
  end

  # set the output variables to be the text of each widget
  # write the result of the above maths to the text of each widget

  @increaseABVNewBrix.set_text(newSB)
  @increaseABVEstimatedABV.set_text(newABV)
  @sugarAddOutput.set_text(sugar)
  @honeyAddOutput.set_text(honey)
end

def differenceBrixMaths
  # to calculate the difference in Brix, which is the same for both imperial and metric
  startingBrix = @increaseABVBrixInput.text.to_f
  desiredABV = @increaseABVABVInput.text.to_f
  @desiredWortVolume = @increaseABVVolumeInput.text.to_f
  finalGravity = 1.015
  originalExtract = (-((513.11767463 * desiredABV + 59931.43605250) - (46882.32536333 * (Math.sqrt(0.00022734 * desiredABV**2 + 0.02819081 * desiredABV + 1.63414684)))) / desiredABV)
  originalGravity = 1.00001 + 0.0038661 * originalExtract + 1.3488 * 10**(-5) * originalExtract**2 + 4.3074 * 10**(-8) * originalExtract**3
  @newStartingBrix = (258.6 * originalGravity - 258.6) / (0.87955073 * originalGravity + 0.12044926)
  apparentExtract = (-668.962) + (1262.45 * finalGravity ) - (776.43 * finalGravity**2) + (182.94 * finalGravity**3)
  attenuationCoefficient = (0.22) + (0.001 * originalExtract)
  realExtract = ((attenuationCoefficient * originalExtract) + apparentExtract) / (1 + attenuationCoefficient)
  estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract))
  @newEstimatedABV = estimatedABW * (finalGravity / 0.794)
  @newEstimatedABV = @newStartingBrix - startingBrix
end