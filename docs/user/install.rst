Getting Started
===============

Installing Quicknav
-------------------

You can install quicknav via one of the supported package managers or by downloading the binary and
adding it to your path. Prebuilt binaries can be downloaded from the GitHub releases page.

In the future, support for more package managers will continue to grow.

+--------------------------+-----------------+----------------------------------------------------------------------------------------------------------------------------+
| Distribution             | Package Manager | Command                                                                                                                    |
+==========================+=================+============================================================================================================================+
| Arch                     | Paru            | :code:`paru -S quicknav`                                                                                                   |
+--------------------------+-----------------+----------------------------------------------------------------------------------------------------------------------------+
| Arch                     | Yay             | :code:`yay -S quicknav`                                                                                                    |
+--------------------------+-----------------+----------------------------------------------------------------------------------------------------------------------------+
| MacOS                    | Homebrew        | :code:`brew tap MrDogeBro/quicknav && brew install quicknav`                                                               |
+--------------------------+-----------------+----------------------------------------------------------------------------------------------------------------------------+
| Debian (or Debian Based) | Install Script  | :code:`sudo /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/MrDogeBro/quicknav/HEAD/scripts/deb-install.sh)"` |
+--------------------------+-----------------+----------------------------------------------------------------------------------------------------------------------------+
| Any (Rust Installed)     | Cargo           | :code:`cargo install quicknav`                                                                                             |
+--------------------------+-----------------+----------------------------------------------------------------------------------------------------------------------------+

Adding Quicknav to Your Shell
-----------------------------

Adding quicknav to your shell is incredibly easy. You just need to add the following line to your
shells configuration file and replace shell_name with the name of your shell. The shells listed
below are the only shells that are currently supported. Other shells may work but are not guaranteed
to. If you would like another shell to be supported, please head over to
`feedback in the discussions tab <https://github.com/MrDogeBro/quicknav/discussions/categories/feedback>`_.

Bash
++++

Add the following to your :code:`~/.bashrc`

.. code:: bash

    eval "$(quicknav init bash)"

Zsh
+++

Add the following to your :code:`~/.zshrc`

.. code:: zsh

    eval "$(quicknav init zsh)"

Fish
++++

Add the following to your :code:`~/.config/fish/config.fish`

.. code:: fish

    quicknav init fish | source


Installing and Building from Source
-----------------------------------

To get the source code, you can clone the `GitHub repository <https://github.com/MrDogeBro/quicknav>`_ using git.
Once you have a copy of the source code, you can install from source or build a binary.

.. code:: bash

    $ git clone https://github.com/MrDogeBro/quicknav
    $ cd quicknav

    # install from source
    $ cargo install --path .

    # build a binary
    $ cargo build --release
