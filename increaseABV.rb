def increaseABV
  button = $builder.get_object("increaseABVButton")
  # assigning the widget "guestimatorButton" to the variable 'button'
  $switch = $builder.get_object("imperialOrMetricSwitch")
  $increaseABVBrixInput = $builder.get_object("increaseABVBrixInput")
  $increaseABVABVInput = $builder.get_object("increaseABVABVInput")
  $increaseABVVolumeInput = $builder.get_object("increaseABVVolumeInput")

  button.signal_connect("clicked") {
    onSwitchActivated
  }
  $increaseABVBrixInput.signal_connect("activate") {
    onSwitchActivated
  }
  $increaseABVABVInput.signal_connect("activate") {
    onSwitchActivated
  }
  $increaseABVVolumeInput.signal_connect("activate") {
    onSwitchActivated
  }
end

def onSwitchActivated
  # to determine if imperial or metric so to use the correct measurements and calculations
  if $switch.active? == true
    differenceBrixCalculations

    sugarToAddMetric = (($desiredWortVolume / 3.78541) * 1.5 * $differenceBrix) * 0.0283495
    honeyToAddMetric = sugarToAddMetric * 1.250001102
    newSB = $newStartingBrix.round(2).to_s + '°Bx'
    newABV = $newEstimatedABV.round(2).to_s + '%'
    sugar = sugarToAddMetric.round(2).to_s + ' kilos'
    honey = honeyToAddMetric.round(2).to_s + ' kilos'

  elsif $switch.active? == false
    differenceBrixCalculations

    sugarToAddImperial = ($desiredWortVolume * 1.5 * $differenceBrix) / 16
    honeyToAddImperial = sugarToAddImperial * 1.25
    newSB = $newStartingBrix.round(2).to_s + '°Bx'
    newABV = $newEstimatedABV.round(2).to_s + '%'
    sugar = sugarToAddImperial.to_i.to_s + ' lbs ' + (sugarToAddImperial % 1 * 16).to_i.to_s + ' oz'
    honey = honeyToAddImperial.to_i.to_s + ' lbs ' + (honeyToAddImperial % 1 * 16).to_i.to_s + ' oz'
  end

  # set the output variables to be the text of each widget
  # write the result of the above maths to the text of each widget

  increaseABVNewBrix = $builder.get_object("increaseABVNewBrix")
  increaseABVNewBrix.set_text(newSB)

  increaseABVEstimatedABV = $builder.get_object("increaseABVEstimatedABV")
  increaseABVEstimatedABV.set_text(newABV)

  sugarAddOutput = $builder.get_object("sugarAddOutput")
  sugarAddOutput.set_text(sugar)

  honeyAddOutput = $builder.get_object("honeyAddOutput")
  honeyAddOutput.set_text(honey)
end

def differenceBrixCalculations
  # to calculate the difference in Brix, which is the same for both imperial and metric
  startingBrix = $increaseABVBrixInput.text.to_f
  desiredABV = $increaseABVABVInput.text.to_f
  $desiredWortVolume = $increaseABVVolumeInput.text.to_f
  finalGravity = 1.015
  originalExtract = (-((513.11767463 * desiredABV + 59931.43605250) - (46882.32536333 * (Math.sqrt(0.00022734 * desiredABV**2 + 0.02819081 * desiredABV + 1.63414684)))) / desiredABV)
  originalGravity = 1.00001 + 0.0038661 * originalExtract + 1.3488 * 10**(-5) * originalExtract**2 + 4.3074 * 10**(-8) * originalExtract**3
  $newStartingBrix = (258.6 * originalGravity - 258.6) / (0.87955073 * originalGravity + 0.12044926)
  apparentExtract = (-668.962) + (1262.45 * finalGravity ) - (776.43 * finalGravity**2) + (182.94 * finalGravity**3)
  attenuationCoefficient = (0.22) + (0.001 * originalExtract)
  realExtract = ((attenuationCoefficient * originalExtract) + apparentExtract) / (1 + attenuationCoefficient)
  estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract))
  $newEstimatedABV = estimatedABW * (finalGravity / 0.794)
  $differenceBrix = $newStartingBrix - startingBrix
end