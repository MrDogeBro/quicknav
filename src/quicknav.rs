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
    #[structopt(alias = "ls")]
    List {
        /// The shortcut to search for (by name)
        shortcut: Option<String>,
    },
    /// Adds a new shortcut
    #[structopt(alias = "new")]
    Add {
        /// The shortcut itself (call)
        call: String,
        /// The shortcut location
        location: String,
        /// The shortcut name
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
        /// The shortcut description
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
    /// Adds a new call for an existing
    /// shortcut
    #[structopt(alias = "new-call")]
    AddCall {
        /// The shortcut you are trying to
        /// add a call to (by name)
        shortcut: String,
        /// The call you want to be added
        call: String,
    },
    /// Edits an existing shortcut
    Edit {
        /// The shortcut to edit (by name)
        shortcut: String,
        /// The new shortcut location
        location: Option<String>,
        /// The new shortcut name
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
        /// The new shortcut description
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
    /// Removes a shortcut
    #[structopt(aliases = &["rm", "del", "delete"])]
    Remove {
        /// The shortcut to remove (by name)
        shortcut: String,
    },
    /// Removes a call for an existing
    /// shortcut without removing the shortcut
    #[structopt(aliases = &["rm-call", "del-call", "delete-call"])]
    RemoveCall {
        /// The call you are trying to remove
        call: String,
    },
    /// Allows for command line configuration of
    /// options
    #[structopt(aliases = &["conf", "cfg", "cf"])]
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
