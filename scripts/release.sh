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

  if [ ! -x "$(command -v wget)" ]; then
    echo "wget needs to be installed for the script to work"
    exit 1
  fi

  if [ ! -x "$(command -v awk)" ]; then
    echo "awk needs to be installed for the script to work"
    exit 1
  fi

  if [ ! -x "$(command -v sed)" ]; then
    echo "sed needs to be installed for the script to work"
    exit 1
  fi

  cd $dirpath

  echo "Releasing GitHub...\n"

  ./scripts/make-github.sh $pkgver

  if [ $? -eq 0 ]; then
    echo "Released GitHub v${pkgver}\n"
  else
    echo "GitHub Release Failed — Stopping..."
    exit 1
  fi

  echo "Releasing Cargo...\n"

  cargo publish

  if [ $? -eq 0 ]; then
    echo "Released Cargo v${pkgver}\n"
  else
    echo "Cargo Release Failed — Stopping..."
    exit 1
  fi

  echo "Releasing AUR...\n"

  ./scripts/make-aur.sh $pkgver

  if [ $? -eq 0 ]; then
    echo "Released AUR v${pkgver}\n"
  else
    echo "AUR Release Failed — Stopping..."
    exit 1
  fi

  echo "Releasing Homebrew...\n"

  ./scripts/make-homebrew.sh $pkgver

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

  echo "Building Debs...\n"

  if [ ! -x "$(command -v docker)" ]; then
    echo "docker needs to be installed for the script to work"
    echo "Debs Build Failed — Stopping..."
    exit 1
  fi

  ./scripts/debbuilder/run.sh $pkgver

  if [ $? -eq 0 ]; then
    echo "Debs Built v${pkgver}\n"
  else
    echo "Debs Build Failed — Stopping..."
    exit 1
  fi
fi
