#!/bin/bash

if [ -z "$1" ]; then
  echo "Package version not passed in"
  exit 1
fi

pkgver=$1
currdir="$(pwd)"
dirpath=""

if [[ "$currdir" == *"main-quicknav" ]]; then
  dirpath="."
elif [[ "$currdir" == *"scripts" ]]; then
  dirpath="../"
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

  gh release -R MrDogeBro/quicknav create v${pkgver} target/release/quicknav
fi
