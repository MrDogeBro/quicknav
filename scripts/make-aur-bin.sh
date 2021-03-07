#!/bin/bash

if [ -x "$(command -v cargo-aur)" ]; then
  command cargo aur

  if [ $? -eq 0 ]; then
    command cd ../aur-quicknav-bin

    if [ -f PKGBUILD ]; then
      command rm PKGBUILD
    fi

    if [ -f .SRCINFO ]; then
      command rm .SRCINFO
    fi

    command mv ../main-quicknav/PKGBUILD PKGBUILD
    command makepkg --printsrcinfo > .SRCINFO
  fi
fi
