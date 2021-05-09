extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;

mod commands;
mod config;
mod quicknav;

use anyhow::Result;
use quicknav::Quicknav;
use structopt::StructOpt;

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
