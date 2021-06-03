#!/bin/bash

if [ -z "$1" ]; then
  echo "Package version not passed in"
  exit 1
fi

pkgver=$1
currdir="$(pwd)"
dirpath=""

if [[ "$currdir" == *"main-quicknav" ]]; then
  if [ -d "../aur-quicknav" ]; then
    dirpath="../aur-quicknav"
  fi
elif [[ "$currdir" == *"scripts" ]]; then
  if [ -d "../../aur-quicknav" ]; then
    dirpath="../../aur-quicknav"
  fi
else
  echo "Script can not be run from current directory"
  exit 1
fi

if [[ "$dirpath" == "."* ]]; then
  if [ ! -d "$dirpath" ]; then
    echo "AUR repos not cloned"
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

  currrel=$(awk -F = '/pkgrel/ {print $2; exit}' PKGBUILD)
  currver=$(awk -F = '/pkgver/ {print $2; exit}' PKGBUILD)
  pkgzip="v$pkgver.tar.gz"

  if [[ "$currver" == "$pkgver" ]]; then
    pkgrel="$((currrel + 1))"
  else
    pkgrel="1"
  fi

  wget "https://github.com/MrDogeBro/quicknav/archive/v$pkgver.tar.gz"

  sha256=$(shasum -a 256 $pkgzip | sed "s/$pkgzip//" | sed "s/ //g")
  rm -f $pkgzip

  sed -i "" "s/^pkgver=.*/pkgver=${pkgver}/" PKGBUILD
  sed -i "" "s/^pkgrel=.*/pkgrel=${pkgrel}/" PKGBUILD
  sed -i "" "s/^sha256sums=.*/sha256sums=(\"$sha256\")/" PKGBUILD

  sed -i "" "s/pkgver = .*/pkgver = ${pkgver}/" .SRCINFO
  sed -i "" "s/pkgrel = .*/pkgrel = ${pkgrel}/" .SRCINFO
  sed -i "" "s/source = .*/source = quicknav-v${pkgver}.tar.gz::https:\/\/github.com\/MrDogeBro\/quicknav\/archive\/v${pkgver}.tar.gz/" .SRCINFO
  sed -i "" "s/sha256sums = .*/sha256sums = $sha256/" .SRCINFO

  git add PKGBUILD .SRCINFO
  git commit -m "Update for v${pkgver} release"
  git push
fi
