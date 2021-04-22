#!/bin/bash

RED="\033[0;31m"
GREEN="\033[0;32m"
BLUE="\033[0;34m"
DIM="\033[2m"
DEF="\033[0m"
BOLD="\033[1m"

if [ -f "/etc/debian_version" ]; then
  if [ "$(whoami)" != "root" ]; then
    echo -e "${RED}This script must be run as root. Please run the script again with sudo.${DEF}"
    exit
  fi

  echo -e -n "${RED}${BOLD}"
  echo '_______       _____      ______'
  echo '__  __ \___  ____(_)________  /______________ ___   __'
  echo '_  / / /  / / /_  /_  ___/_  //_/_  __ \  __ `/_ | / /'
  echo '/ /_/ // /_/ /_  / / /__ _  ,<  _  / / / /_/ /__ |/ /'
  echo '\___\_\\__,_/ /_/  \___/ /_/|_| /_/ /_/\__,_/ _____/'

  echo -e "\n                   Deb Installer"
  echo -e "===================================================${DEF}\n"

  tag=$(curl -s https://api.github.com/repos/MrDogeBro/quicknav/releases/latest |
    awk '/tag_name/ { tag = $2 } /tag_name/ {print tag}' |
    cut -d '"' -f2)
  platformList=($(curl -s https://api.github.com/repos/MrDogeBro/quicknav/releases/latest |
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
      echo -e "➤ ${RED}Stopping install...${DEF}"
      exit
    fi
  fi

  echo -e "\n➤ ┌ Installing ${platformList[$platformChoice]}"

  echo -e "➤ │ ${BLUE}${BOLD}Info${DEF}: Downloading deb file..."
  echo -e "➤ │ ${DIM}$ curl -Ls -o /tmp/quicknav-installer.deb \"https://github.com/MrDogeBro/quicknav/releases/download/${tag}/${platformList[$platformChoice]}\"${DEF}"
  curl -Ls -o /tmp/quicknav-installer.deb "https://github.com/MrDogeBro/quicknav/releases/download/${tag}/${platformList[$platformChoice]}"
  echo -e "➤ │ ${BLUE}${BOLD}Info${DEF}: Deb file downloaded"

  echo -e "➤ │"

  echo -e "➤ │ ${BLUE}${BOLD}Info${DEF}: Installing deb file..."
  echo -e "➤ │ ${DIM}$ apt install -y /tmp/quicknav-installer.deb${DEF}"
  sudo apt install -y /tmp/quicknav-installer.deb &> /dev/null
  echo -e "➤ │ ${BLUE}${BOLD}Info${DEF}: Deb file installed"

  echo -e "➤ │"

  echo -e "➤ │ ${BLUE}${BOLD}Info${DEF}: Cleaning up install..."
  echo -e "➤ │ ${DIM}$ rm -f /tmp/quicknav-installer.deb${DEF}"
  rm -f /tmp/quicknav-installer.deb
  echo -e "➤ │ ${BLUE}${BOLD}Info${DEF}: Cleaned install"

  echo -e "➤ └ Installation complete"

  echo -e "\nThank you for installing quicknav! To get started head follow the instructions on the github page to add quicknav to your shell."
  echo -e "\n${GREEN}Adding Quicknav to Your Shell${DEF}: https://github.com/MrDogeBro/quicknav#adding-quicknav-to-your-shell"
else
  echo -e "${RED}It appears you are not on a debian based system. This script only works on debian based systems. Please check out the section on installing quicknav to get instructions for your OS. Thanks! \n\n${GREEN}Installing Quicknav${DEF}: https://github.com/MrDogeBro/quicknav#installing-quicknav"
fi
