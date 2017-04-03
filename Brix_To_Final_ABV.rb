puts 'Enter Starting Brix'
startingBrix = gets.chomp.to_f

puts 'Enter Final Brix'
finalBrix = gets.chomp.to_f

originalGravity = (startingBrix / (258.6 - ((startingBrix / 258.2) * 227.1))) + 1

finalGravity = (finalBrix / (258.6 - ((finalBrix / 258.2) * 227.1))) + 1

originalExtract = (-668.962) + (1262.45 * originalGravity) - (776.43 * originalGravity**2) + (182.94 * originalGravity**3)

apparentExtract = (-668.962) + (1262.45 * finalGravity) - (776.43 * finalGravity**2) + (182.94 * finalGravity**3)

attenuationCoefficient = (0.22) + (0.001 * originalExtract)

realExtract = (attenuationCoefficient * originalExtract + apparentExtract) / (1 + attenuationCoefficient)

realAttenuation = ((originalExtract - realExtract) / originalExtract) * 100
puts 'Your Real Attenuation is: ' + realAttenuation.round(2).to_s + '%'

estimatedABW = (originalExtract - realExtract) /  ( 2.0665 - (0.010665 * originalExtract))
puts estimatedABW
estimatedABW = (76.08 * (originalGravity - finalGravity)) / (1.775 - originalGravity)
puts estimatedABW

finalABV = estimatedABW * (finalGravity / 0.794)
puts 'Your Final ABV is: ' + finalABV.round(2).to_s + '%'