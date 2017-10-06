#!/bin/sh

cargo build --release &&
sudo install -Dm755 ../target/release/BrewStillery /usr/bin/BrewStillery &&
install -Dm755 ../Arch/BrewStillery.desktop "$pkgdir/usr/share/applications/BrewStillery.desktop" &&
install -Dm755 ../media/BrewStilleryIcon.svg "$pkgdir/usr/share/BrewStillery/BrewStilleryIcon.svg" &&
install -Dm755 ../media/BrewStilleryLogo.svg "$pkgdir/usr/share/BrewStillery/BrewStilleryLogo.svg"