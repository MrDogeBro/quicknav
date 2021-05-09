extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;

mod commands;
mod config;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Quicknav {
    /// Gets the location of a provided shortcut
    Get {
        /// The location to find, known as a call in the
        /// config file
        location: String,
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
    /// Removes a shortcut
    Remove {
        /// The shortcut to remove (by call)
        shortcut: String,
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

fn main() -> Result<()> {
    match run() {
        Ok(res) => std::process::exit(res),
        Err(e) => return Err(e),
    }
}

fn run() -> Result<i32> {
    match Quicknav::from_args() {
        Quicknav::Get { location } => return commands::get(location),
        Quicknav::List { shortcut } => return commands::list(shortcut),
        Quicknav::Add {
            shortcut,
            location,
            name,
            description,
        } => return commands::add(shortcut, location, name, description),
        Quicknav::Remove { shortcut } => return commands::remove(shortcut),
        Quicknav::Init { shell, command } => return commands::init(shell, command),
    }
}
