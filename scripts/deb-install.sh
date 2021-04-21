#!/bin/bash

RED="\033[0;31m"
GREEN="\033[0;32m"
BLUE="\033[0;34m"
DEF="\033[0m"
BOLD="\033[1m"

# if [ -f "/etc/debian_version" ]; then
  # tag=$(curl -s https://api.github.com/repos/MrDogeBro/quicknav/releases/latest |
    # awk '/tag_name/ { tag = $2 } /tag_name/ {print tag}' |
    # cut -d '"' -f2)
  tag="v0.1.0"
  # platformList=($(curl -s https://api.github.com/repos/MrDogeBro/quicknav/releases/latest |
    # awk '/name/ { name = $2 } /name/ { print name }' |
    # awk '/.deb/' |
    # cut -d '"' -f2))
  platformList=($(cat git.txt |
    awk '/name/ { name = $2 } /name/ { print name }' |
    awk '/.deb/' |
    cut -d '"' -f2))
  platformStr=""
  platformIndex=1
  platformChoice=0

  for platform in "${platformList[@]}"; do
      platformStr+="[${platformIndex}] $platform "

      if [ $platformIndex -ne ${#platformList[@]} ]; then
        ((platformIndex++))
      fi
  done

  echo $platformStr
  echo -e -n "➤ ${GREEN}${BOLD}Choose deb file platform (default=1)${DEF}: "
  read -n 1 -p "" debPlatform

  if [ "$debPlatform" != "" ]; then
    echo -e -n "\n"

    for (( i=0; i<${#platformList[@]}; i++ )); do
      if [ $(( ${debPlatform} )) -eq $(( ${i} + 1 )) ]; then
        platformChoice=$i
      fi
    done
  fi

  echo -e -n "➤ ${GREEN}${BOLD}Would you like to install ${platformList[$platformChoice]} (Y/n)${DEF}: "
  read -n 1 -p "" installDeb

  if [ "$installDeb" != "" ]; then
    echo -e -n "\n"

    if [ "$installDeb" != "y" ]; then
      exit
    fi
  fi

  echo -e "\n➤ ┌ Installing ${platformList[$platformChoice]}"

  echo -e "➤ │ ${BLUE}${BOLD}Info${DEF}: Downloading deb file..."
# else
  # echo -e "${RED}It appears you are not on a debian based system. This script only works on debian based systems. Please check out the section on installing quicknav to get instructions for your OS. Thanks! \n\n${GREEN}Installing Quicknav${DEF}: https://github.com/MrDogeBro/quicknav#installing-quicknav"
# fi
