def waterSparge
  button = $builder.get_object("totalWaterButton")
  # assigning the widget "guestimatorButton" to the variable 'button'
  $switch = $builder.get_object("imperialOrMetricSwitchWater")
  $preFermentVolumeInput = $builder.get_object("preFermentVolumeInput")
  $totalGrainInput = $builder.get_object("totalGrainInput")
  $boilTimeInput = $builder.get_object("boilTimeInput")

  button.signal_connect("clicked") {
    onSwitchActivatedWater
  }
  $preFermentVolumeInput.signal_connect("activate") {
    onSwitchActivatedWater
  }
  $totalGrainInput.signal_connect("activate") {
    onSwitchActivatedWater
  }
  $boilTimeInput.signal_connect("activate") {
    onSwitchActivatedWater
  }
end


def onSwitchActivatedWater
  # to determine if imperial or metric so to use the correct measurements and calculations
  if $switch.active? == true
    waterSpargeMaths

    sugarToAddMetric = (($desiredWortVolume / 3.78541) * 1.5 * $differenceBrix) * 0.0283495
    honeyToAddMetric = sugarToAddMetric * 1.250001102
    newSB = $newStartingBrix.round(2).to_s + '°Bx'
    newABV = $newEstimatedABV.round(2).to_s + '%'
    sugar = sugarToAddMetric.round(2).to_s + ' kilos'
    honey = honeyToAddMetric.round(2).to_s + ' kilos'

  elsif $switch.active? == false
    waterSpargeMaths

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



def waterSpargeMaths
  mashThickness = 1.333
  wortShrinkage = 0.04
  # constant value of 4%
  percentBoiloff = 0.1
  # constant value of 10%
  grainAbsorptionImperial = 0.15
  # constant value of 0.15 gallons/lb
  # grainAbsorptionMetric =

  totalWater = (((preFermentVolume+trubLoss)/(1-(wortShrinkage)))/(1-(boilTime*(percentBoiloff)))) + equipmentLoss + (totalGrain * grainAbsorptionImperial-Or-Metric)
  mashWater = (totalGrain * mashThickness)/4
  spargeWater = totalWater - mashWater
end

# inputs
#   1) Pre-Fermentation Volume (gal or l)
#   2) Total Grain (lb or kg)
#   3) Boil Time (in minutes)
#
# specify that we set the other variables as constant (in glade)
#
# outputs
#   1) Mash Water (gal or l)
#   2) Sparge Water (gal or l)
#   3) Total Water (gal or l)
#
# preFermentVolume = user input
# totalGrain = user input
# boilTime = user input
# trubLoss = has to be a percentage of the boil size. the guy has it set to 0.5 gallons, but that won't be accurate for a 100 gallon batch
# equipmentLoss = how much is left in the bottom of the boil kettle (this also needs to be proportionate to the boil size)
# mashThickness = 1.33 lbs/quart
# grainTemperature = constant 70F/21.111C ("room temp")

# total water needed (gal) = (((vBatch+vTrub)/(1-(vShrinkage/100)))/(1-(vBoilTime*(vBoiloff/100)))) + vEquipment + (vGrain * vAbsorptionRate)
# mash volume (gal) = (vGrain * vThickness)/4
# sparge volume (gal) = vTotalWater - vMashVolume
# PreBoilVolume = vTotalWater - (vGrain * vAbsorptionRate) - vEquipment