def guestiMaths
  $guestimatorInput = $builder.get_object("guestimatorInput")
  # assigning the widget "guestimatorInput" to the variable '$guestimatorInput'
  button = $builder.get_object("guestimatorButton")
  # assigning the widget "guestimatorButton" to the variable 'button'
  button.signal_connect("clicked") {
    calculations
  }
  $guestimatorInput.signal_connect("activate") {
    calculations
  }
end

def calculations
# when the button is clicked or enter/return is pressed, do the following:
  startingBrix = $guestimatorInput.text.to_f
  originalGravity = (startingBrix / (258.6 - ((startingBrix / 258.2) * 227.1))) + 1
  finalGravity = 1.015
  originalExtract = (-668.962) + (1262.45 * originalGravity ) - (776.43 * originalGravity**2) + (182.94 * originalGravity**3)
  apparentExtract = (-668.962) + (1262.45 * finalGravity ) - (776.43 * finalGravity**2) + (182.94 * finalGravity**3)
  attenuationCoefficient = (0.22) + (0.001 * originalExtract)
  realExtract = ((attenuationCoefficient * originalExtract) + apparentExtract) / (1 + attenuationCoefficient)
  # realAttenuation = ((originalExtract - realExtract) / originalExtract) * 100
  # puts 'Your Real Attenuation is: ' + realAttenuation.round(2).to_s + '%'
  estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract))
  estimatedABV = estimatedABW * (finalGravity / 0.794)
  abv = estimatedABV.round(2).to_s + '%'
  output = $builder.get_object("guestimatorOutput")
  # set the variable 'output' to be the text of the widget "guestimatorOutput"
  output.set_text(abv)
  # write the result of the above maths (variable 'abv') to the text of the widget "guestimatorOutput"
end