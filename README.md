# Quicknav

A way to quickly navigate your filesystem from the command line.

## Table of Contents

- [What is Quicknav](#what-is-quicknav)
- [Why Quicknav](#why-quicknav)
- [Examples](#examples)
- [Getting Started](#getting-started)
  - [Installing Quicknav](#installing-quicknav)
  - [Adding Quicknav to Your Shell](#adding-quicknav-to-your-shell)
- [Configuring Quicknav](#configuring-quicknav)
  - [Adding and Removing Shortcuts](#adding-and-removing-shortcuts)
  - [Options](#options)
    - [Create Missing Directories](#create-missing-directories)
  - [Init Flags](#init-flags)
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

| Distribution             | Package Manager | Command                                                                                                                |
| ------------------------ | --------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Arch                     | Paru            | `paru -S quicknav`                                                                                                     |
| Arch                     | Yay             | `yay -S quicknav`                                                                                                      |
| MacOS                    | Homebrew        | `brew tap MrDogeBro/quicknav && brew install quicknav`                                                                 |
| Debian (or Debian Based) | Install Script  | `sudo /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/MrDogeBro/quicknav/HEAD/scripts/deb-installer.sh)"` |
| Any (Rust Installed)     | Cargo           | `cargo install quicknav`                                                                                               |

### Adding Quicknav to Your Shell

Adding quicknav to your shell is increadably easy. You just need to add the following line to your shells
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

## Configuring Quicknav

### Adding and Removing Shortcuts

Adding and removing shortcuts is quite simple. You can either directly edit the configuration file or use the
add and remove commands to do the same job. If you want the most configuration, then you will need to use the
config file. The config file can be found at `~/.config/quicknav/quicknav.json` or if you have set the xdg
config home, your config will be found at `$XDG_CONFIG_HOME/quicknav/quicknav.json`.

The built in configuration commands are listed below. To get more info about a command, use the help command
and specify which command you would like help for.

```sh
# display the help message for a command
$ quicknav help command_name

# list the registed shortcuts
$ quicknav list [call]

# add a new shortcut
$ quicknav add <call> <location>

# remove a shortcut
$ quicknav remove <call>
```

If you would like to edit the configuration file directly, new shortcuts must follow the following json structure.
You can add multiple calls (what you use to navigate to the location) but you are only required to include one.

```json
{
  "name": "shortcut name",
  "description": "shortcut description",
  "location": "the location to jump to (~ supported)",
  "calls": ["callname", "anothercall", "maybeevenanothercall"]
}
```

Once you have added a shortcut, you can use the nav command to navigate to that shortcut by one of its calls.
For example, if you want to go to the shortcut in the example above, we could use one of the following commands.

```sh
# uses the first call
$ nav callname

# uses the second call
$ nav anothercall

# uses the third call
$ nav maybeevenanothercall
```

You can also check out the [example configuration](https://github.com/MrDogeBro/quicknav/blob/master/example-configuration.json).

### Options

These are the options that quicknav accepts in its config file, not including shortcuts, under the options section.
These options currently can only be configured by editing the config file but this will change in the future.
Each option listed will give a description of what it does and then the the following information in a table.

| Config Name                                         | Allowed Value                                               | Default Value                                |
| --------------------------------------------------- | ----------------------------------------------------------- | -------------------------------------------- |
| This is the name for the option in the config file. | This is what type of value can be configured for the option | This is what the value is set to by default. |

##### Create Missing Directories

If set to true, quicknav will automatically create the directories in a given path if they do not exist
when the navigation shortcut to that location is used. This is useful if you are moving your config between
computers and would like your shortcut locations to automatically be created for you.

| Config Name                | Allowed Value               | Default Value |
| -------------------------- | --------------------------- | ------------- |
| create_missing_directories | Boolean (`true` or `false`) | `false`       |

### Init Flags

These are flags that you can add to the init command that is used to load your shell profile.
For more info on loading your shell profile, check out [Adding Quicknav to Your Shell]().

- `-c, --command`: Changes the command which is used to navigate to a shortcut. The default command is `nav`.

## License

Quicknav is licensed under an [MIT license](https://github.com/MrDogeBro/quicknav/blob/master/LICENSE).
