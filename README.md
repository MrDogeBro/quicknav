# Quicknav

[![Documentation Status](https://readthedocs.org/projects/quicknav/badge/)](https://quicknav.readthedocs.io/) [![License](https://img.shields.io/github/license/MrDogeBro/quicknav.svg)](https://github.com/MrDogeBro/quicknav/blob/master/LICENSE)

A way to quickly navigate your filesystem from the command line.

## Table of Contents

- [What is Quicknav](#what-is-quicknav)
- [Why Quicknav](#why-quicknav)
- [Examples](#examples)
- [Getting Started](#getting-started)
  - [Installing Quicknav](#installing-quicknav)
  - [Adding Quicknav to Your Shell](#adding-quicknav-to-your-shell)
- [Docs](#docs)
- [License](#license)

## What is Quicknav

Quicknav is a command line tool that allows you to easily jump to specific locations in your filesystem
from a simple command. Quicknav is built for terminal navigation on Unix operating systems.

## Why Quicknav

Quicknav allows you to easily set shortcuts to locations in your filesystem. It is made to speed up
your terminal navigation to commonly accessed places without cluttering your terminal with aliases.

## Examples

```sh
# easily navigate to shortcuts that were set in the config
nav rs     # go to rust projects folder
nav py     # go to python projects folder
nav js     # go to javascript projects folder
```

## Getting Started

### Installing Quicknav

You can install quicknav via one of the supported package managers or by downloading the binary and
adding it to your path. Prebuilt binaries can be downloaded from the [GitHub releases page](https://github.com/MrDogeBro/quicknav/releases).

In the future, support for more package managers will continue to grow.

| Distribution             | Package Manager | Command                                                                                                              |
| ------------------------ | --------------- | -------------------------------------------------------------------------------------------------------------------- |
| Arch                     | Paru            | `paru -S quicknav`                                                                                                   |
| Arch                     | Yay             | `yay -S quicknav`                                                                                                    |
| MacOS                    | Homebrew        | `brew tap MrDogeBro/quicknav && brew install quicknav`                                                               |
| Debian (or Debian Based) | Install Script  | `sudo /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/MrDogeBro/quicknav/HEAD/scripts/deb-install.sh)"` |
| Any (Rust Installed)     | Cargo           | `cargo install quicknav`                                                                                             |

### Adding Quicknav to Your Shell

Adding quicknav to your shell is incredibly easy. You just need to add the following line to your shells
configuration file and replace `shell_name` with the name of your shell. The shells listed below are the only
shells that are currently supported. Other shells may work but are not guaranteed to. If you would like another
shell to be supported, please head over to [feedback in the discussions tab](https://github.com/MrDogeBro/quicknav/discussions/categories/feedback).

##### Bash

Add the following to your `~/.bashrc`

```bash
eval "$(quicknav init bash)"
```

##### Zsh

Add the following to your `~/.zshrc`

```zsh
eval "$(quicknav init zsh)"
```

##### Fish

Add the following to your `~/.config/fish/config.fish`

```fish
quicknav init fish | source
```

## Docs

For more info on quicknav such as configuration, [head over to our docs](https://quicknav.readthedocs.io/) where you can find all of the
information you might need.

## License

Quicknav is licensed under an [MIT license](https://github.com/MrDogeBro/quicknav/blob/master/LICENSE).
