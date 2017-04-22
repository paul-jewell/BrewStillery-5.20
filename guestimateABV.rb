def guestimateABV
  def beerGuestimatorSettings
    @guestimatorInput = $builder.get_object("guestimatorInputBeer")
    @guestimatorOutput = "guestimatorOutputBeer"
    guestiMaths
  end

  buttonBeer = $builder.get_object("guestimatorButtonBeer")
  buttonBeer.signal_connect("clicked") {
    beerGuestimatorSettings
  }
  guestimatorInputBeer = $builder.get_object("guestimatorInputBeer")
  guestimatorInputBeer.signal_connect("activate") {
    beerGuestimatorSettings
  }

  def wineGuestimatorSettings
    @guestimatorInput = $builder.get_object("guestimatorInputWine")
    @guestimatorOutput = "guestimatorOutputWine"
    guestiMaths
  end

  buttonWine = $builder.get_object("guestimatorButtonWine")
  buttonWine.signal_connect("clicked") {
    wineGuestimatorSettings
  }
  guestimatorInputWine = $builder.get_object("guestimatorInputWine")
  guestimatorInputWine.signal_connect("activate") {
    wineGuestimatorSettings
  }

  def champagneGuestimatorSettings
    @guestimatorInput = $builder.get_object("guestimatorInputChampagne")
    @guestimatorOutput = "guestimatorOutputChampagne"
    guestiMaths
  end

  buttonChampagne = $builder.get_object("guestimatorButtonChampagne")
  buttonChampagne.signal_connect("clicked") {
    champagneGuestimatorSettings
  }
  guestimatorInputChampagne = $builder.get_object("guestimatorInputChampagne")
  guestimatorInputChampagne.signal_connect("activate") {
    champagneGuestimatorSettings
  }

  def spiritsGuestimatorSettings
    @guestimatorInput = $builder.get_object("guestimatorInputSpirits")
    @guestimatorOutput = "guestimatorOutputSpirits"
    guestiMaths
  end
  
  buttonSpirits = $builder.get_object("guestimatorButtonSpirits")
  buttonSpirits.signal_connect("clicked") {
    spiritsGuestimatorSettings
  }
  guestimatorInputSpirits = $builder.get_object("guestimatorInputSpirits")
  guestimatorInputSpirits.signal_connect("activate") {
    spiritsGuestimatorSettings
  }
end

def guestiMaths
# when the button is clicked or enter/return is pressed, do the following:
  startingBrix = @guestimatorInput.text.to_f
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
  output = $builder.get_object @guestimatorOutput
  # set the variable 'output' to be the text of the widget "guestimatorOutput"
  output.set_text(abv)
  # write the result of the above maths (variable 'abv') to the text of the widget "guestimatorOutput"
end