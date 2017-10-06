#!/bin/sh

cargo build --release &&
sudo install -Dm755 ../target/release/BrewStillery /usr/bin/BrewStillery &&
sudo install -Dm755 ../Arch/BrewStillery.desktop /usr/share/applications/BrewStillery.desktop &&
sudo install -Dm755 ../media/BrewStilleryIcon.svg /usr/share/BrewStillery/BrewStilleryIcon.svg &&
sudo install -Dm755 ../media/BrewStilleryLogo.svg /usr/share/BrewStillery/BrewStilleryLogo.svg