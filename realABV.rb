def realABV
  button = $builder.get_object("realABVButton")
  # assigning the widget "realABVButton" to the variable 'button'
  $realABVStartingBrixInput = $builder.get_object("realABVStartingBrixInput")
  $realABVFinalBrixInput = $builder.get_object("realABVFinalBrixInput")

  button.signal_connect("clicked") {
    realABVCalculations
  }
  $realABVStartingBrixInput.signal_connect("activate") {
    realABVCalculations
  }
  $realABVFinalBrixInput.signal_connect("activate") {
    realABVCalculations
  }
end


def realABVCalculations
  startingBrix = $realABVStartingBrixInput.text.to_f
  finalBrix = $realABVFinalBrixInput.text.to_f
  originalGravity = (startingBrix / (258.6 - ((startingBrix / 258.2) * 227.1))) + 1
  finalGravity = (finalBrix / (258.6 - ((finalBrix / 258.2) * 227.1))) + 1
  originalExtract = (-668.962) + (1262.45 * originalGravity) - (776.43 * originalGravity**2) + (182.94 * originalGravity**3)
  apparentExtract = (-668.962) + (1262.45 * finalGravity) - (776.43 * finalGravity**2) + (182.94 * finalGravity**3)
  attenuationCoefficient = (0.22) + (0.001 * originalExtract)
  realExtract = (attenuationCoefficient * originalExtract + apparentExtract) / (1 + attenuationCoefficient)
  realAttenuation = ((originalExtract - realExtract) / originalExtract) * 100
  attenuation = realAttenuation.round(2).to_s + '%'
  realABVFinalAttenuation = $builder.get_object("realABVFinalAttenuation")
  realABVFinalAttenuation.set_text(attenuation)
  estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract))
  finalABV = estimatedABW * (finalGravity / 0.794)
  abv = finalABV.round(2).to_s + '%'
  realABVFinalABV = $builder.get_object("realABVFinalABV")
  realABVFinalABV.set_text(abv)
end