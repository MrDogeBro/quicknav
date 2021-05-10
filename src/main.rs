extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;

mod commands;
mod config;
mod quicknav;
mod utils;

use anyhow::{anyhow, Result};
use colored::*;
use quicknav::Quicknav;
use structopt::StructOpt;

fn main() {
    match run() {
        Ok(res) => std::process::exit(res),
        Err(e) => {
            println!("{}: {}", "error".red(), e);
            std::process::exit(1)
        }
    }
}

fn run() -> Result<i32> {
    match Quicknav::from_args_safe() {
        Ok(cmd) => match cmd {
            Quicknav::Get { location } => return commands::get(location),
            Quicknav::List { shortcut } => return commands::list(shortcut),
            Quicknav::Add {
                shortcut,
                location,
                name,
                description,
            } => return commands::add(shortcut, location, name, description),
            Quicknav::Remove { shortcut } => return commands::remove(shortcut),
            Quicknav::Config { option, new_value } => return commands::config(option, new_value),
            Quicknav::Init { shell, command } => return commands::init(shell, command),
        },
        Err(e) => return Err(anyhow!(e)),
    }
}
