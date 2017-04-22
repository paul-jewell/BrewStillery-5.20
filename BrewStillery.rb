require 'gtk3'
require_relative 'guestimateABV'
require_relative 'increaseABV'
require_relative 'realABV'
require_relative 'waterSparge'

gladeFile = 'BrewStillery.glade'

# Construct a Gtk::Builder instance and load our UI description
$builder = Gtk::Builder.new
$builder.add_from_file(gladeFile)

# appID = Gtk::Application.new("uk.co.monkeylog.BrewStillery", :flags_none)
# appID.signal_connect "activate" do |application|
#   mainWindow = Gtk::ApplicationWindow.new(application)
# end
# strip the mainWindow line out of the bullshit

mainWindow = $builder.get_object("mainWindow")
mainWindow.signal_connect("destroy") { Gtk.main_quit }
# Connect signal handlers to the constructed widgets

# @guestimateABV.call 'guestimatorInputBeer', 'guestimatorButtonBeer', 'guestimatorOutputBeer'

guestimateABV
# call method from guestimateABV.rb
increaseABV
# call method from increaseABV.rb
realABV
# call method from realABV.rb
# waterSparge
# call method from waterSparge.rb


mainWindow = $builder.get_object('mainWindow')
mainWindow.set_title("BrewStillery")
mainWindow.show()


Gtk.main