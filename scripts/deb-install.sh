#!/bin/bash

if [ -f "/etc/debian_version" ]; then
  echo
else
  echo -e "\033[0;31mIt appears you are not on a debian based system. This script only works on debian based systems. Please check out the section on installing quicknav to get instructions for your OS. Thanks! \n\n\033[0;32mInstalling Quicknav\033[0m: https://github.com/MrDogeBro/quicknav#installing-quicknav"
fi
