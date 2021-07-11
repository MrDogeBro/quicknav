#!/bin/bash

pkgver=$1
currdir="$(pwd)"
dirpath=""

if [[ "$currdir" == *"main-quicknav" ]]; then
  if [ -d "./scripts/debbuilder" ]; then
    dirpath="./scripts/debbuilder"
  fi
elif [[ "$currdir" == *"scripts" ]]; then
  if [ -d "./debbuilder" ]; then
    dirpath="./debbuilder"
  fi
else
  echo "Script can not be run from current directory"
  exit 1
fi

if [[ "$dirpath" == "."* ]]; then
  if [ ! -x "$(command -v gh)" ]; then
    echo "gh (github cli) needs to be installed for the script to work"
    exit 1
  fi

  cd $dirpath

  docker build -t rustscan-builder . || exit

  # This creates a volume which binds your currentdirectory/debs to
  # the location where the deb files get spat out in the container.
  # You don't need to worry about it. Just chmod +x run.sh && ./run.sh and
  # you'll get yer .deb file in a few minutes. It runs faster after you've used it the first time.
  docker run -v "$(pwd)/debs:/debs" rustscan-builder

  gh release -R MrDogeBro/quicknav upload v${pkgver} debs/quicknav_${pkgver}_amd64.deb debs/quicknav_${pkgver}_armhf.deb debs/quicknav_${pkgver}_i386.deb
fi
