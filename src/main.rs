extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate prettytable;

mod commands;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Quicknav {
    /// Gets the location to one of shortcuts if it exists
    /// in the config file.
    Get {
        /// The location to find, known as a call in the
        /// config file
        location: String
    },
    /// Lists the registered shortcuts
    List,
    /// Initalizes the commands for the shell
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
        Quicknav::List {} => {
            commands::list();
        },
        Quicknav::Init { shell } => {
            commands::init(shell);
        }
    }
}
