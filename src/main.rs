extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate prettytable;

mod commands;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Quicknav {
    /// Gets the location of a provided shortcut
    Get {
        /// The location to find, known as a call in the
        /// config file
        location: String
    },
    /// Lists the registered shortcuts
    List {
        /// The shortcut to search for
        shortcut: Option<String>
    },
    /// Initalizes the shell profile
    Init {
        /// The shell profile to use
        shell: String
    },
}

fn main() {
    match Quicknav::from_args() {
        Quicknav::Get { location } => {
            commands::get(location);
        },
        Quicknav::List { shortcut } => {
            commands::list(shortcut);
        },
        Quicknav::Init { shell } => {
            commands::init(shell);
        }
    }
}
