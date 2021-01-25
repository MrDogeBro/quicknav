extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

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
        Quicknav::Init { shell } => {
            commands::init(shell);
        }
    }
}
