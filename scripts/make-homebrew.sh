#!/bin/bash

if [ -z "$1" ]; then
  echo "Package version not passed in"
  exit 1
fi

pkgver=$1
currdir="$(pwd)"
dirpath=""

if [[ "$currdir" == *"main-quicknav" ]]; then
  if [ -d "../homebrew-quicknav" ]; then
    dirpath="../homebrew-quicknav"
  fi
elif [[ "$currdir" == *"scripts" ]]; then
  if [ -d "../../homebrew-quicknav" ]; then
    dirpath="../../homebrew-quicknav"
  fi
else
  echo "Script can not be run from current directory"
  exit 1
fi

if [[ "$dirpath" == "."* ]]; then
  if [ ! -d "$dirpath" ]; then
    echo "Homebrew tap repo not cloned"
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

  currver=$(awk '/version/ {gsub(/"/, "", $2); print $2}' Formula/quicknav.rb)
  pkgzip="v$pkgver.tar.gz"

  wget "https://github.com/MrDogeBro/quicknav/archive/v$pkgver.tar.gz"

  sha256=$(shasum -a 256 $pkgzip | sed "s/$pkgzip//" | sed "s/ //g")
  rm -f $pkgzip

  sed -i "" "s/version .*/version \"${pkgver}\"/" Formula/quicknav.rb
  sed -i "" "s/sha256 .*/sha256 \"${sha256}\"/" Formula/quicknav.rb
  sed -i "" "s/url .*/url \"https:\/\/github.com\/MrDogeBro\/quicknav\/archive\/v${pkgver}.tar.gz\"/" Formula/quicknav.rb

  git add Formula/quicknav.rb
  git commit -m "Update for v${pkgver} release"
  git push
fi
