def realABV
  def beerSettings
    @realABVStartingBrixInput = $builder.get_object("realABVStartingBrixInputBeer")
    @realABVFinalBrixInput = $builder.get_object("realABVFinalBrixInputBeer")
    @realABVFinalAttenuation = $builder.get_object("realABVFinalAttenuationBeer")
    @realABVFinalABV = $builder.get_object("realABVFinalABVBeer")
    realABVMaths
  end

  buttonBeer = $builder.get_object("realABVButtonBeer")
  buttonBeer.signal_connect("clicked") {
    beerSettings
  }
  realABVStartingBrixInputBeer = $builder.get_object("realABVStartingBrixInputBeer")
  realABVStartingBrixInputBeer.signal_connect("activate") {
    beerSettings
  }
  realABVFinalBrixInputBeer = $builder.get_object("realABVFinalBrixInputBeer")
  realABVFinalBrixInputBeer.signal_connect("activate") {
    beerSettings
  }

  def wineSettings
    @realABVStartingBrixInput = $builder.get_object("realABVStartingBrixInputWine")
    @realABVFinalBrixInput = $builder.get_object("realABVFinalBrixInputWine")
    @realABVFinalAttenuation = $builder.get_object("realABVFinalAttenuationWine")
    @realABVFinalABV = $builder.get_object("realABVFinalABVWine")
    realABVMaths
  end

  buttonWine = $builder.get_object("realABVButtonWine")
  buttonWine.signal_connect("clicked") {
    wineSettings
  }
  realABVStartingBrixInputWine = $builder.get_object("realABVStartingBrixInputWine")
  realABVStartingBrixInputWine.signal_connect("activate") {
    wineSettings
  }
  realABVFinalBrixInputWine = $builder.get_object("realABVFinalBrixInputWine")
  realABVFinalBrixInputWine.signal_connect("activate") {
    wineSettings
  }

  def champagneSettings
    @realABVStartingBrixInput = $builder.get_object("realABVStartingBrixInputChampagne")
    @realABVFinalBrixInput = $builder.get_object("realABVFinalBrixInputChampagne")
    @realABVFinalAttenuation = $builder.get_object("realABVFinalAttenuationChampagne")
    @realABVFinalABV = $builder.get_object("realABVFinalABVChampagne")
    realABVMaths
  end

  buttonChampagne = $builder.get_object("realABVButtonChampagne")
  buttonChampagne.signal_connect("clicked") {
    champagneSettings
  }
  realABVStartingBrixInputChampagne = $builder.get_object("realABVStartingBrixInputChampagne")
  realABVStartingBrixInputChampagne.signal_connect("activate") {
    champagneSettings
  }
  realABVFinalBrixInputChampagne = $builder.get_object("realABVFinalBrixInputChampagne")
  realABVFinalBrixInputChampagne.signal_connect("activate") {
    champagneSettings
  }

  def spiritsSettings
    @realABVStartingBrixInput = $builder.get_object("realABVStartingBrixInputSpirits")
    @realABVFinalBrixInput = $builder.get_object("realABVFinalBrixInputSpirits")
    @realABVFinalAttenuation = $builder.get_object("realABVFinalAttenuationSpirits")
    @realABVFinalABV = $builder.get_object("realABVFinalABVSpirits")
    realABVMaths
  end

  buttonSpirits = $builder.get_object("realABVButtonSpirits")
  buttonSpirits.signal_connect("clicked") {
    spiritsSettings
  }
  realABVStartingBrixInputSpirits = $builder.get_object("realABVStartingBrixInputSpirits")
  realABVStartingBrixInputSpirits.signal_connect("activate") {
    spiritsSettings
  }
  realABVFinalBrixInputSpirits = $builder.get_object("realABVFinalBrixInputSpirits")
  realABVFinalBrixInputSpirits.signal_connect("activate") {
    spiritsSettings
  }
end


def realABVMaths
  startingBrix = @realABVStartingBrixInput.text.to_f
  finalBrix = @realABVFinalBrixInput.text.to_f
  originalGravity = (startingBrix / (258.6 - ((startingBrix / 258.2) * 227.1))) + 1
  finalGravity = (finalBrix / (258.6 - ((finalBrix / 258.2) * 227.1))) + 1
  originalExtract = (-668.962) + (1262.45 * originalGravity) - (776.43 * originalGravity**2) + (182.94 * originalGravity**3)
  apparentExtract = (-668.962) + (1262.45 * finalGravity) - (776.43 * finalGravity**2) + (182.94 * finalGravity**3)
  attenuationCoefficient = (0.22) + (0.001 * originalExtract)
  realExtract = (attenuationCoefficient * originalExtract + apparentExtract) / (1 + attenuationCoefficient)
  realAttenuation = ((originalExtract - realExtract) / originalExtract) * 100
  attenuation = realAttenuation.round(2).to_s + '%'
  @realABVFinalAttenuation.set_text(attenuation)
  estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract))
  finalABV = estimatedABW * (finalGravity / 0.794)
  abv = finalABV.round(2).to_s + '%'
  @realABVFinalABV.set_text(abv)
end