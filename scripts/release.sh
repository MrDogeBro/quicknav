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

  if [ ! -x "$(command -v docker)" ]; then
    echo "docker needs to be installed for the script to work"
    exit 1
  fi

  cd $dirpath

  echo -e "Building Binary...\n"

  cargo build --release

  if [ $? -eq 0 ]; then
    echo -e "Binary Built v${pkgver}\n"
  else
    echo "Binary Build Failed — Stopping..."
    exit 1
  fi

  echo -e "Releasing GitHub...\n"

  ./scripts/make-github.sh $pkgver

  if [ $? -eq 0 ]; then
    echo -e "Released GitHub v${pkgver}\n"
  else
    echo "GitHub Release Failed — Stopping..."
    exit 1
  fi

  echo -e "Releasing Cargo...\n"

  cargo publish

  if [ $? -eq 0 ]; then
    echo -e "Released Cargo v${pkgver}\n"
  else
    echo "Cargo Release Failed — Stopping..."
    exit 1
  fi

  echo -e "Releasing AUR...\n"

  ./scripts/make-aur.sh $pkgver

  if [ $? -eq 0 ]; then
    echo -e "Released AUR v${pkgver}\n"
  else
    echo "AUR Release Failed — Stopping..."
    exit 1
  fi

  echo -e "Releasing Homebrew...\n"

  ./scripts/make-homebrew.sh $pkgver

  if [ $? -eq 0 ]; then
    echo -e "Released Homebrew v${pkgver}\n"
  else
    echo "Homebrew Release Failed — Stopping..."
    exit 1
  fi

  echo -e "Building Debs...\n"

  ./scripts/debbuilder/run.sh $pkgver

  if [ $? -eq 0 ]; then
    echo -e "Debs Built v${pkgver}\n"
  else
    echo "Debs Build Failed — Stopping..."
    exit 1
  fi
fi
