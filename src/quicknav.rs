use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Quicknav {
    /// Gets the location of a provided shortcut
    Get {
        /// The location to find, known as a call in the
        /// config file
        location: String,
        /// If it should search for possible shortcuts
        #[structopt(short = "s", long = "search")]
        search: bool,
    },
    /// Lists the registered shortcuts
    List {
        /// The shortcut to search for
        shortcut: Option<String>,
    },
    /// Adds a new shortcut
    Add {
        /// The shortcut itself (call)
        shortcut: String,
        /// The shortcut location
        location: String,
        /// The shortcut name
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
        /// The shortcut description
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
    /// Edits an existing shortcut
    Edit {
        /// The shortcut itself (call)
        shortcut: String,
        /// The shortcut location
        location: Option<String>,
        /// The shortcut name
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
        /// The shortcut description
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
    /// Removes a shortcut
    Remove {
        /// The shortcut to remove (by call)
        shortcut: String,
    },
    /// Allows for command line configuration of
    /// options
    Config {
        /// The option to change
        option: Option<String>,
        /// The value to change the option to
        new_value: Option<String>,
    },
    /// Initalizes the shell profile
    Init {
        /// The shell profile to use
        shell: String,
        /// Optional way to change the invoke command
        #[structopt(short = "c", long = "command")]
        command: Option<String>,
    },
}
