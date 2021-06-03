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
  cd $dirpath

  echo "Releasing Cargo...\n"

  cargo publish

  if [ $? -eq 0 ]; then
    echo "Released Cargo v${pkgver}\n"
  else
    echo "Cargo Release Failed — Stopping..."
    exit 1
  fi

  echo "Releasing AUR...\n"

  ./scripts/make-aur.sh

  if [ $? -eq 0 ]; then
    echo "Released AUR v${pkgver}\n"
  else
    echo "AUR Release Failed — Stopping..."
    exit 1
  fi

  echo "Releasing Homebrew...\n"

  ./scripts/make-homebrew.sh

  if [ $? -eq 0 ]; then
    echo "Released Homebrew v${pkgver}\n"
  else
    echo "Homebrew Release Failed — Stopping..."
    exit 1
  fi

  echo "Building Binary...\n"

  cargo build --release

  if [ $? -eq 0 ]; then
    echo "Binary Built v${pkgver}\n"
  else
    echo "Binary Build Failed — Stopping..."
    exit 1
  fi
fi
