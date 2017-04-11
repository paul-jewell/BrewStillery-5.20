require 'gtk3'
require_relative 'guestimateABV'
# require_relative 'realABV'
require_relative 'increaseABV'
# require_relative 'increaseABV'

gladeFile = 'BrewStillery.glade'

# Construct a Gtk::Builder instance and load our UI description
$builder = Gtk::Builder.new
$builder.add_from_file(gladeFile)


# Connect signal handlers to the constructed widgets
mainWindow = $builder.get_object("mainWindow")
mainWindow.signal_connect("destroy") { Gtk.main_quit }

guestiMaths
# call method from guestimateABV.rb
increaseABV
# call method from increaseABV.rb


mainWindow = $builder.get_object('mainWindow')
mainWindow.set_title("BrewStillery")
mainWindow.show()


Gtk.main