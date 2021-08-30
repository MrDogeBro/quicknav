Configuration
=============

Adding and Removing Shortcuts
-----------------------------

To add or remove shortcuts, you can either directly edit the configuration file or use the add and remove
commands to do the same job. You can configure basically everything you would need to from commands built in
but you can also edit the config file if you would like. The config file can be found at :code:`~/.config/quicknav/quicknav.json`
or if you have set the xdg config home, your config will be found at :code:`$XDG_CONFIG_HOME/quicknav/quicknav.json`.

The built in configuration commands are listed below in a quick overview. To get more info about a command,
use the help command and specify which command you would like help for.

.. code:: bash

  # display the help message for a command
  $ quicknav help [command]

  # list the registed shortcuts
  $ quicknav list [name]

  # add a new shortcut
  $ quicknav add <call> <location>

  # add a new shortcut call
  $ quicknav add-call <shortcut> <call>

  # remove a shortcut
  $ quicknav remove <shortcut>

  # remove a shortcut call
  $ quicknav remove <call>

  # edit a shortcut
  $ quicknav edit <shortcut> [location]

  # configure settings
  $ quicknav config [param] [value]

If you would like to edit the configuration file directly, new shortcuts must follow the following json structure.
You can add multiple calls (what you use to navigate to the location) but you are only required to include one.

.. code:: json

  {
    "name": "shortcut name",
    "description": "shortcut description",
    "location": "the location to jump to (~ supported)",
    "calls": ["callname", "anothercall", "maybeevenanothercall"]
  }

Once you have added a shortcut, you can use the nav command to navigate to that shortcut by one of its calls.
For example, if you want to go to the shortcut in the example above, we could use one of the following commands.

.. code:: bash

  # uses the first call
  $ nav callname

  # uses the second call
  $ nav anothercall

  # uses the third call
  $ nav maybeevenanothercall

You can also check out the `example configuration <https://github.com/MrDogeBro/quicknav/blob/master/example-configuration.json>`_
for help with setting up your config file.

Editing Shortcuts
+++++++++++++++++

Editing existing shortcuts is achieved using :code:`quicknav edit` which takes two positional arguments :code:`shortcut` and
optional :code:`location`, as well as two optional flag arguments :code:`name` and :code:`description`.

.. code:: bash

  # changing the description of a shortcut called projects
  $ quicknav edit projects --description "This is our new description"

  # changing the name of a shortcut called personal to business
  $ quicknav edit personal --name business

  # changing the location of a shortcut called documents
  $ quicknav edit documents ~/Documents

  # changing everything for a shortcut called aggregate
  $ quicknav edit aggregate /home/myuser/projects --name "root" --description "All my projects"

Adding and Removing Calls
+++++++++++++++++++++++++

Calls are what you actually enter to navigate to the desired shortcut. A single shortcut can have
multiple calls assigned to it. You can either directly edit the configuration file or use the add
and remove call commands to do the same job.

The built in configuration commands are listed below. To get more info about a command, use the help command
and specify which command you would like help for.

.. code:: bash

  # add a new call to a shortcut
  $ quicknav add-call <shortcut> <call>

  # remove a call from a shortcut
  $ quicknav remove-call <call>

Options
-------

These are the options that quicknav accepts in its config file, not including shortcuts, under the options section.
These can be configured via the config command or by directly editing the config file. The config command is listed
below and you can get more info on it by using the help command.

.. code:: bash

  # view current config
  $ quicknav config

  # view current config for single option
  $ quicknav config <option>

  # change the value for a config option
  $ quicknav config <option> <value>

Options List
++++++++++++

Listed below is the info for all of the options quicknav currently supports.

+----------------------------+-----------------------------+---------------+-----------------------------------------------------------------------------------------------------------------------------+
| Config Name                | Allowed Value               | Default Value | Description                                                                                                                 |
+============================+=============================+===============+=============================================================================================================================+
| create_missing_directories | Boolean (`true` or `false`) | `false`       | Automatically create directories in a given path if they do not exist when the navigation shortcut to that location is used |
+----------------------------+-----------------------------+---------------+-----------------------------------------------------------------------------------------------------------------------------+

Init Flags
----------

These are flags that you can add to the init command that is used to load your shell profile.
For more info on loading your shell profile, check out :ref:`adding-quicknav-to-your-shell`.

- :code:`-c, --command`: Changes the command which is used to navigate to a shortcut. The default command is :code:`nav`.
