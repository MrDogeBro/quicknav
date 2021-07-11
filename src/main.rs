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
use structopt::clap::ErrorKind;
use structopt::StructOpt;

fn main() {
    match run() {
        Ok(res) => std::process::exit(res),
        Err(e) => {
            let mut err_msg: String = e.to_string();

            if err_msg.starts_with(format!("quicknav {}", env!("CARGO_PKG_VERSION")).as_str()) {
                println!("{}", err_msg);
                std::process::exit(0);
            }

            if err_msg.starts_with("\u{1b}[1;31merror:\u{1b}[0m ") {
                err_msg = err_msg.replace("\u{1b}[1;31merror:\u{1b}[0m ", "");
            }

            println!("{} {}", "Error:".red(), err_msg);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<i32> {
    match Quicknav::from_args_safe() {
        Ok(cmd) => match cmd {
            Quicknav::Get { location, search } => return commands::get(location, search),
            Quicknav::List { shortcut } => return commands::list(shortcut),
            Quicknav::Add {
                shortcut,
                location,
                name,
                description,
            } => return commands::add(shortcut, location, name, description),
            Quicknav::Edit {
                shortcut,
                location,
                name,
                description,
            } => return commands::edit(shortcut, location, name, description),
            Quicknav::Remove { shortcut } => return commands::remove(shortcut),
            Quicknav::Config { option, new_value } => return commands::config(option, new_value),
            Quicknav::Init { shell, command } => return commands::init(shell, command),
        },
        Err(e) => {
            if e.kind == ErrorKind::VersionDisplayed {
                println!("");
                return Ok(0);
            }

            return Err(anyhow!(e));
        }
    }
}
