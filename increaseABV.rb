include Math

puts '(I)mperial Or (M)etric?'
imperialOrMetric = gets.upcase.chomp

while imperialOrMetric != 'I' || imperialOrMetric != 'M'

  if imperialOrMetric == 'I' || imperialOrMetric == 'M'
    puts 'Enter Your Starting Brix'
    startingBrix = gets.chomp.to_f

    puts 'Enter Your Desired ABV'
    desiredABV = gets.chomp.to_f

    puts 'Enter Your Desired Wort Volume'
    desiredWortVolume = gets.chomp.to_f

    finalGravity = 1.015

    originalExtract = (-((513.11767463 * desiredABV + 59931.43605250) - (46882.32536333 * (Math.sqrt(0.00022734 * desiredABV**2 + 0.02819081 * desiredABV + 1.63414684)))) / desiredABV)

    originalGravity = 1.00001 + 0.0038661 * originalExtract + 1.3488 * 10**(-5) * originalExtract**2 + 4.3074 * 10**(-8) * originalExtract**3

    newStartingBrix = (258.6 * originalGravity - 258.6) / (0.87955073 * originalGravity + 0.12044926)

    apparentExtract = (-668.962) + (1262.45 * finalGravity ) - (776.43 * finalGravity**2) + (182.94 * finalGravity**3)

    attenuationCoefficient = (0.22) + (0.001 * originalExtract)

    realExtract = ((attenuationCoefficient * originalExtract) + apparentExtract) / (1 + attenuationCoefficient)

    estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract))

    newEstimatedABV = estimatedABW * (finalGravity / 0.794)


    differenceBrix = newStartingBrix - startingBrix

    if imperialOrMetric == 'I'
      sugarToAddImperial = (desiredWortVolume * 1.5 * differenceBrix) / 16
      honeyToAddImperial = sugarToAddImperial * 1.25
      puts 'Your New Starting Brix Should Be: ' + newStartingBrix.round(2).to_s
      puts 'Your New Estimated ABV Will Be: ' + newEstimatedABV.round(2).to_s + '%'
      puts 'The Amount Of Sugar To Add Is: ' + sugarToAddImperial.to_i.to_s + 'lbs ' + (sugarToAddImperial % 1 * 16).to_i.to_s + 'oz'
      puts 'The Amount Of Honey To Add Is: ' + honeyToAddImperial.to_i.to_s + 'lbs ' + (honeyToAddImperial % 1 * 16).to_i.to_s + 'oz'
      break
    elsif imperialOrMetric == 'M'
      sugarToAddMetric = ((desiredWortVolume / 3.78541) * 1.5 * differenceBrix) * 0.0283495
      honeyToAddMetric = sugarToAddMetric * 1.250001102
      puts 'Your New Starting Brix Is: ' + newStartingBrix.round(2).to_s
      puts 'Your New Estimated ABV Is: ' + newEstimatedABV.round(2).to_s + '%'
      puts 'The Amount Of Sugar To Add Is: ' + sugarToAddMetric.round(2).to_s + 'kilos'
      puts 'The Amount Of Honey To Add Is: ' + honeyToAddMetric.round(2).to_s + 'kilos'
      break
    end

  else
    puts 'Enter either I or M'
    imperialOrMetric = gets.upcase.chomp
  end
end