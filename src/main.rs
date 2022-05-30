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
use lazy_static::lazy_static;
use quicknav::Quicknav;
use regex::Regex;
use structopt::clap::ErrorKind;
use structopt::StructOpt;

fn main() {
    match run() {
        Ok(res) => std::process::exit(res),
        Err(e) => {
            let mut err_msg: String = e.to_string();

            lazy_static! {
                static ref SPLIT_PKG_VER: Vec<&'static str> =
                    env!("CARGO_PKG_VERSION").split('.').collect();
                static ref HELP_REGEX: Regex = Regex::new(
                    format!(
                        "^quicknav(?:(-[a-zA-Z]+(?:-[a-zA-Z]+)?))? {}",
                        SPLIT_PKG_VER.join("\\.")
                    )
                    .as_str()
                )
                .unwrap();
            }

            if HELP_REGEX.is_match(&err_msg) {
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
            Quicknav::Get { location, search } => commands::get(location, search),
            Quicknav::List { shortcut, quiet } => commands::list(shortcut, quiet),
            Quicknav::Add {
                call,
                location,
                name,
                description,
            } => commands::add(call, location, name, description),
            Quicknav::Edit {
                shortcut,
                location,
                name,
                description,
            } => commands::edit(shortcut, location, name, description),
            Quicknav::AddCall { shortcut, call } => commands::add_call(shortcut, call),
            Quicknav::Remove { shortcut } => commands::remove(shortcut),
            Quicknav::RemoveCall { call } => commands::remove_call(call),
            Quicknav::Config { option, new_value } => commands::config(option, new_value),
            Quicknav::Init { shell, command } => commands::init(shell, command),
        },
        Err(e) => {
            if e.kind == ErrorKind::VersionDisplayed {
                println!();
                return Ok(0);
            }

            Err(anyhow!(e))
        }
    }
}
