def waterSparge
  def beerWaterSettings
    @preFermentVolumeInput = $builder.get_object("preFermentVolumeInputBeer")
    @totalGrainInput = $builder.get_object("totalGrainInputBeer")
    @boilTimeInput = $builder.get_object("boilTimeInputBeer")
    @switch = $builder.get_object("imperialOrMetricSwitchWaterBeer")
    @mashWaterOutput = $builder.get_object("mashWaterOutputBeer")
    @spargeWaterOutput = $builder.get_object("spargeWaterOutputBeer")
    @totalWaterOutput = $builder.get_object("totalWaterOutputBeer")
    onSwitchActivatedWater
  end

  buttonWaterBeer = $builder.get_object("totalWaterButtonBeer")
  buttonWaterBeer.signal_connect("clicked") {
    beerWaterSettings
  }
  preFermentVolumeInputBeer = $builder.get_object("preFermentVolumeInputBeer")
  preFermentVolumeInputBeer.signal_connect("activate") {
    beerWaterSettings
  }
  totalGrainInputBeer = $builder.get_object("totalGrainInputBeer")
  totalGrainInputBeer.signal_connect("activate") {
    beerWaterSettings
  }
  boilTimeInputBeer = $builder.get_object("boilTimeInputBeer")
  boilTimeInputBeer.signal_connect("activate") {
    beerWaterSettings
  }
end

def onSwitchActivatedWater
  # to determine if imperial or metric so to use the correct measurements and calculations
  if @switch.active? == true
    @grainAbsorption = 1.25181176
    # constant value of 1.25181176 litres/kilo
    # 0.15 gal = 0.5678118 L
    # 1 lb  = 0.453592 kg
    @mashThickness = 2.781108353
    # 1.333 quarts = 1.2614885 L
    # 1 lb  = 0.453592 kg
    # 2.781108353 litres/kilo
    waterSpargeMaths

    mash = @mashWater.round(2).to_s + ' litres'
    sparge = @spargeWater.round(2).to_s + ' litres'
    total = @totalWater.round(2).to_s + ' litres'

  elsif @switch.active? == false
    @grainAbsorption = 0.15
    # constant value of 0.15 gallons/lb
    @mashThickness = 1.333
    # 1.333 quarts/lb

    waterSpargeMaths

    mash = @mashWater.round(2).to_s + ' gallons'
    sparge = @spargeWater.round(2).to_s + ' gallons'
    total = @totalWater.round(2).to_s + ' gallons'
  end

  # set the output variables to be the text of each widget
  # write the result of the above maths to the text of each widget

  @mashWaterOutput.set_text(mash)
  @spargeWaterOutput.set_text(sparge)
  @totalWaterOutput.set_text(total)
end

def waterSpargeMaths
  wortShrinkage = 0.04
  # constant value of 4%
  percentBoiloff = 0.1
  # constant value of 10%

  preFermentVolume = @preFermentVolumeInput.text.to_f
  totalGrain = @totalGrainInput.text.to_f
  boilTime = @boilTimeInput.text.to_f / 60

  trubLoss = preFermentVolume * 0.05
  # 5% is an acceptable norm

  equipmentLoss = preFermentVolume * 0.08
  # 8% is an acceptable norm

  @totalWater = (((preFermentVolume + trubLoss)/(1 - wortShrinkage))/(1 - (boilTime * percentBoiloff))) + equipmentLoss + (totalGrain * @grainAbsorption)
  @mashWater = (totalGrain * @mashThickness)/4
  @spargeWater = @totalWater - @mashWater
end